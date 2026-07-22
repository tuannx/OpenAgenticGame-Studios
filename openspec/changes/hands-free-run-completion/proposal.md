# Proposal

## Problem

The current gameplay HUD uses 11px secondary text that is not readable at
camera distance. Game over is a flat score message with clap-to-restart only,
so changing game requires returning to a touch-oriented launcher. Restart also
reconstructs the default Mirror mode before the bridge corrects it, creating a
state-flow risk for Strike and Duo.

## Scope

- Make the in-run HUD readable and minimal: mode, score, life, and combo only.
- Add positive run-completion framing with score, best score, and best combo.
- Let evaluated players lean left/right to select Mirror, Strike, or Duo and
  clap to start the selected next run.
- Keep result navigation deterministic in `brainbreak-core`.
- Synchronize a result-selected mode through the browser bridge and update the
  local taste profile at the completed-run boundary.
- Preserve guide-only, camera evaluation, Duo readiness, reduced motion, and
  keyboard fallback contracts.

## Non-goals

- No scoring/difficulty threshold changes.
- No new camera gesture recognizer.
- No new external art asset or mismatched result illustration.
- No deployment, TURN, or two-peer work.

## Acceptance criteria

1. Result navigation ignores input from unevaluated players.
2. Left/right cycles through all three modes and clap emits one deterministic
   next-run request.
3. Replaying the current mode and switching modes both preserve personal bests;
   Duo does not auto-start without both evaluated players.
4. Result CTA is visible immediately, positively framed, and fully no-touch.
5. In-run HUD text remains readable at phone and camera-distance layouts without
   covering the primary action cue.
6. JavaScript/Rust bridge versions remain synchronized and release WASM loads.

