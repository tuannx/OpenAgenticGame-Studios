# Validation

## Pre-change evidence

- A same-origin synthetic camera stream reused canonical
  `phone_placement_guide.jpg`; production MoveNet detected its illustrated body
  and enabled the real camera-evaluated Mirror Beat path.
- At 390x844, the centered P1 card covered the avatar's hips, both legs, foot
  identity panel, and the nearest track line.
- At 1280x720, the same card covered the runner torso/legs despite the larger
  viewport. The overlap is caused by shared bottom-center ownership, not a
  portrait-only CSS issue.
- Source geometry confirmed the HUD is positioned from the viewport bottom while
  the runner stage also consumes the full viewport and anchors the player near
  91.7 percent of its height.

## Final evidence

- `RunningSurfaceLayout` now derives one stage, passive deck, and existing player
  card layout from viewport size plus displayed-player count. Only `Running`
  receives the reduced stage; all other phases retain the full canvas.
- The deck is painted after `RunnerStage`, masking world/pose overflow, then the
  unchanged player cards are painted inside it. No deterministic rule, camera
  state, browser import, touch action, copy, asset, or allocation was added.
- The focused Rust geometry test covers 390x844, 667x375, and 1440x784 with one
  through four cards. It proves a complete viewport partition, an 18px
  stage-to-HUD gap, card containment, and at least 60 percent stage height.
- The HTTP-served production path was repeated with production MoveNet and a
  synthetic camera stream. At 390x844, 667x375, and 1280x720, the camera-backed
  avatar, foot P1 identity, and nearest lane remained above the neon boundary
  while the P1 score/life card stayed fully inside the lower deck. Short
  landscape retained a clear majority-stage hierarchy rather than collapsing
  gameplay into a narrow strip.
- Browser accessibility snapshots continued to report one camera-evaluated
  player. App-origin console contained only the known Macroquad unused-plugin
  messages; remaining warnings came from the Chrome extension origin.
- `bash refenrece/skills/brainbreak-motion-games/scripts/validate.sh brainbreak`
  passed formatting, strict Clippy, 41 core tests, 25 game/layout tests, release
  WASM, TypeScript typecheck, 72 web tests, and the Vite production build. It then
  stopped at the inherited `wrangler -> miniflare -> sharp` audit finding (3
  high); the offered forced fix would install incompatible Wrangler 4.15.2 and
  was not applied.
- Release artifact `brainbreak-game-78fbe2afc3c0.wasm` is 962,127 bytes with
  SHA-256 `78fbe2afc3c0ad005fdcc69edf11d67651598f9add7a00d505997ad133226059`.
  `mq_js_bundle.js` passed `node --check` and strict-loader grep.
- HTTP probes returned 200 with the expected HTML, WASM, JavaScript, CSS, audio,
  and Markdown MIME types. Attribution source/dist are byte-identical, the QA
  harness is absent from `dist`, and `git diff --check` passes.
- No ABI changed; TypeScript plugin and Rust bridge crate versions remain aligned
  at 7.

## Residuals

- At 390x844 the camera PIP still shares the upper-right region with the action
  beacon. That is the strongest next screen-hierarchy candidate and was not
  hidden inside this lower-HUD milestone.
- Synthetic input does not prove physical camera-distance readability, real
  multi-person entry/crossing/departure, two-peer behavior, or production deploy.
