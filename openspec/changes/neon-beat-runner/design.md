# Design: Neon Beat Runner

## Problem Statement

- A complete runner crosses deterministic gameplay, motion recognition, audio timing, rendering, responsive DOM controls, licensing, and browser delivery.
- Treating those concerns as one frame-loop blob would weaken the tagged framework and make gameplay tests depend on graphics or browser state.

## ADR 1: Keep Runner Rules In The Renderer-Independent Core

Context:

- `brainbreak-core` already owns recognized action semantics and evaluation gating.
- Collision, lane changes, lives, combo, spawning, and restart must be deterministic and testable without Macroquad.

Decision:

- Add an application-specific `RunnerGame` state machine to `brainbreak-core`; Macroquad consumes immutable snapshots and draws them.

Tradeoffs:

- The core crate gains one concrete sample-game domain, but retains no graphics, DOM, audio, or Cloudflare dependency.
- Runner inputs use the framework action masks, avoiding a second motion vocabulary.

Validation impact:

- Unit tests prove collision semantics, evaluation gating, lane bounds, and restart behavior.

Migration and follow-up implications:

- Future games may use the same pattern without forcing runner concepts into `MotionRuntime`.

## ADR 2: Browser Audio Owns Playback; WASM Reads Stable Metrics

Context:

- Browser autoplay policy requires a real user gesture, while Macroquad rendering needs beat phase and energy every frame.

Decision:

- A TypeScript `MusicEngine` owns the HTML audio element, Web Audio analyser, mute state, and 126 BPM clock. Narrow WASM imports expose beat phase, pulse, energy, and playback state.

Tradeoffs:

- Audio is browser-specific; native builds use a deterministic silent visual clock.
- Beat phase follows music playback while obstacle movement remains delta-time deterministic.

Validation impact:

- Vitest covers pure beat math; browser smoke verifies gesture startup and audio asset loading.

Migration and follow-up implications:

- A future native audio adapter can implement the same metric contract.

## ADR 3: Use A Clearly Licensed, Locally Hosted Track

Context:

- “No copyright” is ambiguous and many creator-platform tracks do not permit redistribution inside a game bundle.

Decision:

- Use “Special Spotlight” by Kevin MacLeod from Incompetech, 126 BPM, licensed CC BY 4.0. Host a web-compressed MP3 and provide title, author, source, license link, and modification notice.

Tradeoffs:

- Attribution must remain visible and the media adds approximately 3 MB to the static bundle.
- The clearer redistribution right is preferred over a more famous mainstream song with uncertain game-embedding rights.

Validation impact:

- Verify production audio MIME, size, cache behavior, and visible attribution.

Migration and follow-up implications:

- Track replacement requires equal or stronger license evidence and a new BPM/offset calibration.

## Milestone 1: Runner Domain

Goal:

- Ship deterministic, camera-action-driven runner rules.

Execution slices:

- Model three lanes, obstacles, pickups, per-player state, score/combo/lives, and game phases.
- Map MoveLeft, MoveRight, Jump, Squat, and Clap to runner intent.
- Add focused core tests.

Out of scope:

- Rendering and audio playback.

Touched systems:

- Rust core runtime and tests.

Entry context:

- `brainbreak-core`, motion framework change record, Macroquad skill.

Acceptance criteria:

- No player escapes lane bounds or scores while evaluation is disabled.
- Correct actions avoid hazards; wrong or absent actions cost one life exactly once.

Validation package:

- `cargo fmt`, `cargo clippy`, `cargo test`.

Persona review:

- Readability, runtime regression, and performance lenses.

Fallback note:

- Runner code is additive; the tagged Motion Reactor remains recoverable.

Next dependency:

- Milestone 2.

## Milestone 2: Beat-Synchronized Experience

Goal:

- Replace the abstract stage with a polished neon runner whose feedback follows the music.

Execution slices:

- Render perspective track, runners, pose skeletons, obstacles, pickups, skyline, glow, particles, and HUD.
- Add browser music engine, beat metrics bridge, attribution, mute, and reduced-motion control.

Out of scope:

- Artist-authored 3D assets and post-processing shaders.

Touched systems:

- Macroquad rendering, WASM bridge, TypeScript audio, HTML/CSS UI, static assets.

Entry context:

- Milestone 1 state model, Macroquad web-delivery reference, mobile-game-ux skill.

Acceptance criteria:

- Hazards and required actions are readable at runner speed.
- Critical audio cues have visual equivalents and reduced-motion disables shake/large pulses.

Validation package:

- Macroquad Rust/WASM package plus `npm run typecheck`, `npm test`, and `npm run build`.

Persona review:

- Security, performance, readability, and runtime regression lenses.

Fallback note:

- Music failure falls back to a deterministic beat clock and the game remains playable.

Next dependency:

- Milestone 3.

## Milestone 3: Browser And Production Closeout

Goal:

- Prove the release bundle in an HTTP browser path and Cloudflare production.

Execution slices:

- Run complete validation, served smoke tests, deploy, verify health and all content types.
- Update README, license attribution, task state, and validation evidence.

Out of scope:

- Physical-device camera performance certification.

Touched systems:

- Web release bundle, Cloudflare Worker, documentation.

Entry context:

- Milestones 1 and 2, validation and closeout workflows.

Acceptance criteria:

- Production HTML references retrievable JS/WASM/audio assets with correct MIME types.
- Change record matches the shipped behavior and states residual device risks.

Validation package:

- Full Macroquad Rust/WASM package, browser smoke, Wrangler deploy and production probes.

Persona review:

- All four lenses.

Fallback note:

- Roll back Cloudflare to the tagged base framework if the asset chain fails.

Next dependency:

- None.
