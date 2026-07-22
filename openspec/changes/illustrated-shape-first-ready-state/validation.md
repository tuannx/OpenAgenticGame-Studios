# Validation

## Automated evidence

Passed on the final tree on 2026-07-21:

- `cargo fmt --all -- --check`.
- Strict workspace Clippy across all targets and features.
- Workspace tests: 41 core tests and 18 game/layout tests.
- The two new tests cover ten camera/player/readiness truth combinations,
  ASCII/static copy bounds, selected-art crop geometry, and panel/art/copy/figure
  containment at 390x844, 667x375, and 1440x784.
- `npm run typecheck` and 50 Vitest browser-logic tests.
- Release `wasm32-unknown-unknown` build and production Vite bundle.
- `git diff --check`.

The complete BrainBreak validator passed Rust/native/WASM, TypeScript, tests,
and production build, then stopped at its dependency audit. The unchanged
`wrangler -> miniflare -> sharp` chain reports three inherited high findings;
the offered `npm audit fix --force` would install a breaking Wrangler version,
so it was not applied in this UX milestone.

## Runtime-boundary review

`ready_presentation` receives only current mode, guide-only state, tracked count,
and the two readiness flags. All titles and instructions are static slices.
`draw_phase_overlay` delegates only the Ready render branch; `RunnerGame::update`,
gesture recognition, player evaluation, countdown thresholds, and bridge imports
remain unchanged.

The mapping keeps `Present`, `Searching`, and `Ready` distinct. In particular, a
one-player Duo state does not imply P1 is ready: P1 is neutral until its actual
latched flag becomes true, while P2 remains a framed search silhouette.

## Release WASM and HTTP evidence

- Artifact: `brainbreak-game-c9cdc4dabfbe.wasm`, 950,876 bytes raw and 311,419
  bytes under gzip.
- M14 artifact: 948,983 raw / 310,703 gzip. M15 adds 1,893 raw bytes (+0.20%)
  and 716 gzip bytes (+0.23%) while reusing the existing decoder and textures.
- `wasm-objdump` confirms `brainbreak_bridge_crate_version()` still returns `7`;
  this milestone adds no browser import.
- HTTP returned `200 text/html`, `200 application/wasm`, `200 text/javascript`,
  and `200 image/jpeg` for the entrypoint, final WASM, game JS, and canonical art.
- Server logs confirm the final browser fetched the final JS/WASM, all mode art,
  the phone guide, Macroquad bundle, and audio from the final production tree.

## Browser evidence and discovered infrastructure trap

The first local attempt reached an unrelated existing service because that
service held IPv4 port 4173 while Python successfully bound only IPv6 `::4173`.
The browser URL looked correct but the page title was `SkillMesh Node`. The final
run used an explicit high IPv4 bind (`127.0.0.1:58921`) and curl-verified title,
MIME, and artifact length before opening the browser.

On the final URL, Chrome loaded `Neon Beat Runner • BrainBreak`, created a
5120x2466 backing canvas for a 2560x1233 CSS canvas, decoded all inspected mode
images at their intrinsic 640x640 size, and reported no warning or error from the
application origin. A guide-only run visibly showed the selected Duo art and two
procedural player silhouettes in the running canvas.

The DOM guide-demo gate intentionally remains centered above the canvas until a
user either enables the camera or returns to mode selection. Accepting camera
permission was not pre-authorized, so the run does not claim a direct visual read
of the new center Ready panel. A native Macroquad process also ran successfully,
but the bare binary is not registered as a macOS app bundle and could not be
attached by the approved Computer Use surface. These are evidence gaps, not
rendering failures.

## Persona review

- **Security:** no new input, permission, URL, import, or decode boundary; art
  still passes through M14's bounded fallible JPEG owner.
- **Performance:** Ready copy is stack/static, and art is borrowed. The measured
  release delta is +0.20% raw rather than an unverified zero-cost claim.
- **Readability:** short ASCII labels remove platform-dependent emoji; camera,
  framed search, neutral presence, raised arm, and diamond cues remain distinct
  by shape.
- **Runtime regression:** the deterministic core and ABI are unchanged; semantic
  tests prove presentation from facts rather than duplicating state transitions.

## Residual attended QA

A permissioned physical run is still required to inspect the unobscured Ready
panel for Mirror, Strike, and Duo; verify P1/P2 transitions; and judge copy and
raised-hand recognition at 1.5-2.5 metres. Automated geometry and browser startup
do not substitute for that camera-backed evidence.
