# Proposal

## Problem

The first BrainBreak slice proves camera-to-action-to-game flow, but consumers
still have to coordinate recognizers, players, scoring, and presentation state
inside the Macroquad composition root. Motion is visible only in a small camera
preview, so the interaction does not yet feel like the player's body inhabits
the game world.

## Outcome

- Expose a configurable, renderer-independent `MotionRuntime` API.
- Produce a stable per-frame snapshot with pose, quality, active actions,
  triggers, and hit/miss feedback.
- Render tracked bodies and target zones directly inside an interactive game.
- Keep raw landmarks local and preserve the existing P2P action protocol.

## Non-goals

- General-purpose 3D physics or arbitrary ML model plugins.
- Remote video or remote pose-landmark streaming.
- Replacing the existing visual content-pack format in this milestone.
