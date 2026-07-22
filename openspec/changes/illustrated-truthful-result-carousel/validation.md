# Validation

## Automated evidence

Passed on the final tree on 2026-07-21:

- `cargo fmt --all -- --check`.
- Strict workspace Clippy across all targets and features.
- Workspace tests: 41 core tests and 16 game/layout tests.
- The new tests cover all three real 640x640 JPEGs, malformed/empty/oversized
  fallbacks, center-cover geometry, three-card containment, indicator
  containment, and Duo availability.
- `npm run typecheck` and 50 Vitest browser-logic tests.
- Release `wasm32-unknown-unknown` build and production Vite bundle.
- `git diff --check`.

The complete BrainBreak validator passed Rust/native/WASM, TypeScript, tests,
and production build, then stopped at its dependency audit. The unchanged
`wrangler -> miniflare -> sharp` chain reports three inherited high findings;
the offered `npm audit fix --force` would install a breaking Wrangler version,
so it was not applied in this UX milestone.

## Browser-discovered failure and fix

The first HTTP browser run exposed a real WASM panic in Macroquad
`Texture2D::from_file_with_format`: Macroquad 0.4.15 detects JPEG but does not
enable its decoder. Builds and native layout tests did not expose this runtime
failure.

The final implementation explicitly enables only `image`'s JPEG feature and
uses a fallible decode boundary with limits of 512 KiB encoded input, 1024 px per
dimension, and 8 MiB decoder allocation. Fetch, decode, dimension, or allocation
failure now retains a procedural result card instead of aborting the game.

## Release WASM and HTTP evidence

- Artifact: `brainbreak-game-c90e377c9046.wasm`, 948,983 bytes raw and 310,703
  bytes under gzip.
- Previous M13 artifact: 660,409 bytes raw; explicit JPEG decode adds 288,574
  bytes (+43.7%) without embedding any of the three JPG byte streams.
- `wasm-objdump` confirms `brainbreak_bridge_crate_version()` still returns `7`;
  this milestone adds no browser import.
- HTTP returned `200 application/wasm` for WASM and `200 text/javascript` for
  `game-nu5BS9kf.js`.
- Mirror, Strike, and Duo returned `200 image/jpeg` at 162,375, 198,505, and
  194,045 bytes respectively.
- Server logs confirm the final browser requested the final JS, WASM, and all
  three canonical mode images.

## Browser runtime evidence

- A clean tab loaded the final production bundle and a full-size Macroquad
  canvas; all DOM mode images completed at their intrinsic 640x640 dimensions.
- The previous JPEG panic disappeared, proving the final WASM passed texture
  decode and entered its frame loop.
- No application-origin warning or error appeared during the steady observation
  window. Remaining warnings came from an installed browser extension; three
  informational loader messages only noted unused bundled Macroquad plugins.

This pass proves final asset fetch/decode and browser startup. It does not expose
the result phase without a permissioned evaluated camera player, so it does not
claim physical result composition.

## Persona review

- **Security:** canonical same-origin input is still bounded before decode;
  malformed and oversized files fail closed to procedural presentation.
- **Performance:** three textures load once and are borrowed per frame. No
  per-frame path, decode, texture, or collection allocation was introduced. The
  measured +43.7% raw WASM decoder cost is recorded rather than hidden.
- **Readability:** one asset owner, one fallible loader, pure crop/layout helpers,
  and a fixed result-choice array keep roles explicit.
- **Runtime regression:** deterministic lean/clap navigation remains unchanged;
  render availability reads the same P1/P2 evaluation facts, and ABI stays v7.

## Residual attended QA

Without a permissioned physical run that reaches results, image composition,
selection clarity, unavailable-Duo recognition, and lean/clap readability at
1.5-2.5 metres remain unverified rather than inferred from geometry tests.
