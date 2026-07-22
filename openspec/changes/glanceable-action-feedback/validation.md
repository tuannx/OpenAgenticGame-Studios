# Validation

## Automated evidence

- `cargo test -p brainbreak-game --all-features` — 11/11 game and layout tests
  passed. New coverage proves outcome mapping, 0.72-second bounded lifetime,
  four-marker containment, shape-coded life geometry, and combo visibility from
  x2.
- `cargo clippy -p brainbreak-game --all-targets --all-features -- -D warnings`,
  `cargo fmt --all -- --check`, and `git diff --check` — passed.
- `npm test -- --run src/audio.test.ts src/bridge.test.ts` — 7/7 focused tests
  passed. The pure pitch mapping is one semitone per five combo capped at four,
  music rate is explicitly 1, and bridge forwarding preserves `(kind, combo)`.
- `npm run typecheck` — passed.
- Full native `cargo test --all-targets --all-features` — 27 core and 11
  game/layout tests passed; full strict Clippy passed.
- `bash refenrece/skills/brainbreak-motion-games/scripts/validate.sh brainbreak`
  — native Rust, release WASM, strict TypeScript, 47/47 web tests, and Vite
  production build passed before the unchanged dependency-audit residual.
- Final artifacts: `brainbreak-game-7dbfa256e83f.wasm` and
  `game-iJlpPLpb.js`.

## ABI evidence

- Source plugin and Rust crate versions are synchronized at v6.
- `wasm-objdump -x` reports `bb_play_feedback` with type 8,
  `(i32, i32) -> nil`.
- `wasm-objdump -d` reports `brainbreak_bridge_crate_version` returning
  `i32.const 6`.

## Served-browser evidence

- The final release was cold-loaded from `127.0.0.1:4199`.
- At 390x844, 844x390, and 1440x784 the body and canvas exactly matched the
  viewport. The launcher remained visible and the final `game-iJlpPLpb.js`
  plus local Macroquad loader were active.
- App-origin warning/error count was zero, so the v6 import migration produced
  no missing-import or plugin runtime regression.
- JavaScript returned `Content-Type: text/javascript`; the hashed game WASM
  returned `Content-Type: application/wasm`.
- The browser viewport override was reset, test tab closed, and preview server
  stopped after QA.

## Persona review

- Security: the bridge carries one existing bounded `u16` combo value as `u32`;
  no camera, storage, network, or external-input surface widened.
- Performance: presentation state is a fixed four-pulse array, remains visible
  only 0.72 seconds, and adds no asset request or unbounded render-loop storage.
- Readability: deterministic judgment, bounded presentation, audio synthesis,
  and persistent HUD have explicit separate roles; both bridge versions moved
  together.
- Accessibility: outcome and life state use shape plus copy instead of color or
  sound alone; reduced motion removes scale punch while retaining the signal.
- Runtime regression: native rules, release import signature/version, all web
  tests/build, three viewport classes, MIME types, and app-origin console were
  checked.

## Physical-session residuals

Automated QA deliberately did not inject a fake evaluated pose. A permissioned
camera/audio session is still required to judge outcome readability from
1.5-2.5 metres, SFX/music mix, real haptics, reduced-motion feel, and simultaneous
two-person markers. These are not inferred from layout tests or cold loading.

## Existing dependency residual

The full validator stops at the unchanged transitive Sharp `<0.35.0` advisory
through Miniflare/Wrangler after every code/build gate passes. The suggested
force fix would install a breaking Wrangler version and was not applied.
