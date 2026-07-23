# Tasks — Stacked character overlays

## Progress

- [x] `guide_coach.rs`: shared center stage, 40% alpha, filled character, slow anim
- [x] `juice.rs`: filled cartoon silhouette + pink aura at 40% alpha
- [x] Runner + Supernova: overlap wiring + stage VFX
- [x] Studio beds → 140 BPM rock-dance; core/audio BPM sync
- [x] Validate native + WASM + web tests
- [x] Deploy Cloudflare (existing worker)

## Validation

- `cargo test -p brainbreak-game -p brainbreak-core` → 79 + 36 passed
- `npm run typecheck && npm test` → 118 passed
- `npm run build:wasm` → `brainbreak-game-0b2eb883bbba.wasm`
- `npm run deploy` → https://brainbreak-motion-party.tuannx87.workers.dev
  - `/health` build: `wasm-0b2eb883bbba`
