# Tasks — Shared mid-playfield stage

## Progress

- [x] Remove bottom-strip bias from `guide_coach_layout` (was ~0.62h with weak centering / small figures)
- [x] Mid-playfield stage: height **0.68**, slack bias **0.48**, figure scale **0.48·stage**
- [x] Hip-match pink overlays in Runner + Supernova (no extra downward push)
- [x] Regression test: reject legacy bottom-40% geometry
- [x] Validate + deploy Cloudflare

## Validation

- `cargo test -p brainbreak-game guide_coach` → 5 passed
- `npm run build:wasm` → `brainbreak-game-a12629ebd4fc.wasm`
- `npm run deploy` → https://brainbreak-motion-party.tuannx87.workers.dev
  - `/health` build: `wasm-a12629ebd4fc`
