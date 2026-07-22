# Proposal

The production Mirror Beat path can report one tracked player and show only a P1
HUD while the Macroquad stage still draws both P1 and an unevaluated robot P2.
At 390x844 the false P2 overlaps the camera-backed P1 avatar, making one-person
play look like two bodies occupying the same lane.

Give the stage one explicit player-presence policy. Single-player modes retain
P1 as the setup anchor and reveal additional slots only when their evaluation is
true. Duo Groove retains both required local slots before evaluation so its
two-person requirement stays visible. Evaluated remote slots remain visible.

## Non-goals

- Changing tracking identity, pose thresholds, evaluation, scoring, collision,
  multiplayer transport, camera layout, HUD placement, Rust/JS ABI, or assets.
- Redesigning player avatars or adding another illustration, legend, touch
  action, tutorial, or persistent preference.
- Claiming a synthetic still proves real two-person crossing or identity
  stability.
