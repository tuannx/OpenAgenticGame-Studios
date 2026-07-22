# Validation

## Automated evidence

- `cargo fmt --all -- --check` — passed in the full gate.
- `cargo clippy --all-targets --all-features -- -D warnings` — passed.
- `cargo test --all-targets --all-features` — 22 deterministic core tests and
  2 Macroquad layout tests passed.
- Core coverage includes evaluated-only result input, opposing-input cancel,
  cyclic mode selection, one-shot requests, best preservation, new-best state,
  direct replay, and Duo two-player readiness.
- Layout coverage checks 390×844, 667×375, and 1440×784 for a three-chip result
  panel and four-player HUD bounds.
- `npm run typecheck` — strict TypeScript passed.
- `npm test` — 16 tests passed across 5 files, including v5 read/write bridge,
  callback, integer normalization, and bounds.
- `bash refenrece/skills/brainbreak-motion-games/scripts/validate.sh brainbreak`
  — full native/release-WASM/web/audit gate passed with no Rust/WASM warning.
- Final fingerprinted artifact: `brainbreak-game-931557e7b727.wasm`.

## HTTP and browser smoke

- Preview HTML returned `text/html`.
- Mode art returned `image/jpeg`.
- Final WASM returned `application/wasm` and loaded with the new v5 import.
- A fresh Chrome tab and a clean reload both booted with no app-origin warning
  or error; `#boot-screen` exited.
- Desktop 1440×784 and phone 390×844 matched canvas/client viewport dimensions
  and had no body overflow.
- An old tab that had lived through several bundle replacements briefly logged
  a deleted-texture error. It did not reproduce on either the final clean tab or
  its reload, so it is recorded as stale-preview behavior rather than evidence
  against the final artifact.

## Sequential persona review

### Security and privacy

- Result input is filtered by `player.evaluated`; unevaluated and guide-only
  input cannot navigate or start a scored run.
- The new browser import accepts only normalized mode values 0/1/2.
- Runtime callbacks update mode/taste aggregates only; no frame, pose, keypoint,
  image, video, or new network path was introduced.

### Performance

- Mode navigation and layout are fixed-size operations.
- Result-only formatting is off the active running path; HUD work remains O(4)
  with no unbounded storage or async work.
- Vision remains dynamically imported and unchanged by M3.

### Readability and architecture

- Core owns the state transition and exposes a one-shot request; the Macroquad
  composition root owns bridge effects.
- Layout functions are pure and tested independently of the graphics context.
- HUD/result rendering remains colocated with the existing Macroquad overlay
  path; a separate presentation module is unnecessary at the current size.

### Runtime regression

- Replay preserves the selected non-default mode instead of reconstructing
  Mirror first.
- Switching to Duo cannot bypass two-player readiness.
- Cue and beat meter render only while running; player cards hide behind Ready
  and Result overlays, removing duplicate/overlapping status.
- Bridge plugin and Rust crate versions are both 5, and HTTP-served WASM booted.

## Residual attended QA gap

Camera permission was not accepted in automation. The final rendered HUD/result,
physical lean selection, clap confirmation, camera-distance readability, mode
switch under a tracked body, and two-person Duo transition still require an
attended real-camera session. The layout and state-machine tests prove bounds
and semantics, not physical gesture feel or final pixels.
