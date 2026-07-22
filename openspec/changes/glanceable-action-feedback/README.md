# Change: Glanceable Action Feedback

**Date:** 2026-07-21  
**Target:** `brainbreak` gameplay outcome feedback and HUD hierarchy  
**Status:** M10 implementation and automated/browser ABI QA complete; physical
camera/audio tuning remains pending

## Goal

Make each evaluated outcome understandable while the player remains at camera
distance. Success, beat pickup, and collision recovery need distinct shapes,
short positive language, and combo-aware sound without obscuring the next action
or changing deterministic judgment.

## Milestone: M10 — Action Outcome Hierarchy

### In scope

- Retain each one-frame deterministic feedback event in a bounded 0.72-second
  presentation pulse.
- Draw distinct check, beat-spark, and recovery-cross markers with a player
  identity accent and short labels.
- Compress the bottom HUD into score, three shape-coded life pips, and combo only
  when the multiplier is meaningful.
- Pass the evaluated player's combo to browser feedback audio through bridge ABI
  v6 and pitch only the feedback oscillator.
- Keep the 126 BPM backing track playback rate unchanged.

### Out of scope

- Changing recognition, collision, score, life, combo, obstacle timing, or game
  duration.
- Adding blocking praise popups, camera-derived analytics, new image assets, or
  touch controls.
- Claiming physical camera/audio quality without an interactive permissioned run.

### Acceptance criteria

1. Dodge, beat pickup, and collision recovery have distinct shape-plus-label
   presentations that remain visible long enough to register but under 0.9s.
2. Up to four simultaneous feedback markers use a bounded responsive row and do
   not cover the action beacon or bottom HUD at target viewports.
3. HUD lives work without color, combo x1 is hidden, and all HUD geometry stays
   inside 390x844, 667x375, and 1440x784.
4. Rust passes the actual evaluated-player combo through synchronized bridge ABI
   v6; successful SFX pitch rises by one semitone per five combo, capped at four.
5. Background music remains at playback rate 1.0 so audio metrics continue to
   represent the declared 126 BPM clock.

## Evidence

See `validation.md`. The final WASM exposes bridge version 6 and imports the
two-argument feedback function, while the HTTP-served release loads without an
app-origin runtime error at all target viewport classes. Camera-distance
readability, audible mix quality, and haptic feel remain physical-session
evidence, not build claims.
