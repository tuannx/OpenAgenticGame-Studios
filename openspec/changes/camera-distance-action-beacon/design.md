# Design

## Ownership

`RunnerGame::next_cue()` remains the deterministic source of the nearest
obstacle. `visuals.rs` maps that immutable value to presentation only. The DOM,
vision bridge, recognizers, scoring, collision, and network actions are
unchanged; no JavaScript/WASM import changes are required and bridge ABI remains
v5.

## Semantic model

The main word is an action intent (`DODGE`, `JUMP`, `SQUAT`, `CLAP`). A distinct
line-drawn silhouette makes the word redundant. The lane is deliberately shown
as a three-slot diagram rather than an arrow: the highlighted slot is where the
obstacle exists, not necessarily a command to move toward it. A bottom rail
fills from horizon to collision using a clamped monotonic proximity value.

## Milestone M9

Goal:

- Make the next physical action readable at camera distance without changing
  deterministic evaluation.

Execution slices:

- Add pure presentation mapping, responsive layout, and proximity helpers.
- Render the panel, silhouette, lane diagram, and approach rail.
- Add focused Rust tests and a durable taste rule.

Out of scope:

- Gameplay rules, browser bridge changes, new assets, and simulated evaluation.

Touched systems:

- Macroquad rendering, Rust layout tests, BrainBreak taste reference.

Entry context:

- BrainBreak architecture contract and taste guide.
- Macroquad Rust/WASM validation reference.
- Mobile game camera-distance readability guidance.

Acceptance criteria:

- Four distinct action mappings; shape-plus-text lane and timing semantics.
- Responsive containment across 390x844, 667x375, and 1440x784.
- No per-frame string formatting or unbounded render-loop storage.
- Release browser artifact loads without an app-origin console regression.

Validation package:

- Focused Rust tests, full native Rust gates, release WASM validation, web tests
  and build, plus an HTTP-served browser smoke check.

Persona review:

- Security, performance, readability, and runtime regression lenses.

Fallback note:

- If the expanded cue harms small-landscape play, retain the pure mapping and
  proximity helpers but fall back to a compact horizontal panel for that layout.

Next dependency:

- A real camera-distance playtest can tune silhouette scale and telegraph lead
  time without reopening evaluation ownership.
