# Change: Honest Guide-Only Camera Return

**Date:** 2026-07-21  
**Target:** `brainbreak` launcher fallback and camera return flow  
**Status:** M8 implementation and automated/browser QA complete; upstream audit residual remains

## Goal

Turn the current guide-only dead end into an honest visual demo. Choosing the
no-camera path must keep evaluation disabled, show the selected same-style game
image, and provide one obvious camera action plus one quiet way back to modes.

## Milestone: M8 — Demo Without Dead End

### In scope

- Rename the launcher fallback so it promises a demo, not playable motion.
- Add a responsive guide-demo dialog using the selected mode illustration.
- Keep one 60 px primary camera CTA in the phone thumb zone.
- Keep a secondary return-to-modes action and normal keyboard semantics.
- Exit guide-only presentation before entering camera setup.
- Preserve camera-first scoring and the one-touch-then-no-touch contract.

### Out of scope

- Allowing keyboard, touch, or gamepad actions to score without camera evaluation.
- Adding an autonomous scripted gameplay bot.
- Changing Rust scoring, recognizers, bridge ABI, or the camera model.
- Production deployment or claims about a physical camera.

### Acceptance criteria

1. The launcher labels the fallback as a no-camera demo.
2. The demo shows the resolved mode image/title and explicitly states that score,
   collision judgment, and multiplayer actions are disabled.
3. Camera is the dominant action with at least a 60 px target; returning to mode
   selection remains visible but secondary.
4. Camera return clears guide-only presentation before the existing ready flow;
   back returns to the selected mode without altering evaluation safety.
5. Phone portrait, phone landscape, and desktop layouts remain inside the
   viewport with usable focus order and no app-origin runtime regression.

## Evidence

See `validation.md`. The fallback now has no navigation dead end and does not
weaken camera-first evaluation. The full dependency audit retains the upstream
Sharp/Miniflare residual already recorded by M7.
