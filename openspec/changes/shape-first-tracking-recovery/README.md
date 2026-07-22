# Change: Shape-First Tracking Recovery

**Date:** 2026-07-21  
**Target:** `brainbreak-game` deterministic `TrackingHold` presentation  
**Status:** M16 implemented and validated at semantic, layout, native,
release-WASM, HTTP, and browser-startup levels; attended tracking-loss recovery
QA remains

## Goal

Make sensor-loss recovery unmistakably different from an intentional pause,
while preserving the selected mode's visual identity and giving one/two players
a no-touch path back into the run from camera distance.

## Milestone: M16 — Shape-First Tracking Recovery

### In scope

- Map deterministic evaluated/ready facts to static recovery presentation.
- Reuse the canonical selected-mode art and M15 motion-figure language.
- Show searching, present, and ready P1/P2 states by geometry rather than color,
  emoji, or punctuation glyphs.
- Replace the generic non-countdown status branch with a dedicated recovery
  renderer, leaving the countdown renderer focused on `2 / 1` only.
- Add semantic and responsive layout coverage.

### Out of scope

- Tracking thresholds, pose inference, readiness actions, countdown duration,
  pause behavior, scoring, bridge imports, or browser permission flows.
- New art, touch recovery controls, or another DOM owner for runtime state.
- Claiming physical-room readability without a permissioned camera run.

### Acceptance criteria

1. Recovery presentation derives from `RunnerGame` evaluated and ready facts,
   distinguishes every required single/Duo recovery state, and never labels a
   merely present player as ready.
2. `TrackingHold` no longer says the intentional-pause title `PAUSED`; it uses
   short static ASCII copy plus framed-search, neutral-presence, and raised-hand
   figures that remain distinct without color.
3. The selected mode's existing texture is borrowed and center-cropped with a
   procedural fallback; no asset, decode, path, or owned string is created in
   the frame loop.
4. Recovery panel, art, copy, and both player cues remain inside 390x844,
   667x375, and 1440x784; `RunnerGame::update` and ABI remain unchanged.
5. Native tests, release WASM, production web build, HTTP/browser startup, and
   attended physical-camera evidence are reported as separate truth levels.

## Evidence

See `validation.md` after implementation.
