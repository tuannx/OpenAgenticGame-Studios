# Validation

## Pre-change evidence

- The production accessibility snapshot exposed the Ready dialog after camera
  intent but no active element inside it.
- Pressing Escape left the Ready dialog and live tracking state unchanged.
- Source inspection showed `getUserMedia()` could resolve after `stopVision()`
  and continue assigning/starting a stream because startup had no cancellation
  ownership.

## Final evidence

### Automated gates

Passed on the final tree on 2026-07-21:

- Strict TypeScript and all 72 Vitest browser-logic tests, including three
  cancellation tests and the Ready entry focus contract.
- Strict Rust formatting/Clippy, 41 deterministic core tests, 22 game/layout
  tests, release `wasm32-unknown-unknown`, production Vite build, Macroquad
  loader syntax/strict-mode checks, and `git diff --check`.

The complete validator stopped only at the unchanged dependency audit. The
inherited `wrangler -> miniflare -> sharp` chain reports three high findings;
the offered forced fix installs incompatible Wrangler 4.15.2 and was not
applied. The artifact, loader, checksum, and HTTP checks after that audit gate
passed manually.

### Browser behavior

The HTTP-served production bundle used a same-origin synthetic 640x480
MediaStream and the real lazy vision/MoveNet path:

- While camera permission was delayed 30 seconds, Ready itself was the active
  dialog. Tab moved to the visible collapsed fallback summary and Shift+Tab
  remained there; hidden launcher, canvas, settings, and disabled fallback
  buttons never became focus stops.
- Escape immediately returned to the launcher with `BẬT CAMERA` active while
  the request was still unresolved. When that request later resolved, the
  harness recorded one request, one resolution, and exactly one track stop; the
  launcher and focus owner remained unchanged.
- After real model initialization/tracking status, Escape again restored the
  launcher focus and stopped the active synthetic track exactly once.
- The explicit `Chọn lại game` fallback restores focus to the selected launcher
  mode card rather than retaining focus on the hidden Ready action.
- At 390x844, the camera preview/status and full illustrated Ready card stayed
  contained and Ready owned focus. At 667x375, the compact split layout retained
  the action compass, camera status, privacy badge, and Ready focus with no
  overflow visible in the captured viewport.
- No application error or warning was recorded. Console noise was limited to
  existing Macroquad unused-plugin logs and unrelated Chrome-extension warnings.

Synthetic video proves application focus, cancellation, model handoff, stream
cleanup, and responsive presentation. It does not prove the operating-system
permission prompt, physical-camera alignment, gesture thresholds, or
screen-reader announcement cadence.

### Release and HTTP evidence

- WASM remains `brainbreak-game-425aa5274bb6.wasm`, 954,575 bytes, SHA-256
  `425aa5274bb6c9a3766e5e384505d97e65d3817ea163cae9731f218f562ce0f5`.
- Final entry HTML is 10,365 bytes; `game-CjAqsFNd.js` is 36,469 bytes;
  `vision-D6t-5xsv.js` is 1,348,518 bytes; CSS is 31,047 bytes. Fingerprinted audio remains
  `special-spotlight-db0e06528b9a.mp3`, 3,079,985 bytes.
- Built/source attribution remain byte-identical with SHA-256
  `f04d45438657db9b3f118dfddcbc3ed2ba9c2c511f4db92f7629cfc2e75d0b3b`.
- Explicit IPv4 HTTP returned `text/html`, `application/wasm`,
  `text/javascript`, `text/css`, `audio/mpeg`, and `text/markdown` for the entry,
  WASM, game script, stylesheet, MP3, and attribution document.

### Ownership and non-regression

- Focus/cancellation live in the DOM/browser boundary; camera-backed evaluation,
  deterministic gameplay, pose navigation, scoring, multiplayer, Rust, bridge
  imports, ABI, assets, audio, and persisted taste schema are unchanged.
- The temporary same-origin QA harness was removed by the final production
  build and is absent from `web/dist`.

### Residual attended QA

A real device/person session must still verify the native permission sheet,
physical camera LED shutdown, screen-reader dialog/status cadence, mirrored
landmarks, one/two-person gestures, and legibility at 1.5-2.5 metres. Physical
audio, low-brightness/projector appearance, and real-device safe areas also
remain unverified.
