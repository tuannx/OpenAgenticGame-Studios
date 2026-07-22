# Change: Camera Framing Coach

**Date:** 2026-07-21  
**Target:** `brainbreak` camera-ready flow and no-touch motion navigation  
**Status:** M6 implementation and automated/browser permission smoke complete; attended tracked-camera QA pending

## Goal

Turn the first live-camera screen into a truthful AR setup coach. A detected
pose is not automatically playable: the browser must guide the player into a
usable head–shoulder–hip–hand frame, wait briefly for stable identity, and only
then allow lean and raised-hand navigation.

## Milestone: M6 — Stable Framing Before Motion Navigation

### In scope

- Pure, local-only pose-framing assessment with typed outcomes.
- One concise Vietnamese instruction at a time: enter frame, move closer/back,
  center, show the required body landmarks, invite P2, or begin gestures.
- A short stable-pose dwell before pose data reaches setup navigation.
- A neon reticle on the real camera preview, using the active visual theme.
- Existing touch fallback retained but secondary and unavailable until a safe
  tracked state or recoverable camera error.
- Deterministic unit tests, full BrainBreak validation, and served responsive
  browser QA without claiming physical-camera behavior that was not observed.

### Out of scope

- Changing Rust recognizer/scoring thresholds or bridge ABI.
- Persisting pose, camera frames, or biometric/framing measurements.
- Treating synthetic pose fixtures as proof of real camera distance or lighting.
- Deploying a production build.

### Acceptance criteria

1. Low-quality, cropped, edge-bound, too-near, and too-far poses do not reach
   setup navigation and produce one specific recovery instruction.
2. A usable pose remains stable for at least 400 ms before lean/confirm input is
   accepted; loss of framing cancels the active confirmation immediately.
3. Duo invites P2 when only one player is framed while still allowing P1 to lean
   to another mode; two-player confirmation remains owned by motion navigation.
4. The camera preview exposes a theme-consistent framing reticle and clear
   ready/not-ready state without adding a touch step or storing raw camera data.
5. Strict TypeScript, focused tests, full Rust/native/WASM/web gates, and local
   HTTP/browser smoke pass with responsive layouts and no app-origin errors.

### Fallback

If framing is unreliable, the existing collapsed touch disclosure remains the
accessible escape hatch. Guide-only mode remains non-evaluated.

## Evidence

See `validation.md` for the final command, browser, persona-review, and residual
physical-camera evidence. No production deployment was requested.
