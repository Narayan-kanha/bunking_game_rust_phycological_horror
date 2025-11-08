# Freshman Roll — Prototype

This is a minimal, data-driven prototype to play storyboard timelines for endings/paths using Bevy.

- Engine: Rust + Bevy 0.14
- Narrative assets: YAML under `assets/narrative/`
- Visualization: logs + background color changes per frame; no fonts/assets required

## Run

Prereqs:
- Rust stable (1.75+ recommended)
- Cargo

Run:
```bash
cargo run
```

Controls:
- 1 — Play Path 1: THE TRUE WAKE
- 6 — Play Path 6: THE SUNK LEGEND
- Esc — Return to menu (auto when timeline finishes)

## Add a new narrative path

1. Create a YAML in `assets/narrative/` following the schema:
```yaml
title: "Your Title"
frames:
  - index: 1
    time: "00:00–00:05"
    camera: "Description"
    lighting: "Description"
    notes: "VO/SFX notes"
  # ...
```

2. Add a static include in `src/main.rs` and an OnEnter loader similar to `path1_load`.

3. Optionally validate:
```bash
python3 scripts/validate_timeline.py assets/narrative/pathX.yaml
```

## Next steps

- Replace console logs with on-screen UI (font asset + Bevy UI text)
- Camera choreography (2D/3D), post-process FX, and audio events
- Narrative graph for choices and act gating
- World systems (exploration, NPCs, missions) and state transitions into timelines
