# Validation

## Automated evidence

Passed on the final tree on 2026-07-21:

- Strict TypeScript typecheck and 54 Vitest tests across 11 files.
- Strict workspace Clippy, 41 deterministic core tests, and 22 game/layout
  tests remain green.
- Release `wasm32-unknown-unknown`, production Vite build, formatting, and
  `git diff --check` passed.

The complete BrainBreak validator passed Rust/native/WASM, TypeScript, all tests,
and the production build, then stopped at the unchanged dependency audit. The
`wrangler -> miniflare -> sharp` chain reports three inherited high findings;
the offered forced fix installs an incompatible Wrangler version and was not
applied in this presentation milestone.

## Runtime-truth and hierarchy review

The existing `body.guide-demo-open` state now owns all guide-dialog visibility.
While it is active, the control panel, camera card, music credit, and guide
warning are absent; the selected canonical illustration, plain mode title,
non-evaluation truth, camera recovery, and mode return are the only dialog
content. Compact portrait centers the card instead of exposing a second focal
region above it. Short landscape retains its side-by-side art/action layout.

The three mode titles are now plain presentation strings. `modeLabel` uppercases
those strings directly instead of relying on a trailing-emoji removal regex.
Camera and return buttons expose plain semantic labels while fixed CSS camera
and chevron shapes remain stable through the camera button's async status text.

Guide-only evaluation, selected-mode resolution, camera/audio gesture gating,
focus handling, Escape navigation, taste persistence, Rust gameplay, browser
imports, and ABI are unchanged. No asset or network request was added.

## Release and HTTP evidence

- WASM remains byte-identical to M20:
  `brainbreak-game-425aa5274bb6.wasm`, 954,575 bytes raw / 313,033 gzip.
- Final entry HTML is 10,427 bytes raw / 3,411 gzip; game JS is 33,887 raw /
  11,864 gzip; CSS is 29,552 raw / 7,161 gzip.
- The deferred vision chunk remains 1,348,440 bytes raw / 341,483 gzip; no
  camera model or inference code changed.
- Explicit IPv4 HTTP returned `text/html`, `application/wasm`,
  `text/javascript`, `text/css`, and `image/jpeg` for the final entry, WASM,
  game script, stylesheet, and selected canonical illustration.

## Browser and responsive evidence

The production bundle was served over explicit IPv4 HTTP and exercised at
390x844 and 667x375:

- The initial portrait audit showed Party & settings, inactive camera status,
  and gameplay chrome visibly competing above the bottom-aligned guide card.
- Final isolated portrait and short-landscape runs showed the full canonical
  image, plain mode title, limitation truth, 60px camera recovery, and 48px mode
  return with no technical chrome, clipping, scrollbar, or overlap.
- Semantic snapshots contained both plain button labels and no Party & settings
  or inactive-camera text while the dialog was active.
- Tab moved camera -> back -> camera, proving the two-action focus loop; Escape
  returned to the BRAINBREAK PARTY launcher.
- Application-origin console filtering returned no warning or error.

A dual-WebGL side-by-side screenshot again produced compositor clipping after a
reload; isolated viewport reruns rendered correctly. Temporary QA fixtures were
removed after evidence capture.

## Persona review

- **Security/privacy:** camera permission, pose handling, persistence, network,
  and runtime evaluation boundaries did not change. Honest no-score/no-collision/
  no-multiplayer copy remains visible.
- **Performance:** no asset, listener, state machine, or frame-loop work was
  added. The change is bounded CSS/DOM presentation and shorter title strings.
- **Readability/accessibility:** one dialog state owns visibility; fixed shapes
  are independent of emoji fonts; semantic labels, 60/48px targets, focus loop,
  and Escape recovery remain intact.
- **Runtime regression:** selected art, mode resolution, camera return, launcher
  return, guide-only non-evaluation, and ABI retain their existing owners.

## Residual attended QA

Camera permission was not accepted. A physical session must still verify the
camera recovery button through permission/model startup, screen-reader dialog
announcement on target devices, safe-area behavior, and child/adult readability.
Layout and keyboard evidence do not prove those behaviors.
