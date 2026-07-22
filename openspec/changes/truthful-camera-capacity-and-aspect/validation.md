# Validation

## Automated evidence

- Focused TypeScript contracts:
  `npm test -- --run web/src/motion-capability.test.ts web/src/motion-navigation.test.ts web/src/bridge.test.ts`
  — 14/14 passed.
- Full web logic: `npm test -- --run` — 45/45 passed across 10 files.
- Strict TypeScript: `npm run typecheck` — passed.
- Native Rust: `cargo test --workspace --all-targets` — 27 core and 3 layout
  tests passed.
- Strict Rust: `cargo clippy --workspace --all-targets -- -D warnings` and
  `cargo fmt --all -- --check` — passed.
- Release browser artifact: `npm run build` — passed; outputs include
  `brainbreak-game-dc50488823cc.wasm`, `game-QmV8QnKX.js`,
  `vision-C8orH6DF.js`, and `style-DeL_v_pf.css`.
- Bridge ABI remains version 5: the change adds no Rust/WASM import.

## Served-browser evidence

Vite preview was served at a dedicated local HTTP origin and inspected through
the in-app browser.

- 390×844 launcher: 390 px document width, no horizontal/vertical overflow;
  setup card 366×719.7 px inside the viewport.
- 390×844 permission setup: camera 343.2×257.4 px and ready card 366×372 px,
  no overlap or horizontal overflow.
- 844×390 permission setup: camera 337.6×253.2 px and ready card 422×225.5 px,
  no overlap or page overflow.
- 1440×784 launcher: 980×653.3 px setup card inside the viewport with all four
  illustrated mode cards aligned and no page overflow.
- Keyboard ArrowRight moved the selected/focusable radio from Random to Mirror.
- Camera intent changed the truthful live status to `Đang chờ quyền camera` and
  the ready instruction to `Chọn Cho phép camera để tiếp tục`.
- Gate schema was 5; desktop-UA capability was 2 even at 390 px, confirming that
  viewport width is not treated as device capability. Mobile/iPadOS capacity 1
  is covered by pure user-agent/touch-hint tests.
- Both video and overlay computed `object-fit: contain`; pre-permission fallback
  remained 4:3. Intrinsic portrait geometry is covered by the pure geometry
  contract, not claimed as physical-camera proof.
- A cold active tab produced no new app-origin warning/error during the steady
  observation window. Closing a Macroquad tab still emits the previously known
  `glBindTexture called with an already deleted texture` teardown error.

## Dependency-audit residual

`bash refenrece/skills/brainbreak-motion-games/scripts/validate.sh brainbreak`
passed Rust/native/WASM, TypeScript, 45 web tests, and the production build, then
stopped at `npm audit --audit-level=high`:

- dev-only `wrangler@4.112.0` brings `miniflare@4.20260714.0` and
  `sharp@0.34.5`; the advisory requires Sharp 0.35+;
- registry-current Wrangler 4.113.0 still publishes Miniflare with Sharp 0.34.5;
- Cloudflare's compatible Sharp 0.35.2 update is still an open PR with one
  Windows check failing: <https://github.com/cloudflare/workers-sdk/pull/14493>;
- Sharp 0.35 includes the updated libvips line:
  <https://sharp.pixelplumbing.com/changelog/v0.35.0/>.

No `npm audit fix --force` downgrade or direct Sharp override was applied because
Miniflare's upstream patch contains code changes required for Sharp 0.35.

## Physical-camera gap

Intrinsic metadata propagation, actual portrait-stream presentation, landmark
alignment, orientation change, distance guidance, camera/audio interaction, and
two-person behavior require an attended real-camera session. Automated fixtures
are not proof of those physical surfaces.
