# Design

## Ownership

- `vision.ts` owns camera, backend selection, detector lifecycle, and typed
  `VisionStatus` events; it does not own localized UI copy.
- `vision-loader.ts` owns the lazy import promise, latest overlay preferences,
  deduplication, retry after rejection, and stop-if-loaded behavior.
- `vision-status.ts` maps typed events to small presentation descriptions.
- `main.ts` owns DOM transitions and the one explicit camera-intent boundary.

## Loader contract

Constructing the loader or updating preferences performs no import. `load()`
creates one pending promise, applies the latest preferences after resolution,
and returns the same module to concurrent callers. Rejection clears only that
pending promise so retry remains possible. `stopIfLoaded()` never triggers a
new download and safely handles an import failure.

This prevents the current failure path where cleanup can re-import the same
module that just failed to load.

## Status contract

```text
launcher -- camera intent --> preparing-module
vision --> requesting-camera --> initializing-ai --> calibrating --> tracking
                                                   \-> inference-error
```

`VisionStatus` is a discriminated union. UI copy is exhaustive in a light module
that does not depend on TensorFlow. Pose-driven gesture hints continue to own the
tracking-ready state so backend status cannot overwrite lean/hold feedback.

## Presentation

- Reuse `ready-pose-img` and the selected mode artwork; introduce no new style.
- Show one live status badge over that image.
- Keep lean/raise-hand guidance and hold meter visible as the primary path.
- Put touch start/back actions inside a collapsed semantic `details` fallback.
- Preserve ≥44 px fallback summary target and keyboard/screen-reader semantics.

## Performance evidence

The before/after proof is asset presence, not a guessed timing claim: on a fresh
origin, launcher `pageAssets` must exclude the generated `vision-*.js` chunk.
Production chunk size remains reported separately because camera intent still
needs to download it.
