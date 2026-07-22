# Change: Illustrated Shape-First Ready State

**Date:** 2026-07-21  
**Target:** `brainbreak-game` deterministic Ready presentation  
**Status:** M15 implemented and validated at semantic, layout, native,
release-WASM, HTTP, and browser-startup levels; direct attended Ready-screen QA
remains

## Goal

Keep the selected BrainBreak mode visually recognizable after the DOM setup
screen hands control to Macroquad, while making the no-touch start action
readable by body shape from camera distance.

## Milestone: M15 — Illustrated Shape-First Ready State

### In scope

- Reuse the already-loaded canonical mode texture in the Ready overlay with a
  procedural fallback.
- Map runtime facts to a typed, allocation-free Ready presentation.
- Replace emoji-dependent copy with short static labels and procedural player,
  camera, and raised-hand shapes.
- Distinguish searching, present, and ready players without relying on color.
- Keep the panel inside phone portrait, phone landscape, and desktop targets.

### Out of scope

- Gesture recognition, scoring, player evaluation, countdown, pause, result,
  bridge imports, or mode selection behavior.
- New art generation or duplicate assets.
- Claiming camera-distance readability without an attended physical run.

### Acceptance criteria

1. Every Ready state uses a typed static presentation and creates no owned
   title or instruction string in the frame render path.
2. The canonical selected-mode art is borrowed from the existing one-time
   `ModeArt` load, center-cropped without stretching, and missing art retains a
   complete procedural presentation.
3. Camera unavailable, no player, one-player-found, signal-waiting, and P1/P2
   ready facts have distinct short copy and geometric cues; no emoji glyph is
   required to understand the action.
4. The Ready panel, art, copy region, and one/two-player cues stay contained at
   390x844, 667x375, and 1440x784; runtime state transitions remain untouched.
5. Native tests, release WASM, production web build, HTTP delivery, and browser
   startup are verified separately; attended physical-camera QA remains
   explicit when unavailable.

## Evidence

See `validation.md` after implementation. Geometry, browser startup, and
physical distance readability remain separate proof levels.
