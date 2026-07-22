# Validation

## Pre-change evidence

- Immediate HTTP-served state: `#boot-screen` was a full-viewport `display:grid`
  element at opacity 1, pointer-events auto, z-index 10, without inert or
  `aria-hidden`; the launcher was the only visible dialog at z-index 20.
- After 1.2 seconds, the timeout added `.hidden`, but that class only set opacity
  0 and pointer-events none. Display remained grid and the element still lacked
  inert/`aria-hidden`.
- Both immediate and settled Chrome accessibility snapshots exposed the legacy
  `Neon Beat Runner` H1 and `Charging the rhythm highway…` paragraph after the
  complete `BRAINBREAK PARTY` launcher dialog.
- Because the launcher fully covers the boot surface and hides the canvas, the
  legacy surface supplied neither useful loading feedback nor layout protection.

## Automated evidence

Passed on the final tree on 2026-07-21:

- New `shell-entry.test.ts` locks the first-paint launcher, inert canvas, and
  absence of legacy boot markup/copy.
- Strict TypeScript typecheck and all 59 Vitest browser-logic tests.
- Strict Rust formatting/Clippy, 41 deterministic core tests, and 22 game/layout
  tests.
- Release `wasm32-unknown-unknown`, production Vite build, Macroquad loader
  syntax/strict-mode patch checks, built-source absence checks, and
  `git diff --check`.

The complete validator passed native/Rust/WASM, TypeScript, tests, and build,
then stopped at the unchanged dependency audit. The inherited
`wrangler -> miniflare -> sharp` chain reports three high findings. Its offered
forced fix installs incompatible Wrangler 4.15.2 and was not applied.

## Browser and responsive evidence

The production bundle was served over explicit IPv4 HTTP and inspected in
Chrome at immediate first paint and after 1.2 seconds:

- `#boot-screen` was absent at both points. The accessibility snapshot ended
  with the launcher dialog and no longer exposed the duplicate legacy H1/status.
- Exactly one launcher dialog was visible. The real canvas remained 2560x1233,
  inert and `visibility:hidden`, preserving Macroquad's layout surface.
- The production game script and local `mq_js_bundle.js` were present, and the
  server observed the fingerprinted WASM request.
- Keyboard remained body -> selected mode -> camera -> guide -> selected mode;
  reverse Tab returned to guide. Enter opened guide with its camera action
  focused, and Escape returned to the concrete mode resolved from Random.
- Same-origin 390x844 and 667x375 frames retained the complete illustrated
  launcher hierarchy above the fold, with no legacy loading copy in either
  accessibility subtree. The temporary QA file was removed.
- Application-origin warning/error filtering was empty.

## Capability and resource evidence

Fresh launcher frames loaded the local loader, fingerprinted WASM, canonical
mode art, and placement art without a vision request. The MP3 appeared only
after the explicit guide action in keyboard QA; no vision chunk was requested.
Camera, detector, audio engine, scoring, collision, multiplayer, Rust, bridge
imports, ABI, and assets were unchanged.

## Release and HTTP evidence

- WASM remains `brainbreak-game-425aa5274bb6.wasm`, 954,575 bytes raw / 313,033
  gzip with unchanged content SHA-256.
- Final entry HTML is 10,507 bytes raw / 3,412 gzip, down 166 raw bytes from M26.
- Game JS is 34,088 raw / 12,023 gzip, down 91 raw bytes after removing the
  timeout. CSS is 30,194 raw / 7,260 gzip, down 485 raw bytes after removing the
  dead screen, spinner, animation, and reduced-motion exception.
- Fingerprinted audio remains
  `special-spotlight-db0e06528b9a.mp3`, 3,079,985 bytes raw.
- Built and source attribution documents retain identical SHA-256
  `f04d45438657db9b3f118dfddcbc3ed2ba9c2c511f4db92f7629cfc2e75d0b3b`.
- Explicit IPv4 HTTP returned `text/html`, `application/wasm`,
  `text/javascript`, `text/css`, `audio/mpeg`, and `text/markdown` for the final
  entry, WASM, game script, stylesheet, MP3, and attribution document.

## Persona review

- **Readability/accessibility:** one H1/status owner remains at first paint;
  opacity and pointer-events no longer masquerade as semantic hiding.
- **Performance:** one timer and 742 raw shipped bytes across HTML/JS/CSS were
  removed; WASM and all assets are byte-identical.
- **Safety/privacy:** no permission, storage, camera, pose, evaluation, or
  network boundary changed.
- **Runtime regression:** production evidence proves the launcher still appears,
  canvas dimensions/inertness remain valid, WASM loads, and keyboard recovery
  still crosses launcher/guide correctly.

## Residual attended QA

Camera permission, physical audio playback, and a target screen reader were not
started. A real-device session must still verify camera denial/recovery,
announcement timing, safe-area readability, low-brightness visibility, and mix.
