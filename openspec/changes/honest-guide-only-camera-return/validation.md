# Validation

## Automated evidence

- `npm run typecheck` — passed.
- `npm test -- --run` — 45/45 web tests passed across 10 files.
- `npm run build` — release WASM and Vite production build passed.
- Output artifacts: `brainbreak-game-dc50488823cc.wasm`,
  `game-BzC2HFXs.js`, `vision-D4tT78cq.js`, and `style-LSFdbqrU.css`.
- `cargo fmt --all -- --check` and `git diff --check` — passed.
- No JavaScript/WASM imports changed; bridge ABI remains v5.

## Served-browser evidence

The release build was served over local HTTP and inspected through the in-app
browser.

- Launcher fallback is now `Xem demo không camera` with a 171.8×48 px target on
  390×844.
- Demo entry resolved Random to Mirror once and rendered
  `/assets/game_mode_mirror.jpg` with title `Mirror Beat`.
- Visible safety copy states `không điểm • không va chạm • không multiplayer`;
  the older floating warning is hidden while the dialog owns presentation.
- 390×844: card 366×541.9 px, image 328×226.2 px, camera CTA 328×60 px,
  back action 328×48 px; no page overflow.
- 844×390: card 760×277 px, image 371.6×220 px, camera CTA 334.4×60 px,
  back action 334.4×48 px; no page overflow.
- 1440×784: card 760×341.7 px, image 351.7×242.6 px, camera CTA
  318.3×60 px, back action 318.3×48 px; no page overflow.
- Initial focus lands on camera; Tab moves to back, wraps to camera, and
  Shift+Tab wraps back. Escape returns to the concrete Mirror card with focus.
- Back hides the demo, clears guide-only state, restores the launcher, and keeps
  Mirror selected.
- Camera hides the demo, clears both `guide-only` and `guide-demo-open`, retains
  Mirror, and reaches the existing permission state:
  `Đang chờ quyền camera` / `Chọn Cho phép camera để tiếp tục`.
- A final cold tab on the release origin produced no app-origin warning/error.
  Closing a separate permission-pending Macroquad tab still reproduced the known
  teardown-only `glBindTexture` error.

## Persona review

- Security: no new external input, persistence, network, or evaluation path;
  guide-only score/collision/multiplayer denial remains Rust-enforced.
- Performance: the dialog reuses an already shipped mode image and adds no asset
  request or render-loop work.
- Readability: demo state and recovery are browser-owned; Rust/bridge ABI stay
  untouched. Shared return behavior is centralized in `returnFromGuideDemo`.
- Runtime regression: camera and back branches, focus wrap, three responsive
  viewports, cold console, TypeScript, release WASM, and full build were checked.

## Existing external residual

`bash refenrece/skills/brainbreak-motion-games/scripts/validate.sh brainbreak`
passed 27 Rust core tests, 3 layout tests, release WASM, strict TypeScript,
45 web tests, and the Vite build, then exited at the unchanged transitive
Sharp/Miniflare high-severity audit advisory. A force downgrade/override remains
unsafe until Cloudflare releases its compatible Sharp 0.35 update.

Physical camera grant, pose alignment, audio interaction, and two-person behavior
were not exercised by this browser-only fallback milestone.
