# Proposal

The running HUD is currently centered at the bottom of the full-screen stage.
The nearest runner is also anchored at the bottom center, so the opaque P1 card
covers the camera-backed avatar's hips, legs, foot identity, and lane feedback at
both 390x844 portrait and 1280x720 desktop.

Give Running one responsive surface layout: the stage ends above a dedicated HUD
deck, while non-running overlays retain the full canvas. The deck keeps the same
minimal neon identity, score, life, and meaningful-combo language and adds no
control or instruction.

## Non-goals

- Changing player presence, tracking, pose projection, scoring, collision,
  camera layout, action-beacon placement, PIP placement, audio, assets, or ABI.
- Moving the HUD to a lane-dependent corner or making it chase the avatar.
- Adding a tutorial, legend, persistent copy, preference, or touch action.
- Claiming synthetic camera input proves physical camera-distance readability,
  real multi-person identity, or two-peer behavior.
