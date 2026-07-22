# Design

## Typed browser-boundary presentation

`vision-status.ts` already maps the typed adapter lifecycle to localized copy.
It also owns a pure `unknown -> CameraStartFailurePresentation` boundary for
startup rejections. The mapper reads only a structural string `name` and maps:

- `NotAllowedError` / `SecurityError` -> grant permission, then retry;
- `NotFoundError` / `DevicesNotFoundError` -> no camera, use no-score preview;
- `NotReadableError` / `TrackStartError` -> camera busy, close the other app;
- every other value -> generic retry or preview guidance.

No adapter exception text reaches the DOM. `main.ts` consumes the presentation,
keeps evaluation false through the existing `stopCamera()` boundary, returns to
the launcher, and focuses the primary camera CTA on the next animation frame.

## Recovery hierarchy

The alert becomes a compact rounded status rail with a CSS camera/slash shape.
It is information, not another action. The existing filled camera CTA remains
the retry path and the existing readable guide-only action remains the fallback.
The rail may wrap but must keep both actions above the fold at 390x844 and
667x375.

Short landscape keeps the same 48px guide target while tightening only rail
padding and the no-touch sentence line-height. Privacy and no-touch truth remain
visible; recovery density never comes from hiding either statement.

## Invariants

- Camera/audio still begin only after explicit user intent.
- Camera failure leaves evaluation disabled and no camera stream alive.
- Retry remains possible after every failure; loader/pending state is not stuck.
- Guide-originated and launcher-originated failures converge on one launcher
  recovery state with retry focus.
- No new asset, listener, storage field, Rust code, bridge import, or ABI change.

## Validation package

Add pure mapping tests, typecheck/focused web tests, production build, and a
same-origin browser harness that replaces only `getUserMedia()` with deterministic
DOMExceptions. Verify permission/missing/busy copy, focus, retry count, preview
transition, responsive containment, no evaluation/stream, and browser logs. Close
with native/WASM/web, HTTP MIME, loader, fingerprint, and release-size gates.
