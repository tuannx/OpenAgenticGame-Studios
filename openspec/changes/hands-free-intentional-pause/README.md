# Change: Hands-Free Intentional Pause

**Date:** 2026-07-21  
**Target:** `brainbreak` camera action vocabulary, runner pause state, and pause presentation  
**Status:** M12 implemented and validated; attended camera ergonomics remain

## Goal

Let a player intentionally pause and resume from camera distance without
touching the device or deliberately leaving the frame. The pause gesture must be
distinct from short jumps and rhythm claps, and the deterministic runtime must
freeze before another gameplay judgment occurs.

## Milestone: M12 — Intentional Motion Pause

### In scope

- Recognize a one-second, both-hands-high-and-separated pause hold.
- Keep pause as a control action that cannot score, miss, or break combo.
- Add explicit `Paused` and `PauseResuming` runner states.
- Clear in-flight hazards on pause, require evaluated clap readiness, and resume
  through the existing two-second countdown.
- Require both P1 and P2 to clap before a Duo run resumes.
- Render a large shape-first pause icon and one concise instruction.

### Out of scope

- Browser camera APIs, pose transport format, bridge imports/version, scoring,
  obstacle balance, audio ABI, or touch controls.
- Persisting pause poses or changing tracking thresholds.
- Claiming physical gesture ergonomics without an attended camera run.

### Acceptance criteria

1. Pause fires only after both confident wrists remain above their shoulders,
   separated beyond clap distance, for one second; a jump-sized transient or
   overhead clap cannot trigger it.
2. An evaluated pause trigger enters `Paused` before action, beat, distance,
   score, life, combo, or collision progression and clears hazards.
3. Pause is excluded from rhythm/mirror scoring and miss feedback.
4. Single-player resumes after an evaluated clap; Duo requires evaluated claps
   from both players, then enters a frozen two-second `PauseResuming` countdown.
5. Tracking loss during intentional resume returns to `Paused`, while tracking
   recovery continues to return to `TrackingHold`.
6. The pause icon, status, and countdown stay inside phone portrait, phone
   landscape, and desktop viewports without HUD/cue competition.

## Evidence

See `validation.md`. Automated recognizer/state/layout coverage, release WASM,
and HTTP loading passed. They remain separate from attended physical gesture and
camera-distance evidence.
