# Change: Shape-Dominant Countdown Breath

**Date:** 2026-07-21  
**Target:** `brainbreak-game` deterministic start/resume countdown presentation  
**Status:** M18 implemented and validated at semantic, layout, native,
release-WASM, HTTP, and browser-startup levels; attended countdown QA remains

## Goal

Turn the two-second run-entry breath into a camera-distance visual rhythm: one
large geometric numeral and one draining time ring, with no extra instruction,
touch action, or competing image.

## Milestone: M18 — Shape-Dominant Countdown Breath

### In scope

- Map `countdown_remaining` to typed `Two`/`One` presentation.
- Draw numerals as bounded vector strokes instead of font glyphs.
- Draw a fixed 12-tick ring that drains within each one-second numeral stage.
- Keep the shared `GET READY` treatment for Starting, Resuming, and
  PauseResuming.
- Add pure stage/progress/stroke/layout tests.

### Out of scope

- Countdown duration, phase transitions, gesture consumption, tracking-loss
  destinations, gameplay progression, audio, bridge imports, or DOM timers.
- Adding mode photography to the two-second safety breath; the numeral must own
  the first glance as required by the existing taste contract.
- Claiming physical timing/readability without an attended camera run.

### Acceptance criteria

1. Presentation derives only from the core's public remaining time, handles
   bounded/non-finite values safely, and never mutates or mirrors countdown
   authority.
2. `2` and `1` use visibly distinct fixed vector strokes; a 12-tick ring drains
   monotonically within each numeral second and resets only at `2 → 1`.
3. `GET READY`, ring, ticks, and numeral remain contained at 390x844, 667x375,
   and 1440x784; no font numeral, image, emoji, or per-frame allocation is used.
4. Starting, Resuming, and PauseResuming retain one shared renderer while core
   state, timing, interruption destinations, and ABI remain unchanged.
5. Native tests, release WASM, web build, HTTP/browser startup, and attended
   physical-camera evidence are reported separately.

## Evidence

See `validation.md`.
