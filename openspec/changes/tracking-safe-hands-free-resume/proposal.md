# Proposal

## Problem

During `Running`, losing camera evaluation suppresses scoring but does not stop
beat, distance, or obstacle progression. A returning player can therefore be
judged or hit immediately after stepping back into frame. A hidden browser tab
can also retain the last pose until vision produces another callback.

This is technically camera-backed evaluation, but it is not a safe or friendly
AR interaction: gameplay changes while the player cannot see or control it.

## Scope

- Freeze deterministic runner progression when the required tracked players
  disappear during an active run.
- Clear in-flight hazards on hold so resume cannot cause an immediate collision.
- Require an evaluated movement or clap to ready, then show a short deterministic
  countdown before running again.
- Require both local players to return and signal in Duo mode.
- Invalidate stale browser poses whenever the page becomes inactive.
- Present the hold and countdown as large, low-copy Macroquad overlays.

## Non-goals

- No new pause gesture or camera recognizer.
- No scoring, obstacle difficulty, or pose-threshold tuning.
- No camera permission auto-start, background camera execution, or raw-pose storage.
- No deployment, TURN, or P2P protocol changes.

## Acceptance criteria

1. Losing required camera evaluation during `Running` enters a typed hold state
   before distance, beat, score, life, combo, or obstacle judgment advances.
2. Entering hold removes in-flight obstacles and unevaluated inputs cannot ready
   or resume the run.
3. Evaluated movement/clap starts a two-second countdown; progression stays
   frozen until it completes and tracking loss restarts the hold.
4. Duo requires P1 and P2 to be evaluated and both to signal readiness.
5. A hidden page clears its pose snapshot and reports evaluation disabled until
   a fresh visible-page pose arrives.
6. Native Rust, release WASM, web tests/build, and an HTTP browser focus/resize
   smoke pass without weakening camera/audio gesture gates.
