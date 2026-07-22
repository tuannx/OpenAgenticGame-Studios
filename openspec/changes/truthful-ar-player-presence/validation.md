# Validation

## Pre-change evidence

- A same-origin synthetic camera stream used the canonical phone-placement art;
  production MoveNet detected its illustrated full-body person and enabled the
  real camera-evaluated Mirror Beat path.
- At 1280x720 the camera/status and HUD truthfully reported one player/P1, while
  the stage drew a camera-backed cyan P1 plus an unevaluated purple robot P2.
- At 390x844 the false P2 occupied the same center lane and visibly overlaid P1,
  increasing ambiguity at the most constrained AR viewport.
- Source inspection confirmed `RunnerStage::draw` intentionally kept both local
  indices 0 and 1 visible even when P2 was not evaluated; the HUD independently
  rendered only the evaluated count.

## Final evidence

- `stage_player_visibility` now gives single modes only their P1 anchor, reserves
  P1/P2 for Duo Groove, and reveals other slots only when their deterministic
  runner evaluation is true. Two focused Rust tests cover single, duo, and
  evaluated remote presence.
- The HTTP-served production path was repeated with production MoveNet and the
  same synthetic camera stream. At 1280x720 and 390x844, status/HUD/stage agreed
  on one camera-evaluated player and the false purple P2 was absent.
- The portrait run completed through the no-touch game loop and result screen
  with only the cyan camera-backed P1 present. The remaining P1 HUD/avatar
  overlap is recorded as a separate spatial-hierarchy follow-up, not hidden by
  this presence change.
- `bash refenrece/skills/brainbreak-motion-games/scripts/validate.sh brainbreak`
  passed formatting, Clippy, 41 core tests, 24 game/layout tests, the release
  WASM build, TypeScript typecheck, 72 web tests, and the production Vite build.
  The script then stopped at the inherited `wrangler -> miniflare -> sharp`
  audit finding (3 high); the offered forced fix would install incompatible
  Wrangler 4.15.2 and was intentionally not applied.
- Release artifact `brainbreak-game-816af0a12dba.wasm` is 961,340 bytes with
  SHA-256 `816af0a12dbad5ad16007bfad734d33cf8152e27050761f610b5d3846ec373c0`.
  `mq_js_bundle.js` passed `node --check` and the strict-loader grep.
- HTTP probes returned 200 with `text/html`, `application/wasm`,
  `text/javascript`, `text/css`, `audio/mpeg`, and `text/markdown` for the
  expected HTML, WASM, JavaScript, CSS, music, and attribution files.
- Music attribution source and distribution copies are byte-identical. The
  temporary camera harness is absent from `dist`, and `git diff --check` passes.
- No browser import or ABI changed; TypeScript bridge version and Rust bridge
  crate version remain aligned at 7. Browser console contained only known
  Macroquad unused-plugin logs plus extension-origin warnings, with no app error.

## Attended gaps

- A synthetic detected body does not prove real two-person entry order,
  identity crossing, departure, or camera-distance readability.
- No P2P/two-peer or deployed-production behavior changed or was claimed.
