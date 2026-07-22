# Validation

## Automated evidence

Passed on the final tree on 2026-07-21:

- Strict TypeScript typecheck and 54 Vitest tests across 11 files.
- Strict native Rust checks, 41 deterministic core tests, and 22 game/layout
  tests remain green.
- Release `wasm32-unknown-unknown`, production Vite build, Macroquad loader
  syntax/patch checks, and `git diff --check` passed.

The complete BrainBreak validator passed Rust/native/WASM, TypeScript, all tests,
and the production build, then stopped at the unchanged dependency audit. The
`wrangler -> miniflare -> sharp` chain reports three inherited high findings;
the offered forced fix installs an incompatible Wrangler version and was not
applied in this presentation milestone.

## Runtime-truth and ownership review

The native `details` and `summary` remain the only disclosure state owner. The
closed summary now carries one decorative CSS sliders shape in a 48px target;
its plain `Party & settings` label remains semantically present but becomes
visible only while open. The panel then expands to a readable 230px instead of
remaining a 150px rail in short landscape.

All camera, audio, overlay, opacity, theme, reduced-motion, room, and Studio
elements retain their existing identifiers, order, and event owners. Guide-only
still hides the entire control panel while its modal owns the viewport. No
JavaScript, asset, browser import, camera, scoring, multiplayer, persistence,
Rust, or ABI behavior changed.

## Release and HTTP evidence

- WASM remains byte-identical to M21:
  `brainbreak-game-425aa5274bb6.wasm`, 954,575 bytes raw / 313,033 gzip.
- Final entry HTML is 10,554 bytes raw / 3,429 gzip; game JS is 33,887 raw /
  11,863 gzip; CSS is 30,424 raw / 7,298 gzip.
- The selected canonical illustration remains 162,375 bytes raw / 159,243 gzip;
  no asset or request was added.
- Explicit IPv4 HTTP returned `text/html`, `application/wasm`,
  `text/javascript`, `text/css`, and `image/jpeg` for the final entry, WASM,
  game script, stylesheet, and canonical illustration.

## Browser and responsive evidence

Production CSS was served over explicit IPv4 HTTP and exercised at 390x844 and
667x375:

- The before fixture showed the closed technical title wrapping across a
  145-150px card in both target orientations.
- Final computed closed panel and summary geometry was exactly 48x48 in both
  viewports. The label was visually clipped while remaining in the semantic
  snapshot.
- Portrait open geometry was a 230x602 panel with a 230x48 summary and the full
  control set visible. Short-landscape open geometry was 230x311 with bounded
  vertical scrolling rather than an unreadable narrow rail.
- Pointer activation opened the panel; Enter closed it; Space reopened it.
  Sequential keyboard focus reached `SUMMARY` with a solid 3px cyan outline,
  and Enter opened it from that focus state.
- The final semantic snapshot retained `Party & settings` in every state and
  exposed the complete control set only in the open frames.
- Application-origin warning/error filtering returned empty. Browser-extension
  liveness warnings were unrelated and excluded by URL origin.

Temporary QA fixtures were removed after evidence capture.

## Persona review

- **Security/privacy:** camera permission, local pose handling, network, and
  evaluation boundaries did not change. Camera stop/start remains available.
- **Performance:** one small DOM span pair and CSS geometry replace prose in the
  collapsed visual state; no asset, listener, state machine, or frame work was
  added.
- **Readability/accessibility:** optional technical prose leaves the gameplay
  hierarchy while its accessible label, 48px target, native keyboard activation,
  focus outline, reduced-motion control, and complete expanded content remain.
- **Runtime regression:** all control identifiers and owners are unchanged;
  launcher, Ready, guide-only, taste, gameplay, and ABI boundaries remain intact.

## Residual attended QA

Camera permission was not accepted. A physical session must still verify target-
device safe areas, screen-reader announcement, far-distance discoverability of
the sliders shape, and child/adult readability. Production-CSS and keyboard
evidence do not prove those attended behaviors.
