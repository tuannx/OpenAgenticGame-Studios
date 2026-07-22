# Validation

## Pre-change evidence

- A same-origin production harness replaced only `getUserMedia()` with a
  deterministic `NotAllowedError`; the real lazy vision module, catch path,
  launcher, CSS, and Macroquad bundle remained unchanged.
- Both 390x844 and 667x375 returned safely to the launcher with camera evaluation
  off, camera CTA enabled, and the truthful guide action still visible.
- The alert exposed only `Không thể bật camera`. It did not distinguish denied
  permission from missing/busy hardware or name the already-visible next paths.
- The generic line was visually easy to miss between taste/privacy copy and the
  guide fallback. Programmatic guide-originated activation supplied no focus
  evidence for the retry action.

## Iteration evidence

- The first cause-aware rail rendered cleanly at 390x844, but the longest busy
  message left only the icon edge of the guide action inside the 667x375 fold.
  The short-landscape pass therefore tightened rail padding and no-touch line
  height while retaining the 48px guide target and both truth statements.
- The next geometry measurement put the fallback bottom at 370.05px inside a
  371px iframe—technically contained but without safe-area tolerance. Header/grid
  spacing and non-interactive rail height were reduced further; action target
  sizes and truth copy remained unchanged.
- The final longest-message measurement put the fallback bottom at 358.05px in
  the 371px short-landscape frame, restoring about 13px of lower tolerance. The
  portrait fallback ended at 771.23px inside its 840px frame.

## Automated evidence

Passed on the final tree on 2026-07-21:

- `vision-status.test.ts` now covers all six browser error names, the generic
  fallback, and redaction of private adapter detail. Strict TypeScript and all
  66 Vitest browser-logic tests passed.
- Strict Rust formatting/Clippy, 41 deterministic core tests, and 22 game/layout
  tests passed.
- Release `wasm32-unknown-unknown`, production Vite build, Macroquad loader
  syntax/strict-mode repair checks, absence of the temporary QA page, and
  `git diff --check` passed.

The complete validator stopped only at the unchanged dependency audit. The
inherited `wrangler -> miniflare -> sharp` chain reports three high findings;
the offered forced fix installs incompatible Wrangler 4.15.2 and was not
applied.

## Browser and responsive evidence

The HTTP-served production bundle was exercised with a same-origin QA harness
that replaced only iframe `getUserMedia()` with deterministic DOMExceptions:

- `NotAllowedError`, `NotReadableError`, and `NotFoundError` each made exactly
  one camera call, returned to the launcher, left no stream, and displayed the
  exact permission, busy, or missing-camera recovery message.
- Launcher-originated and guide-originated failures both focused the existing
  camera retry CTA on the next animation frame. Retrying from the guide raised
  the call count from one to two and returned to the same stable launcher state.
- After failure, Tab moved camera -> guide and Shift+Tab moved guide -> camera.
  The guide/no-score preview remained available and opened its existing dialog.
- Final 390x844 and 667x375 frames kept the complete recovery rail and both
  actions visible. The 52px camera and 48px fallback targets were not reduced.
- Application-origin warning/error filtering was empty, and the temporary
  same-origin harness was removed after QA.

The failure branch still calls `stopCamera()` and explicitly disables camera
evaluation before changing presentation. The harness proves rejected-start
recovery with no MediaStream; it does not substitute for an operating-system
permission prompt test.

## Capability and resource evidence

The change adds one pure error presenter and consumes it at the existing camera
startup boundary. Camera constraints, detector lifecycle, audio, scoring,
collision, multiplayer, Rust, bridge imports, ABI, storage, and assets are
unchanged. Adapter exception messages never reach the DOM.

## Release and HTTP evidence

- WASM remains `brainbreak-game-425aa5274bb6.wasm`, 954,575 bytes, with SHA-256
  `425aa5274bb6c9a3766e5e384505d97e65d3817ea163cae9731f218f562ce0f5`.
- Final entry HTML is 10,507 bytes; game JS is 34,792 bytes; CSS is 30,978
  bytes. Fingerprinted audio remains
  `special-spotlight-db0e06528b9a.mp3`, 3,079,985 bytes.
- Built and source attribution documents remain byte-identical with SHA-256
  `f04d45438657db9b3f118dfddcbc3ed2ba9c2c511f4db92f7629cfc2e75d0b3b`.
- Explicit IPv4 HTTP returned `text/html`, `application/wasm`,
  `text/javascript`, `text/css`, `audio/mpeg`, and `text/markdown` for the final
  entry, game WASM, game script, stylesheet, MP3, and attribution document.

## Persona review

- **Family clarity:** copy distinguishes permission, absent hardware, busy
  hardware, and unknown startup failure without exposing technical details.
- **Low/no touch:** no action was added; the dominant retry and honest no-score
  preview remain the two existing choices.
- **Readability/accessibility:** recovery uses a shape-first rail, preserves both
  action target sizes, and restores focus to a visible retry target.
- **Safety/privacy:** camera is still requested only after intent; every startup
  failure fails closed and leaves evaluation disabled.

## Residual attended QA

A physical camera permission grant/denial, real-device safe areas, target screen
reader announcement timing, low-brightness visibility, and audible mix were not
verified. Those checks require an attended browser/device session; deterministic
DOMException QA is not presented as operating-system permission proof.
