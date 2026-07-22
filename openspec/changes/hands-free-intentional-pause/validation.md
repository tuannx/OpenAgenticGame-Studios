# Validation

## Automated evidence

- `cargo test -p brainbreak-core`: 37 passed. Coverage includes the one-second
  dwell threshold, transient and overhead-clap rejection, tracking-loss reset,
  control-channel isolation throughout the dwell, pre-progression freeze, score
  isolation, hazard clearing, single/Duo clap readiness, frozen countdown, and
  loss destinations.
- `cargo test -p brainbreak-game`: 12 passed, including pause-panel containment
  at 390x844, 844x390, and 1440x784.
- `cargo fmt --all -- --check` and strict workspace Clippy passed.
- The full BrainBreak validator passed native Rust, release WASM, TypeScript,
  47 Vitest tests, and the production web bundle before stopping at the
  dependency-audit gate described below.
- Release artifact: `brainbreak-game-e27cf3d8a45f.wasm` (659,434 bytes).
  `wasm-objdump` confirms `brainbreak_bridge_crate_version()` still returns 6.
  Pause reuses the existing `u32` action mask and adds no browser import, so no
  bridge-version bump is required.

## HTTP/browser evidence

- The production build loaded in Chrome from `http://127.0.0.1:4199/`; all
  requested JavaScript, image, Macroquad bridge, and WASM resources returned
  200. After self-review tightened dwell isolation, the final rebuilt index,
  game loader, and WASM artifact were served again over HTTP and returned 200.
- No application-origin console error or warning was observed. The only browser
  warnings came from an unrelated installed extension.
- At 390x844, the setup card measured 366x738 at `(12, 53)` and stayed fully in
  the viewport. At 844x390, the 820x366 card and primary camera action stayed in
  view with no document overflow; 59 px of internal card scroll contains the
  secondary demo action. At 1440x784, the 980x694 card stayed fully visible.
- The full-window game canvas matched every tested viewport with no document
  overflow.
- Final HTTP content types were `application/wasm` for the release artifact and
  `text/javascript` for `game-DZkx6HKv.js`; the browser loader ABI remains 6.

## Persona and taste review

- A first-time player sees one dominant camera action plus the complete
  no-touch contract before permission: lean to choose, raise a hand to play,
  and hold both hands high to pause.
- The pause presentation uses the universally recognizable two-bar shape, one
  short resume instruction, and no touch affordance competing with camera play.
- The deliberate one-second hold lives in a control-only action channel, so an
  accidental control gesture cannot alter score, life, miss, or combo state.

## Dependency audit attribution

`npm audit` reports three high-severity inherited advisories in the development
tool chain `wrangler -> miniflare -> sharp`. The offered `npm audit fix --force`
would install `wrangler@4.15.2` as a breaking change, so it was not applied as
part of this bounded motion-UX milestone. Runtime and bundle gates completed
before this audit failure.

## Residual attended QA

A permissioned physical camera session must still judge one-second hold comfort,
false positives during jumping, pause-symbol readability at 1.5-2.5 metres,
release-then-clap discoverability, and two-person behavior. This milestone does
not claim those physical observations from automated poses or layout geometry.
