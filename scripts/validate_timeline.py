#!/usr/bin/env python3
import argparse
import sys
from pathlib import Path
import yaml

def mmss_to_sec(s: str) -> float:
    s = s.strip().replace("–", "-")
    left, right = s.split("-")
    def parse(mmss):
        m, sec = mmss.strip().split(":")
        return int(m) * 60 + float(sec)
    return parse(left), parse(right)

def validate_file(path: Path) -> int:
    with open(path, "r", encoding="utf-8") as f:
        data = yaml.safe_load(f)
    frames = data.get("frames", [])
    ok = True
    last_end = -1.0
    for i, fr in enumerate(frames, start=1):
        idx = fr.get("index")
        if idx != i:
            print(f"[ERR] {path}: frame order mismatch at position {i}, index={idx}", file=sys.stderr)
            ok = False
        t = fr.get("time", "")
        try:
            a, b = mmss_to_sec(t)
            if b <= a:
                print(f"[ERR] {path}: non-positive duration at frame {i} time '{t}'", file=sys.stderr)
                ok = False
            if a < last_end:
                print(f"[WARN] {path}: overlapping frames around {i} ({a:.2f} < {last_end:.2f})", file=sys.stderr)
            last_end = b
        except Exception as e:
            print(f"[ERR] {path}: cannot parse time '{t}': {e}", file=sys.stderr)
            ok = False
    if ok:
        print(f"[OK] {path} — {len(frames)} frames, end at {last_end:.2f}s")
        return 0
    return 1

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("files", nargs="+", help="YAML timeline files")
    args = ap.parse_args()
    rc = 0
    for f in args.files:
        rc |= validate_file(Path(f))
    sys.exit(rc)

if __name__ == "__main__":
    main()