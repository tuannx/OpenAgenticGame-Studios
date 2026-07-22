# Validation

## Automated evidence

Passed on the final tree on 2026-07-21:

- Strict TypeScript typecheck and 54 Vitest tests across 11 files.
- Strict workspace Clippy, 41 deterministic core tests, and 22 game/layout
  tests remain green.
- Release `wasm32-unknown-unknown`, production Vite build, formatting, and
  `git diff --check` passed.

The complete BrainBreak validator passed Rust/native/WASM, TypeScript, all
tests, and the production build, then stopped at the unchanged dependency audit.
The `wrangler -> miniflare -> sharp` chain reports three inherited high findings;
the offered forced fix installs an incompatible Wrangler version and was not
applied in this presentation milestone.

## Runtime-truth and hierarchy review

At short landscape height, the media-query override now wins over the generic
mobile two-column rule. All four canonical mode images occupy one equal-width
row, leaving a dedicated full-width 52px camera CTA followed by the zero-touch
promise, local-only privacy truth, and 48px guide-only escape. The large
placement illustration remains on portrait and desktop but stays omitted in the
existing short-landscape branch.

Launcher emoji were removed from badges, titles, and actions. The canonical
images retain mode identity; fixed CSS camera/eye symbols and the existing
selected check carry action/state. An unavailable mode receives a geometric
lock plus its existing concise reason. No new asset, image copy, carousel,
scroll interaction, analytics signal, or permission path was added.

Mode recommendation, radio navigation, capability filtering, Random resolution,
camera/audio gesture gating, guide-only evaluation, Rust gameplay, browser
imports, and ABI are unchanged.

## Release and HTTP evidence

- WASM remains byte-identical to M19:
  `brainbreak-game-425aa5274bb6.wasm`, 954,575 bytes raw / 313,033 gzip.
- Final entry HTML is 10,441 bytes raw / 3,431 gzip; game JS is 33,930 raw /
  11,911 gzip; CSS is 28,897 raw / 7,098 gzip.
- The deferred vision chunk remains 1,348,440 bytes raw / 341,480 gzip; no
  camera model or inference code changed.
- Explicit IPv4 HTTP returned the BrainBreak HTML plus `application/wasm`,
  `text/javascript`, `text/css`, and `image/jpeg` for final WASM, game script,
  stylesheet, and canonical art.

## Browser and responsive evidence

The production bundle was served over HTTP and exercised through real CSS
iframes at 390x844 and 667x375:

- Portrait retains the two-by-two illustrated grid, placement guide, camera CTA,
  zero-touch/privacy copy, and guide-only escape in the initial viewport.
- The initial short-landscape audit showed the two-by-two grid pushing the
  camera CTA entirely below the fold behind an internal scrollbar.
- The final 667x375 composition shows all four illustrated modes, camera CTA,
  zero-touch/privacy copy, and guide-only escape simultaneously with no visible
  scrollbar or overlap.
- Beat Strike remained selectable as the checked radio in the compact row. The
  guide-only escape opened its truthful illustrated demo with both camera return
  and change-game actions visible.
- A dual-WebGL iframe screenshot produced one transient portrait capture
  artifact; an isolated 390x844 rerun rendered the complete launcher correctly,
  confirming it was capture contention rather than CSS clipping.
- The final launcher runs emitted no warning or error from the application
  origin. Browser-extension warnings were excluded from app evidence.

Temporary responsive fixtures were removed after QA.

## Persona review

- **Security/privacy:** no permission auto-start, pose persistence, network,
  camera, or evaluation boundary changed; local-only truth remains visible.
- **Performance:** no asset was added and no frame-loop work changed. CSS grows
  by roughly 1.6 KB raw versus M19 while WASM and deferred vision bytes remain
  unchanged.
- **Readability/accessibility:** images keep a shared art language; text labels,
  geometric action icons, selected shape, lock shape, radio semantics, keyboard
  selection, and honest fallback remain redundant channels.
- **Runtime regression:** camera gesture gating, mode availability, taste
  recommendation, guide-only non-evaluation, and gameplay are unchanged.

## Residual attended QA

Camera permission was not accepted. A physical phone/tablet session must still
verify first-touch reachability with real safe-area insets, permission and model
handoff, child/adult readability, and rotation while a live video stream owns
the viewport. Permission-state layout does not prove those behaviors.
