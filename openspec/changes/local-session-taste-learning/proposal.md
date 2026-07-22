# Proposal

## Problem

The M1 launcher gives first-time players a clear camera-placement guide and a
one-touch-to-no-touch entry flow. Repeat players currently see the same amount
of setup every time, even when they repeatedly choose the same game and already
complete motion readiness quickly.

## Scope

- Remember completed game-mode starts and visual preferences on this device.
- Capture aggregate setup outcomes: camera attempts, gesture starts, touch
  fallback starts, guide-only starts, setup exits, and gesture time-to-ready.
- Recommend a repeat mode only after a stable preference signal.
- Condense, but never remove, the placement guide after repeated fast gesture
  success.
- Restore the full guide after the most recent fallback or setup failure.
- Explain the local-only learning boundary in the launcher.

## Non-goals

- No server sync, account identity, cross-device profiling, analytics beacon,
  raw camera data, pose/keypoint storage, or per-frame event history.
- No changes to authoritative Rust scoring or judgment.
- No automatic camera permission request and no removal of accessibility touch
  fallbacks.
- No production deployment in this milestone.

## Acceptance criteria

1. Corrupt or unavailable browser storage cannot block launch.
2. A mode is not recommended until it has been started at least twice.
3. Compact setup requires at least two fast gesture-confirmed starts and a
   successful latest outcome; guide-only, fallback, or exit restores full setup.
4. Theme, overlay mode, and camera opacity restore locally with validated bounds.
5. The serialized profile contains aggregate values only and no pose, image,
   video, or keypoint fields.
6. First-time and returning launchers remain responsive and keyboard accessible.

