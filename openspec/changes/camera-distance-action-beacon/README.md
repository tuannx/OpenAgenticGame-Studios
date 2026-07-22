# Change: Camera-Distance Action Beacon

**Date:** 2026-07-21  
**Target:** `brainbreak` deterministic gameplay presentation  
**Status:** M9 implementation and automated/browser smoke QA complete; physical
camera-distance visual QA remains pending

## Goal

Replace the small text-only next-obstacle cue with one action beacon that remains
legible from camera distance. The beacon must communicate action, obstacle lane,
and approach timing through shape as well as text without changing scoring or
collision ownership.

## Milestone: M9 — Shape-First Gameplay Cue

### In scope

- Draw one large action silhouette for dodge, jump, squat, or clap.
- Show the obstacle lane as a three-slot spatial diagram instead of a directional
  instruction.
- Show approach timing with a bounded proximity rail.
- Keep the beacon responsive in phone portrait, phone landscape, and desktop.
- Record the durable camera-distance cue rule and focused layout evidence.

### Out of scope

- Changing obstacle generation, recognition, scoring, collision, or multiplayer.
- Adding DOM gameplay controls, debug scoring, or a simulated camera player.
- Changing browser imports, bridge ABI, audio, or vision capability.
- Claiming physical two-metre readability without a permissioned camera session.

### Acceptance criteria

1. Every hazard maps to a distinct pictogram and concise action verb.
2. Lane information uses three spatial slots, with the obstacle lane encoded by
   fill and outline rather than color alone.
3. Proximity is clamped and increases monotonically as the obstacle approaches.
4. The beacon stays inside all target viewports and does not overlap the header.
5. Native tests, release WASM, web build, and served browser smoke checks retain
   the existing deterministic safety boundary.

## Evidence

See `validation.md`. The shipped bundle now contains the shape-first beacon,
passes native and release WASM gates, and loads without an app-origin browser
warning. A physical camera-distance gameplay run remains an explicit evidence
gap because browser QA did not bypass camera-backed evaluation to enter Running.
