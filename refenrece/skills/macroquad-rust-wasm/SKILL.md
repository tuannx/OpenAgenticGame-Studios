---
name: macroquad-rust-wasm
version: 1.0.0
description: Build, review, test, optimize, and ship Macroquad games in Rust for native and browser WebAssembly targets. Use for Macroquad project setup, game-loop architecture, rendering/input/audio integration, responsive canvas UX, WASM packaging, static hosting, or Rust game performance work.
argument-hint: "[new|review|build|deploy] [native|web|both]"
user-invocable: true
allowed-tools: Read, Glob, Grep, Write, Edit, Bash, WebSearch, WebFetch
---

# Macroquad Rust/WASM

Build lightweight, cross-platform games with Rust and Macroquad while keeping
game rules testable outside the rendering loop and treating browser behavior as
a first-class runtime target.

## Load First

Read these files before changing a Macroquad project:

1. `Cargo.toml` and `rust-toolchain.toml` when present.
2. `src/main.rs`, game-state modules, and platform adapters.
3. Existing `assets/`, `web/`, deployment, and CI configuration.
4. `docs/engine-reference/macroquad/VERSION.md`.
5. `references/web-delivery.md` for browser builds or deployment.

Verify the current Macroquad release from official sources before creating or
upgrading a dependency pin. Do not infer the latest version from this skill.

## Route The Task

- `new`: scaffold a Cargo project and a thin Macroquad adapter.
- `review`: inspect architecture, frame-loop behavior, assets, UX, and WASM risks.
- `build`: run native and/or WASM validation without deploying.
- `deploy`: produce a static web bundle and verify it through an HTTP server.

When no route is provided, infer it from the request and repository state.
Confirm native, web, or both when the target changes architecture or delivery.

## Architecture Contract

Prefer a clean, dependency-directed layout:

```text
src/
├── main.rs                 # composition root and Macroquad frame loop
├── game/                   # deterministic rules and state transitions
│   ├── mod.rs
│   └── state.rs
├── ports/                  # narrow input, rendering, audio, persistence contracts
└── platform/
    └── macroquad/          # Macroquad adapters
assets/                     # runtime assets, paths preserved for web hosting
web/                        # index.html and locally pinned loader
tests/                      # domain and integration tests
```

Keep physics, scoring, progression, and state transitions independent from
draw calls when practical. Pass plain input snapshots into game logic and render
from immutable state. Avoid global mutable state and deep inheritance.

Use one explicit composition root. Introduce traits only at real volatility or
test seams; do not add abstraction layers around stable one-line APIs.

## Frame Loop

Use Macroquad's async frame boundary and delta time:

```rust
use macroquad::prelude::*;

#[macroquad::main("Game")]
async fn main() {
    let mut game = Game::new();

    loop {
        let dt = get_frame_time().min(0.05);
        let input = InputFrame::capture();
        game.update(input, dt);

        clear_background(BLACK);
        game.draw();
        next_frame().await;
    }
}
```

- Clamp exceptional frame gaps before physics integration.
- Use a fixed timestep accumulator when deterministic physics requires it.
- Keep I/O asynchronous and never block the WASM frame loop.
- Batch draw work, reuse allocations, and measure before optimizing.

## Responsive UX And Input

- Derive layout from `screen_width()` and `screen_height()` on every frame or
  when the viewport changes.
- Convert pointer coordinates into world/UI coordinates explicitly.
- Support mouse, touch, and keyboard only when the product requires them; keep
  action mapping above raw device events.
- Make touch targets readable and reachable on small screens.
- Handle focus loss, resize, orientation change, and suspended audio gracefully.
- Gate browser audio startup behind a real user gesture when required.
- Preserve aspect ratio intentionally; choose letterboxing, crop, or adaptive
  layout rather than stretching by accident.

## Assets And Browser Boundary

- Load assets through Macroquad's async APIs before entering dependent gameplay.
- Macroquad 0.4.15 enables only PNG/TGA decoders by default. Loading a JPEG with
  `load_texture()` can reach an internal panic because format detection succeeds
  while JPEG support is absent. If canonical web art is JPEG, explicitly enable
  `image`'s `jpeg` feature and decode through a fallible boundary before creating
  `Texture2D`; cover this with a real-asset decode test and an HTTP browser load.
- Keep asset paths relative and preserve their layout in the deployed bundle.
- Serve `.html`, `.wasm`, JavaScript, and assets over HTTP; do not validate by
  opening `file://` directly.
- Use Macroquad/miniquad's loader for the standard web path. Do not introduce
  `wasm-bindgen` unless another dependency or explicit JavaScript API requires it.
- Prefer a locally pinned loader in production so upstream changes cannot alter
  a deployed game unexpectedly.

## Validation

Run the reusable validation script from the target Cargo project:

```bash
refenrece/skills/macroquad-rust-wasm/scripts/validate.sh /path/to/game
```

The script requires the WASM target to be installed and runs:

- `cargo fmt --all -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test --all-targets --all-features`
- a release `wasm32-unknown-unknown` build with the Rust 1.96+ linker
  compatibility flag described in `references/web-delivery.md`

For browser-facing changes, also serve the release bundle locally and test:

- initial load without console errors
- keyboard, mouse, and touch behavior as applicable
- viewport resize and mobile orientation
- asset/audio loading
- tab blur/focus and pause behavior
- stable frame pacing in a release build

## Closeout

Report native and web targets separately. Name every command run, browser path
verified, unverified platform, bundle-size observation, and remaining UX or
performance risk.

## Guardrails

- Do not block inside the async main loop.
- Do not couple domain tests to a graphics context unnecessarily.
- Do not claim browser support from a successful Rust compile alone.
- Do not deploy debug WASM artifacts.
- Do not depend on a remote loader without making that delivery choice explicit.
- Do not guess current crate versions or browser support details.
