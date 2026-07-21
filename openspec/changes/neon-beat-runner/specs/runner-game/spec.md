# Runner Game Spec

## Requirements

### R1. Camera-Backed Motion Controls

Acceptance:

- MoveLeft and MoveRight change one lane per rising action without leaving three-lane bounds.
- Jump, Squat, and Clap create time-bounded states visible in both rules and rendering.
- Players without evaluation permission do not score, lose lives, or send evaluated actions.

### R2. Readable Endless-Runner Challenge

Acceptance:

- Obstacles approach through deterministic patterns and communicate the required action through shape plus text/icon, not color alone.
- A collision is resolved once per obstacle per player.
- Game over exposes an immediate clap/restart action and a near-miss summary.

### R3. Audio-Reactive Presentation

Acceptance:

- Music begins only after the camera/user gesture and may be muted without stopping gameplay.
- Scene light and pulse metrics follow the 126 BPM track clock with analyzed audio energy layered on top.
- Reduced-motion mode removes camera shake and large spatial pulses while preserving readable color and shape cues.

### R4. Attribution And Redistribution

Acceptance:

- The game and repository identify “Special Spotlight,” Kevin MacLeod, its source, CC BY 4.0 license, and the web-compression modification.
- Audio is hosted as a versioned local asset rather than a runtime third-party dependency.

## Non-Requirements

- Exact visual or mechanical reproduction of Temple Run.
- Full 3D art, accounts, purchases, or persistent meta-progression.

## Validation Notes

- Use the Macroquad Rust/WASM package plus web typecheck/test/build.
- Closeout requires an HTTP browser smoke and production media/MIME verification.
