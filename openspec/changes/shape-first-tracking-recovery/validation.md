# Validation

## Automated evidence

Passed on the final tree on 2026-07-21:

- `cargo fmt --all -- --check`.
- Strict workspace Clippy across all targets and features.
- Workspace tests: 41 core tests and 19 game/layout tests.
- The new recovery test covers no/local/remote evaluation for single mode, all
  required P1/P2 evaluation combinations for Duo, partial/both readiness,
  shape-state truth, static ASCII copy, and exclusion of the intentional-pause
  word `PAUSED`.
- Shared motion-prompt geometry keeps panel, canonical art, content, and both
  player figures inside 390x844, 667x375, and 1440x784.
- `npm run typecheck` and 50 Vitest browser-logic tests.
- Release `wasm32-unknown-unknown` build, production Vite bundle, and
  `git diff --check`.

The complete BrainBreak validator passed Rust/native/WASM, TypeScript, tests,
and production build, then stopped at the unchanged dependency audit. The
`wrangler -> miniflare -> sharp` chain reports three inherited high findings;
`npm audit fix --force` proposes a breaking Wrangler version and was not applied
in this presentation-only milestone.

## Runtime-truth review

Single-mode recovery uses `any evaluated` and `any evaluated-and-ready`, matching
the deterministic core's capacity rule. Duo presentation reads only required
local P1/P2 facts. A focused case proves that evaluated remote P3/P4 players do
not make a Duo recovery screen claim that its two required players are present.

`TrackingHold` no longer routes through the generic `RUN PAUSED` panel. Missing,
present, and ready figures are mapped independently, and all copy is static.
`RunnerGame::update`, tracking thresholds, action masks, countdown duration,
scoring, browser imports, and bridge files are untouched. The remaining generic
status renderer was narrowed to the deterministic `GET READY 2 / 1` countdown.

## Release WASM and HTTP evidence

- Artifact: `brainbreak-game-289d51e8b5fc.wasm`, 952,227 bytes raw and 311,969
  bytes under gzip.
- M15 artifact: 950,876 raw / 311,419 gzip. M16 adds 1,351 raw bytes (+0.14%)
  and 550 gzip bytes (+0.18%) while sharing the existing layout, figure drawer,
  art owner, and decoder.
- `wasm-objdump` confirms `brainbreak_bridge_crate_version()` remains `7`; no
  browser import changed.
- An explicit IPv4 server on `127.0.0.1:58922` returned the expected BrainBreak
  title and `200 application/wasm`, `200 text/javascript`, and `200 image/jpeg`
  for the final WASM, game JS, and canonical Duo art.
- Final server logs show the browser fetched the final JS/WASM, all four launcher
  images, phone guide, and Macroquad loader. The only 404 was the inherited,
  non-game `favicon.ico`; every required runtime asset returned 200.

## Browser evidence

Chrome loaded `Neon Beat Runner • BrainBreak`, created a 5120x2578 backing canvas
for a 2560x1289 CSS canvas, and decoded all inspected mode/setup images at their
intrinsic 640x640 dimensions. No warning or error came from the application
origin.

This proves the final release starts, imports resolve, and shared art still
loads after the renderer refactor. Without accepting camera permission, the
browser cannot create an evaluated `Running` session and then remove/reacquire a
required player, so it does not prove direct TrackingHold composition or timing.

## Persona review

- **Security:** no permission, input, asset, decoder, network, or import boundary
  changed; canonical image decoding retains M14's limits and fallback.
- **Performance:** two presenters return stack-only fixed values and borrow one
  shared renderer. Measured release growth is +0.14% raw.
- **Readability:** sensor recovery no longer impersonates intentional pause;
  missing, present, and ready states use corner frame, neutral body, and raised
  arm plus diamond rather than punctuation or color alone.
- **Runtime regression:** core state transitions and ABI are unchanged; mapping
  tests consume the same evaluation/readiness facts rather than duplicating
  transition authority.

## Residual attended QA

A permissioned physical run must still enter gameplay, lose and reacquire P1,
P2, and both players, then verify the unobscured recovery panel, readiness
latency, and shape/copy recognition at 1.5-2.5 metres. Browser startup and pure
geometry cannot replace that evidence.
