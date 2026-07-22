# Change: Truthful Shape-First Pause Recovery

**Date:** 2026-07-21  
**Target:** `brainbreak-game` deterministic intentional `Paused` presentation  
**Status:** M17 implemented and validated at semantic, layout, native,
release-WASM, HTTP, and browser-startup levels; attended pause/recovery QA
remains

## Goal

Keep the intentional pause control unmistakably simple while showing whether
each required player is missing, present, or clap-ready, so nobody must approach
the screen or guess why resume has not started.

## Milestone: M17 — Truthful Shape-First Pause Recovery

### In scope

- Derive pause guidance from deterministic evaluated and latched-ready facts.
- Preserve the dominant two-bar pause glyph and `PAUSED` title.
- Reuse the shared framed, neutral, and raised-hand player figures for single
  and Duo resume states.
- Replace bullet/abbreviated partial-ready copy with short static ASCII actions.
- Expand responsive layout and semantic coverage.

### Out of scope

- Pause recognition, dwell geometry, readiness actions, countdown duration,
  tracking thresholds, core transitions, bridge imports, or touch controls.
- Adding photographic mode art to the safety/control state; M12 deliberately
  keeps the pause symbol dominant while the frozen world preserves continuity.
- Claiming physical gesture or room-distance behavior without camera evidence.

### Acceptance criteria

1. Pause presentation uses the same evaluated/ready facts as the core, covers
   every required single/Duo state, and never tells an unevaluated player to
   clap before returning to frame.
2. The two-bar glyph and `PAUSED` remain dominant; searching, present, and ready
   players use distinct geometry and stable P1/P2 positions without relying on
   color, emoji, bullets, or owned per-frame strings.
3. Copy exposes only the next useful action: return to frame, clap to resume, or
   get ready. Merely present players never receive a ready shape.
4. Panel, pause bars, title, instruction, and one/two player figures remain
   contained at 390x844, 667x375, and 1440x784; core and ABI remain unchanged.
5. Native tests, release WASM, production web build, HTTP/browser startup, and
   attended physical-camera evidence are reported separately.

## Evidence

See `validation.md` after implementation.
