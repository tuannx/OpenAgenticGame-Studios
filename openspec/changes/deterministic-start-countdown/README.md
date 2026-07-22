# Change: Deterministic Start Countdown

**Date:** 2026-07-21  
**Target:** `brainbreak` first-run and hands-free replay transitions  
**Status:** M11 implementation and automated/HTTP browser QA complete; attended
camera timing QA remains pending

## Goal

Give players a short, visible preparation beat after camera-backed readiness and
before every run starts. The deterministic runtime must own this transition so
first launch and result replay cannot enter moving hazards abruptly or diverge
from tracking-resume safety.

## Milestone: M11 — Camera-Distance Run Entry

### In scope

- Add an explicit deterministic `Starting` phase for first launch and replay.
- Keep the world frozen through the existing two-second countdown.
- Return an interrupted initial countdown to `Ready`; keep interrupted tracking
  resume routed to `TrackingHold`.
- Reuse the large Macroquad countdown presentation for `Starting` and
  `Resuming`, without adding a second DOM countdown.
- Cover single-player, Duo, replay, countdown freeze, and tracking-loss paths.

### Out of scope

- Changing camera recognition, setup navigation, scoring, hazards, game length,
  audio ABI, or bridge version.
- Adding new touch controls, image assets, or a longer tutorial sequence.
- Claiming physical camera-distance or audio quality without an attended run.

### Acceptance criteria

1. An evaluated ready signal enters `Starting(2.0s)` instead of `Running`, and
   no beat, distance, score, collision, or obstacle progression occurs until the
   following frame after countdown completion.
2. Duo enters `Starting` only after P1 and P2 are both evaluated and ready.
3. Hands-free result replay resets the run into the same `Starting` countdown
   while preserving personal best state.
4. Losing required evaluation during `Starting` returns to `Ready`; losing it
   during `Resuming` still returns to `TrackingHold` and clears hazards.
5. `Starting` and `Resuming` share one camera-distance `GET READY` presentation,
   while HUD, action cue, and beat meter remain hidden.

## Evidence

See `validation.md`. Native Rust, release WASM, web build, and HTTP-served clean
loads pass. The full validator stops at the known transitive
`wrangler -> miniflare -> sharp` audit finding; it is recorded separately from
the M11 runtime result. An attended camera run remains required for physical
readability and timing quality.
