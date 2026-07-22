# Validation

Validation evidence will be recorded after implementation.

## Pre-change evidence

- The real guide state exposed only one modal, hid settings/camera, focused its
  camera action, and kept the canvas inert, so behavior ownership was correct.
- Computed canvas visibility and opacity remained `visible` and `1`.
- The desktop screenshot visibly exposed `BEAT STRIKE` in the top-left,
  `GUIDANCE ONLY` in the top-right, and lower player silhouettes outside the
  guide card. Central neon sun/lane ambience was useful and should be retained.
- A first opaque-circle pass removed both corner labels but its radius kept the
  lower center translucent, so player silhouettes remained. The final design
  uses an ellipse and a 52% opaque stop to shorten that vertical reveal without
  flattening the horizon, plus a bottom fade to remove the last lower-third heads.
- Keyboard QA exposed that leaving the guide made the launcher visible but left
  focus on `body`. The selected mode card is now focused on the next animation
  frame, after its hidden parent has become focusable.
- The combined portrait/landscape capture exposed very faint corner labels that
  desktop no longer showed. A symmetric side mask now reserves translucency for
  only the centered horizon instead of letting radial geometry reopen corners
  on narrow viewports.

## Automated evidence

Passed on the final tree on 2026-07-21:

- Strict TypeScript typecheck and all 58 Vitest browser-logic tests.
- Strict Rust formatting/Clippy, 41 deterministic core tests, and 22
  game/layout tests.
- Release `wasm32-unknown-unknown`, production Vite build, Macroquad loader
  syntax/strict-mode patch checks, and `git diff --check`.

The complete validator passed native/Rust/WASM, TypeScript, tests, and build,
then stopped at the unchanged dependency audit. The inherited
`wrangler -> miniflare -> sharp` chain reports three high findings. The offered
forced fix would install incompatible Wrangler 4.15.2 and was not applied.

## Browser and responsive evidence

The HTTP-served production entry was exercised in Chrome at desktop and in
same-origin 390x844 and 667x375 frames:

- The final three-layer scrim keeps the centered neon sun and upper road as
  ambience while fully suppressing both corner labels and lower player/HUD
  silhouettes.
- The canonical Mirror Beat illustration and guide copy/actions stay complete
  and bounded in portrait and short landscape. The temporary QA page was removed.
- Exactly one guide dialog is visible. `#glcanvas` remains visible at opacity 1
  and inert; settings and camera chrome do not reappear.
- The camera and return actions remain 60px and 48px high. Focus starts on the
  camera action and cycles camera -> back -> camera in both directions.
- Escape and the return action both reopen the launcher with the selected Mirror
  mode card focused after the next animation frame.
- Application-origin warning/error filtering was empty. The first dual-WebGL
  capture showed a transient compositor clip; the delayed retry was clean and
  no CSS change was made for that tooling artifact.

## Capability and resource evidence

A fresh origin remained idle for more than six seconds without MP3 or vision
requests. Explicit guide-only intent loaded the existing fingerprinted MP3 and
no vision chunk. Camera, detector, scoring, collision, multiplayer, taste state,
audio engine, Rust, bridge imports, and ABI were unchanged.

## Release and HTTP evidence

- WASM remains `brainbreak-game-425aa5274bb6.wasm`, 954,575 bytes raw / 313,033
  gzip with unchanged content SHA-256.
- Final entry HTML is 10,673 bytes raw / 3,466 gzip; game JS is 34,179 raw /
  12,053 gzip; CSS is 30,679 raw / 7,384 gzip.
- Fingerprinted audio remains
  `special-spotlight-db0e06528b9a.mp3`, 3,079,985 bytes raw.
- Built and source attribution documents retain identical SHA-256
  `f04d45438657db9b3f118dfddcbc3ed2ba9c2c511f4db92f7629cfc2e75d0b3b`.
- Explicit IPv4 HTTP returned `text/html`, `application/wasm`,
  `text/javascript`, `text/css`, `audio/mpeg`, and `text/markdown` for the final
  entry, WASM, game script, stylesheet, MP3, and attribution document.

## Residual attended QA

Camera permission and physical audio playback were not started. A real-device
session must still verify camera denial/recovery, target-device screen-reader
announcement, safe-area readability, low-brightness visibility, and audible mix.
