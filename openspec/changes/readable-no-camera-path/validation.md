# Validation

## Pre-change evidence

- At immediate HTTP-served first paint, `#boot-screen` still had opacity 1 and
  no hidden class, but its z-index was 10 while the full-screen launcher was 20.
  `elementsFromPoint` and the screenshot placed all launcher art above boot, so
  the stale boot copy is dead presentation rather than the next visible screen.
- The real fallback measured 183.1x48px, 11px, weight 400, `#64748b`, and opacity
  0.76. Effective contrast over the launcher gradient measured 2.64-2.80:1.
- The dominant camera CTA measured 918x58px, 18px, weight 900. Improving the
  fallback text does not require changing its area or challenging that hierarchy.

## Automated evidence

Passed on the final tree on 2026-07-21:

- Strict TypeScript typecheck and 58 Vitest tests across 12 files.
- Strict Rust formatting/Clippy, 41 deterministic core tests, and 22 game/layout
  tests remain green.
- Release `wasm32-unknown-unknown`, production Vite build, Macroquad loader
  syntax/patch checks, and `git diff --check` passed.

The complete BrainBreak validator passed Rust/native/WASM, TypeScript, all tests,
and production build, then stopped at the unchanged dependency audit. The
`wrangler -> miniflare -> sharp` chain reports three inherited high findings;
the offered forced fix installs incompatible Wrangler 4.15.2 and was not applied.

## Presentation and contrast evidence

The final production action reads `Xem demo không camera • không điểm`. Chrome
reported 284.3x48px, `#94a3b8`, opacity 1, 12px, and weight 700. Its scroll width
matched its rendered width and scroll height matched 48px, so the single-line
action neither clipped nor created a hidden nested target.

Static sRGB calculation against both launcher-card gradient endpoints gives
7.40:1 over `#0a0e28` and 6.88:1 over `#210c40`, up from effective 2.64-2.80:1.
The camera CTA remains 918x58px at desktop, 18px/900, full-width, and gradient
filled. The fallback remains compact, transparent, underlined, and secondary by
shape and area rather than by illegibility.

## Browser and responsive evidence

The real production entry was exercised at desktop and rendered at 390x844 and
667x375 through same-origin QA frames:

- Desktop showed the complete illustrated launcher, dominant camera CTA,
  no-touch promise, local taste note, and readable fallback in one bounded card.
- Portrait retained the two-by-two canonical art grid, placement image, camera
  CTA, promise, taste note, and the complete fallback label above the fold.
- Short landscape retained the four-art row and complete action hierarchy; the
  longer fallback stayed one line without stealing width from the camera CTA.
- No camera/settings/runtime canvas chrome reappeared. Exactly one launcher
  dialog remained visible.

Keyboard QA cycled selected mode -> camera CTA -> guide-only -> selected mode.
Shift+Tab from the selected mode returned to guide-only. Enter opened guide-only
with its camera action focused; Escape returned to the launcher with the selected
mode focused. The truthful copy change added no focus stop or touch step.

The temporary responsive QA page was removed after evidence capture.

## Capability and runtime evidence

A fresh production reload remained idle for more than six seconds without an
MP3 or vision-chunk request. Explicit guide-only intent loaded the existing MP3
and no vision chunk. Application-origin warning/error filtering was empty.

Camera, vision, scoring, pose evaluation, multiplayer, taste persistence, audio
engine, Rust, bridge, and ABI source were unchanged.

## Release and HTTP evidence

- WASM remains `brainbreak-game-425aa5274bb6.wasm`, 954,575 bytes raw / 313,033
  gzip with unchanged full SHA-256.
- Final entry HTML is 10,673 bytes raw / 3,465 gzip; game JS is 34,142 raw /
  12,036 gzip; CSS is 30,519 raw / 7,338 gzip.
- Fingerprinted audio remains
  `special-spotlight-db0e06528b9a.mp3`, 3,079,985 bytes raw.
- Built and source attribution documents retain identical SHA-256
  `f04d45438657db9b3f118dfddcbc3ed2ba9c2c511f4db92f7629cfc2e75d0b3b`.
- Explicit IPv4 HTTP returned `text/html`, `application/wasm`,
  `text/javascript`, `text/css`, `audio/mpeg`, and `text/markdown` for the final
  entry, WASM, game script, stylesheet, MP3, and attribution document.

## Persona review

- **Security/privacy:** the fallback discloses its no-camera/no-score consequence
  earlier. No permission, storage, camera, pose, or network surface widened.
- **Performance:** one text change and bounded CSS token changes add no asset,
  listener, state, detector, frame-loop, or WASM work.
- **Readability/accessibility:** the 48px action now clears contrast requirements,
  keeps visible underline/focus treatment, and states its evaluation limit before
  navigation.
- **Runtime regression:** launcher ownership, guide focus loop, camera laziness,
  guide audio behavior, and all deterministic evaluation boundaries remain intact.

## Residual attended QA

Camera permission and physical audio playback were not started. A real device
session must still verify camera denial/recovery, target-device screen-reader
announcement, safe-area readability, low-brightness visibility, and audible mix.
Build, HTTP, headful keyboard, and static contrast evidence do not prove those
attended surfaces.
