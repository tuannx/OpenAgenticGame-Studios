# Change: Illustrated Capability-Truthful Result Carousel

**Date:** 2026-07-21  
**Target:** `brainbreak-game` result presentation and canonical mode artwork  
**Status:** M14 implemented and validated at deterministic, native,
release-WASM, HTTP, and browser-load levels; attended result-screen QA remains

## Goal

Make the hands-free result screen feel like the same neon BrainBreak product as
the launcher and camera-ready screens. Reuse the canonical mode illustrations,
keep the selected mode readable by shape, and make an unavailable Duo choice
visibly unavailable instead of merely skipping it in runtime navigation.

## Milestone: M14 — Illustrated Truthful Result Choice

### In scope

- Load the existing Mirror, Strike, and Duo images once from their canonical web
  asset paths, with a graceful procedural fallback.
- Render those images inside the three result choices without embedding duplicate
  image bytes in the WASM artifact.
- Preserve border weight and a shape marker for the selected mode.
- Dim and lock Duo when both required evaluated players are not present, matching
  the deterministic navigation rule.
- Keep `LEAN LEFT / RIGHT TO CHOOSE` and `CLAP TO PLAY` as the only actions.
- Add pure crop/layout/availability tests and persist the learned visual rule.

### Out of scope

- Changing mode selection order, gestures, scoring, outcome semantics, camera
  capacity policy, bridge imports, or taste-profile schema.
- Replacing the canonical launcher artwork or generating another visual style.
- Claiming camera-distance result readability without an attended result run.

### Acceptance criteria

1. Result choices use the same three canonical illustrated assets as launcher
   and camera-ready setup; missing assets degrade to the existing procedural
   card rather than blocking startup or gameplay.
2. Artwork is center-cropped without stretching, remains clipped to each card,
   and every card plus both action hints stays inside phone portrait, phone
   landscape, and desktop target viewports.
3. Selection remains distinguishable without color using border weight and a
   geometric marker; Duo shows a lock-shaped unavailable state whenever its
   deterministic choice is unavailable.
4. Lean navigation and clap replay remain authoritative and no touch, DOM state,
   new WASM import, or per-frame asset allocation is introduced.
5. Native tests, release WASM, production web build, HTTP asset delivery, and
   browser loading are verified separately; attended physical result QA remains
   explicit if camera access is unavailable.

## Evidence

See `validation.md` after implementation. Build, HTTP loading, and physical
camera-distance readability remain separate truth levels.
