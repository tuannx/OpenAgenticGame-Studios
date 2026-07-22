# Validation

## Automated evidence

- `cargo test -p brainbreak-game action_cue_tests --all-features` — 4/4
  focused cue tests passed.
- `cargo clippy -p brainbreak-game --all-targets --all-features -- -D warnings`
  — passed.
- `cargo fmt --all -- --check`, `git diff --check`, and
  `cargo test --all-targets --all-features` — passed; 27 core and 7 game/layout
  tests passed.
- `bash refenrece/skills/brainbreak-motion-games/scripts/validate.sh brainbreak`
  — native Rust, release `wasm32-unknown-unknown`, strict TypeScript, 45/45 web
  tests, and Vite production build passed before the unchanged dependency audit
  residual.
- Release artifacts include `brainbreak-game-c281357ab04a.wasm` and
  `game-hMkaD2-3.js`. No JavaScript/WASM import changed; bridge ABI remains v5.

## Served-browser evidence

- The final release was cold-loaded at `127.0.0.1:4198` after the first preview
  port was correctly rejected as evidence because an unrelated existing service
  worker owned that origin.
- 390x844, 844x390, and 1440x784 each rendered a canvas exactly matching the
  viewport; body width/height matched the viewport with no page overflow.
- The main JavaScript returned `Content-Type: text/javascript`; the hashed game
  WASM returned `Content-Type: application/wasm`.
- The release canvas initialized at all three sizes. App-origin warning/error
  count was zero; the only collected warnings came from an unrelated wallet
  extension.
- The browser viewport override was reset, test tabs were closed, and all
  preview servers were stopped after QA.

## Persona review

- Security: presentation consumes the existing immutable nearest obstacle and
  does not widen camera, input, scoring, collision, storage, or network paths.
- Performance: the old per-frame `format!` allocation is gone; the beacon uses
  static labels and bounded Macroquad primitives with no new asset request.
- Readability: action mapping, responsive layout, proximity, lane semantics,
  and drawing have explicit roles; the three-slot diagram avoids treating an
  obstacle lane as a movement direction.
- Runtime regression: native rules, release WASM, web tests/build, three
  responsive browser sizes, content types, and app-origin console were checked.

## Residual evidence gap

The Running-phase beacon was not visually exercised from a real 1.5-2.5 metre
camera session. Automated QA deliberately did not inject a fake evaluated pose
or weaken guide-only safety to reach Running. A permissioned physical playtest
is still required to tune real-world pictogram scale and telegraph lead time.

## Existing dependency residual

The full validator stops at the unchanged transitive Sharp `<0.35.0` advisory
through Miniflare/Wrangler after every code/build gate passes. `npm audit
fix --force` would install a breaking Wrangler version and was not applied.
