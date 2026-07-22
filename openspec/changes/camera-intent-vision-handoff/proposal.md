# Proposal

## Measured problem

On a fresh local origin, Browser `pageAssets` observed
`vision-BoLXREif.js` before any camera interaction. The launcher restores overlay
mode and theme by importing `vision.ts` twice, so its 1.35 MB uncompressed chunk
enters the initial network path despite being declared dynamic.

After the camera button is pressed, the stance screen immediately says it is
finding a player even while JavaScript is still loading, permission may still be
pending, or TensorFlow is selecting a backend. Two prominent touch fallback
buttons also compete with the intended hands-free gesture instructions.

## Scope

- Keep overlay/theme preferences in a light browser adapter without importing
  TensorFlow or MoveNet at launcher boot.
- Load the vision module once on explicit camera intent, apply the latest
  preferences, and allow a failed import to retry cleanly.
- Replace free-form vision status strings with typed, UI-agnostic phase events.
- Show concise Vietnamese progress for module preparation, permission,
  initialization, calibration, and tracking.
- Preserve the selected neon mode image and demote touch controls into a
  collapsed accessibility fallback.

## Non-goals

- No camera auto-start, background permission prompt, or speculative idle fetch.
- No pose model/backend/threshold change.
- No Rust gameplay, bridge ABI, scoring, audio, or P2P change.
- No production deployment or field-performance claim.

## Acceptance criteria

1. A fresh launcher load does not request the generated vision chunk before
   explicit camera intent.
2. Restored or changed theme/overlay values reach vision when it eventually
   loads, without loading it merely to store preferences.
3. Concurrent camera requests share one import; an import rejection resets the
   loader and a later request retries.
4. Stance copy distinguishes preparing AI, requesting permission, initializing,
   calibrating, tracking, and recoverable error states without parsing strings.
5. The selected same-style mode image remains dominant; gesture controls remain
   visible while touch fallback controls are collapsed and accessible.
6. Strict TypeScript/tests, production build, full BrainBreak gate, fresh HTTP
   asset inventory, and 390×844/1440×784 browser layouts pass.
