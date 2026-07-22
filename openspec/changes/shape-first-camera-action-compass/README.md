# Change: Shape-First Camera Action Compass

**Date:** 2026-07-21  
**Target:** Browser camera-ready navigation presentation  
**Status:** M19 implemented and validated at typed-presentation, native,
release-WASM, HTTP, desktop, portrait, and short-landscape permission-state
levels; attended tracked-camera QA remains

## Goal

Make the one-touch-to-no-touch handoff readable from camera distance: keep the
selected canonical mode image, replace emoji/sentence navigation with geometric
left/right cues and a raised-hand radial hold, and show only one current physical
correction.

## Milestone: M19 — Shape-First Camera Action Compass

### In scope

- Reuse the selected mode's canonical neon image in the ready screen.
- Replace font arrows, hand emoji, and the linear percentage bar with bounded
  CSS/SVG geometry: two lean chevrons and one radial raised-hand hold.
- Map framing/navigation state through a typed pure presenter with static action
  copy and an explicit visual-state discriminator.
- Remove per-pose visible percentage formatting and emoji from camera-ready
  status copy.
- Preserve the collapsed touch fallback and current accessibility progress
  semantics.

### Out of scope

- Pose thresholds, identity stability, camera capacity, hold duration, gesture
  confirmation, selected-mode policy, Rust evaluation, bridge ABI, or new assets.
- Changing launcher or guide-demo controls outside the camera-ready state.
- Claiming physical readability, recognition, or timing without permissioned
  camera evidence.

### Acceptance criteria

1. Framing truth always outranks navigation guidance; the presenter emits one
   static correction or one stable ready/holding state without emoji, bullets,
   or a visible numeric percent.
2. The selected canonical image remains the dominant identity surface while
   left/right chevrons and a radial raised-hand glyph carry action meaning
   without relying on font symbols or color alone.
3. The radial ring derives only from existing `confirmProgress`; no second
   timer, threshold, state machine, or touch action is introduced.
4. Camera-ready copy, action compass, fallback, and live camera card remain
   contained by existing portrait, short-landscape, and desktop breakpoints.
5. TypeScript tests, production build, HTTP/browser startup and permission-state
   composition, and residual attended camera QA are reported separately.

## Evidence

See `validation.md`.
