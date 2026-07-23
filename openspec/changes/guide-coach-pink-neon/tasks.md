# Tasks — Guide coach + pink neon user skeleton

## Progress

- [x] `guide_coach.rs`: bottom 40% band, 50% frame alpha, animated poses
- [x] VO cue id → `GuidePose` map (+ dance verb cycle)
- [x] Pink neon silhouette helpers in `juice.rs` (alpha 0.50, glow mul 1.55)
- [x] Supernova: pink overlay in upper playfield + bottom coach by phase/VO
- [x] BrainBreak: pink overlay + coach from next hazard (jump/clap/squat/dodge)
- [x] Validate native + WASM + web tests
- [x] Deploy Cloudflare (existing worker)

## Validation

- `cargo test -p brainbreak-game` → 35 passed
- `npm run typecheck && npm test` → 118 passed
- `npm run build:wasm` → `brainbreak-game-89e6212f8fb9.wasm`
- `npm run deploy` → https://brainbreak-motion-party.tuannx87.workers.dev
  - `/health` build: `wasm-89e6212f8fb9`
