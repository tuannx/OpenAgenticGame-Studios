# Design

## One stage presence policy

Derive a fixed four-slot visibility mask from `GameMode` plus each
`RunnerPlayer.evaluated` flag:

- P1 remains visible as the primary setup/player anchor in every mode.
- P2 is reserved visibly before evaluation only in Duo Groove.
- Any additional P2/P3/P4 slot becomes visible when its authoritative runner
  evaluation is true.

The stage consumes this pure mask before `draw_runner`; avatar drawing itself,
pose projection, lane physics, feedback, and HUD rendering remain unchanged.

## Invariants

- Mirror Beat and Beat Strike never imply an unevaluated second player.
- Duo Groove preserves two visible required positions while waiting.
- An evaluated local or remote slot is never hidden by presentation policy.
- Presence is derived from deterministic runner state, not DOM copy or camera
  preview text.
- No hot-loop allocation, browser import, ABI bump, asset, or touch step is
  introduced.
