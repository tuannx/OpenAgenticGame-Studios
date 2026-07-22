# Proposal

## Problem

The current runner pauses safely when tracking disappears, but it has no
intentional no-touch pause action. A player must leave the camera frame or walk
back to the device, violating the zero-touch contract and conflating a chosen
break with a sensor failure.

## Scope

- Introduce a deliberate, one-second two-hand hold that does not overlap the
  clap recognizer.
- Freeze and clear the runner in an explicit intentional pause state.
- Resume with clap readiness and the established deterministic countdown.
- Give the frozen screen one shape-first pause symbol and one instruction.

## Non-goals

- No hidden DOM shortcut, touch pause button, camera auto-start, new image asset,
  scoring/balance change, or P2P transport schema change.
- No production deployment.

## Success criteria

Pause and resume are fully operable at camera distance, remain camera-evaluated,
cannot alter score, and have deterministic one- and two-player tests plus native,
WASM, web, and HTTP browser evidence.
