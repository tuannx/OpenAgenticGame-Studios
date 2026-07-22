# Change: Time-Boxed Positive Brain Break

**Date:** 2026-07-21  
**Target:** `brainbreak` deterministic session clock, result presentation, and local aggregate taste learning  
**Status:** M13 implemented and validated at deterministic, native, release-WASM,
and HTTP/browser-launcher levels; attended physical-camera pacing remains

## Goal

Make every camera-backed run a bounded 90-second brain break that ends on a
positive, hands-free result even when the player never loses all energy. Learn
from the completed run using constant-size local aggregates only.

## Milestone: M13 — Bounded Positive Session

### In scope

- Count 90 seconds of active `Running` time in the deterministic Rust runtime.
- Freeze that clock during setup, countdown, pause, and tracking recovery.
- Finish before another action, collision, or world step at the time boundary.
- Distinguish `BreakComplete` from `EnergySpent` and emit one result event.
- Present a shape-first session rail and positive result copy without new touch.
- Record bounded per-mode completion and outcome aggregates in local taste data.
- Bump and verify both bridge version values for the new result-event import.

### Out of scope

- Changing obstacle balance, camera thresholds, pause gestures, P2P payloads, or
  the 126 BPM music clock.
- Persisting frames, landmarks, body geometry, pose history, or player identity.
- Claiming physical pacing or result readability without an attended camera run.

### Acceptance criteria

1. Only evaluated active `Running` frames advance the shared 90-second clock;
   every non-running phase freezes it.
2. Reaching the limit enters results and clears hazards before another movement,
   score tick, beat, collision, or life judgment occurs.
3. The runtime emits exactly one typed result event per run and replay resets the
   clock/outcome while preserving personal bests.
4. Running shows one non-text-dependent progress rail; the result remains
   positive and hands-free for both timeout and depleted-energy outcomes.
5. The browser stores only bounded outcome counters, uses them to weight mode
   taste, and the release WASM imports the synchronized bridge v7 contract.

## Evidence

See `validation.md`. Deterministic rules, native execution, release WASM, HTTP
load, and attended physical-camera pacing remain separate truth levels.
