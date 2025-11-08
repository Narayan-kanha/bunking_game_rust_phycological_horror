#!/usr/bin/env python3
"""
normalize_map_colors.py

Reads an input top-down map image, maps every pixel to the nearest color
from a provided legend using perceptual LAB distances, applies optional denoising
and a majority (mode) smoothing pass to remove speckles, and writes a cleaned
map and a visual diff image.

Usage:
    python normalize_map_colors.py --input map.png --output map_clean.png --diff diff.png

Tweak parameters:
    --median N       apply median filter with kernel N (0 = disabled)
    --modeiters N    run N passes of modal smoothing (default 2)
    --threshold T    optional maximum deltaE threshold to map (unused by default)
"""

from PIL import Image, ImageFilter, ImageOps
import numpy as np
import math
import argparse
from collections import Counter

# ========== CONFIG: your final palette (hex -> name) ==========
LEGEND = {
    "#00FF00": "grass",
    "#0000FF": "water",
    "#453503": "mud",
    "#000000": "road",
    "#808080": "building",
    "#006400": "tree",
    "#FF0000": "player_spawn",
    "#FFFF00": "npc_spawn",
    "#FF00FF": "car_spawn",
    "#FFA500": "shop",
    "#C0C0C0": "school",
    "#964B00": "fence",
    "#00FFFF": "teleport"
}

# ========== color conversion helpers (sRGB -> LAB) ==========
# Reference white D65
REF_X = 0.95047
REF_Y = 1.00000
REF_Z = 1.08883

def hex_to_rgb(hex_color):
    h = hex_color.lstrip('#')
    return tuple(int(h[i:i+2], 16) for i in (0, 2, 4))

def srgb_to_linear(c):
    c = c / 255.0
    if c <= 0.04045:
        return c / 12.92
    else:
        return ((c + 0.055) / 1.055) ** 2.4

def rgb_to_xyz(r, g, b):
    # convert 0-255 ints to linear RGB then XYZ (sRGB D65)
    r_l = srgb_to_linear(r)
    g_l = srgb_to_linear(g)
    b_l = srgb_to_linear(b)
    # sRGB to XYZ matrix
    x = r_l * 0.4124564 + g_l * 0.3575761 + b_l * 0.1804375
    y = r_l * 0.2126729 + g_l * 0.7151522 + b_l * 0.0721750
    z = r_l * 0.0193339 + g_l * 0.1191920 + b_l * 0.9503041
    return (x, y, z)

def f_xyz(t):
    delta = 6/29
    if t > (delta**3):
        return t ** (1/3)
    else:
        return (t / (3 * (delta**2))) + (4/29)

def xyz_to_lab(x, y, z):
    xr = x / REF_X
    yr = y / REF_Y
    zr = z / REF_Z
    fx = f_xyz(xr)
    fy = f_xyz(yr)
    fz = f_xyz(zr)
    l = 116 * fy - 16
    a = 500 * (fx - fy)
    b = 200 * (fy - fz)
    return (l, a, b)

def rgb_to_lab_tuple(rgb):
    x, y, z = rgb_to_xyz(*rgb)
    return xyz_to_lab(x, y, z)

def delta_e_lab(lab1, lab2):
    # simple Euclidean deltaE in LAB (good enough for our mapping)
    return math.sqrt((lab1[0]-lab2[0])**2 + (lab1[1]-lab2[1])**2 + (lab1[2]-lab2[2])**2)

# Precompute legend LAB palette
PALETTE_RGB = [hex_to_rgb(h) for h in LEGEND.keys()]
PALETTE_HEX = list(LEGEND.keys())
PALETTE_NAMES = [LEGEND[h] for h in PALETTE_HEX]
PALETTE_LAB = [rgb_to_lab_tuple(rgb) for rgb in PALETTE_RGB]

# ========== utilities ==========
def find_nearest_palette(rgb):
    lab = rgb_to_lab_tuple(rgb)
    best_idx = 0
    best_dist = float("inf")
    for i, plab in enumerate(PALETTE_LAB):
        d = delta_e_lab(lab, plab)
        if d < best_dist:
            best_dist = d
            best_idx = i
    return best_idx, best_dist

def majority_mode_of_block(index_grid, x, y, w, h, radius=1):
    # returns index (palette index) that is majority in neighborhood
    votes = []
    for yy in range(max(0, y-radius), min(h, y+radius+1)):
        for xx in range(max(0, x-radius), min(w, x+radius+1)):
            votes.append(index_grid[yy, xx])
    return Counter(votes).most_common(1)[0][0]

# ========== main processing ==========
def normalize_image(in_path, out_path, diff_path=None, median=3, modeiters=2, verbose=True):
    img = Image.open(in_path).convert("RGB")
    w, h = img.size
    if verbose:
        print(f"[+] Loaded {in_path} ({w}x{h})")

    if median and median >= 1:
        if verbose:
            print(f"[+] Applying median filter (kernel {median})...")
        img = img.filter(ImageFilter.MedianFilter(size=median))

    pixels = np.array(img)  # shape (h, w, 3), dtype=uint8
    h, w = pixels.shape[0], pixels.shape[1]

    # create arrays
    mapped_idx = np.empty((h, w), dtype=np.int32)
    mapped_rgb = np.empty((h, w, 3), dtype=np.uint8)
    distances = np.empty((h, w), dtype=np.float32)

    # Map all pixels to nearest palette entry
    if verbose:
        print("[+] Mapping pixels to nearest legend color (LAB space). This may take a bit...")
    total = h * w
    count = 0
    # vectorized-ish: for large images, loop by rows
    for y in range(h):
        row = pixels[y]
        for x in range(w):
            rgb = tuple(row[x])
            idx, dist = find_nearest_palette(rgb)
            mapped_idx[y, x] = idx
            mapped_rgb[y, x] = PALETTE_RGB[idx]
            distances[y, x] = dist
        count += w
        if verbose and (y % 128 == 0):
            print(f"    processed row {y}/{h}")

    # majority smoothing passes (mode filter) to remove speckles
    if modeiters and modeiters > 0:
        if verbose:
            print(f"[+] Applying {modeiters} majority-mode smoothing passes...")
        for it in range(modeiters):
            new_idx = mapped_idx.copy()
            for y in range(h):
                for x in range(w):
                    new_idx[y, x] = majority_mode_of_block(mapped_idx, x, y, w, h, radius=1)
            mapped_idx = new_idx
            for i, rgb in enumerate(PALETTE_RGB):
                mapped_rgb[mapped_idx == i] = rgb
            if verbose:
                print(f"    mode pass {it+1}/{modeiters} done")

    # Build output image
    out_img = Image.fromarray(mapped_rgb, mode="RGB")
    out_img.save(out_path)
    if verbose:
        print(f"[+] Saved cleaned image -> {out_path}")

    # Save diff image highlighting changed pixels (red overlay)
    if diff_path:
        if verbose:
            print("[+] Creating diff visualization...")
        orig = np.array(Image.open(in_path).convert("RGB"))
        changed = np.any(orig != mapped_rgb, axis=2)
        # create visual: keep original but mark changed pixels red or outline them
        vis = orig.copy()
        # mark changed pixels with red tint overlay
        vis[changed] = (255, 0, 0)
        Image.fromarray(vis).save(diff_path)
        if verbose:
            changed_pct = changed.sum() * 100.0 / (w*h)
            print(f"[+] Diff saved -> {diff_path} (changed {changed.sum()} pixels, {changed_pct:.2f}%)")

    # print counts per legend color
    counts = Counter(mapped_idx.flatten().tolist())
    if verbose:
        print("[+] Legend counts (hex, name, count):")
        for i, hexc in enumerate(PALETTE_HEX):
            print(f"    {hexc}  {PALETTE_NAMES[i]:12s}  {counts.get(i,0)}")

    # also print some statistics: mean distance, max
    if verbose:
        print(f"[+] distance stats: mean {distances.mean():.2f}, max {distances.max():.2f}")

    return out_path

# ========== CLI ==========
def parse_args():
    p = argparse.ArgumentParser(description="Normalize a map image to a fixed color legend.")
    p.add_argument("--input", "-i", required=True, help="Input image (e.g. map.png)")
    p.add_argument("--output", "-o", default="map_clean.png", help="Output cleaned image")
    p.add_argument("--diff", "-d", default="map_diff.png", help="Diff visualization output (optional)")
    p.add_argument("--median", type=int, default=3, help="Median filter kernel size (0 to disable)")
    p.add_argument("--modeiters", type=int, default=2, help="How many majority smoothing passes to run")
    return p.parse_args()

if __name__ == "__main__":
    args = parse_args()
    normalize_image(args.input, args.output, diff_path=args.diff, median=args.median, modeiters=args.modeiters)
