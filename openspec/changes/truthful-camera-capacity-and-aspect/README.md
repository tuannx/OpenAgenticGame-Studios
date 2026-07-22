# Change: Truthful Camera Capacity and Aspect

**Date:** 2026-07-21  
**Target:** `brainbreak` local motion-mode availability and camera preview  
**Status:** M7 implementation and automated/browser QA complete; attended camera QA and upstream audit fix pending

## Goal

Make every setup and result-screen choice reflect what the active local camera
pipeline can actually evaluate. A one-pose device must never offer, randomly
select, or navigate into Duo, and the preview must preserve the acquired camera
stream's intrinsic aspect instead of cropping it into a fixed 4:3 card.

## Milestone: M7 — Capability-Truthful Motion Setup

### In scope

- One lightweight capability contract shared by setup, lazy vision loading, and
  bridge mode normalization.
- Fail-closed Duo availability for local pipelines limited to one pose.
- Available-mode-only Random, touch, lean, keyboard, and result navigation.
- A visible, same-style unavailable Duo card with a concise reason.
- Camera and overlay geometry driven by intrinsic video dimensions, with no crop.
- Deterministic TypeScript/Rust tests, full native/WASM/web validation, and
  served responsive browser smoke.

### Out of scope

- Raising mobile pose capacity without measured inference evidence.
- Changing recognizer/scoring thresholds or the JavaScript/WASM import ABI.
- Persisting device fingerprints, camera frames, or pose measurements.
- Claiming physical skeleton alignment without an attended camera session.
- Production deployment.

### Acceptance criteria

1. Mobile and iPadOS-touch capability hints select the one-pose profile; desktop
   selects the two-pose profile.
2. With one-pose capacity, Duo is visibly unavailable and cannot be selected by
   Random, touch, keyboard, lean navigation, bridge writes, or result navigation.
3. With two-pose capacity, all three modes retain their current behavior.
4. The camera card and overlay use the same intrinsic stream aspect and do not
   crop the preview; portrait streams remain usable within phone bounds.
5. Strict TypeScript, focused tests, Rust/native/WASM/web gates, and local HTTP
   browser smoke pass without app-origin errors or responsive overflow.

## Evidence

See `validation.md`. Real-camera preview and landmark alignment remain pending
until camera permission and a physical player are available. The full project
validator currently stops at a newly published transitive `sharp` advisory whose
compatible Cloudflare dependency update is not released; no unsafe force-
downgrade or untested override was applied.
