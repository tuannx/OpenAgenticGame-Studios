# Tasks

## Progress Snapshot

- Inventory + SCAMPER + scoring: `scamper-scoring.md`
- Sensory ritual zoom scoring: `sensory-ritual-zoom.md`
- M1 shell done; M2 taste decisions auto-resolved by score; M3 hard-delete deferred
- M4 sensory zoom (this slice): Duo invite + Drop tension/juice + hit SFX + copy

## Milestone Execution Loop

1. read the active change and relevant `.codex/core` docs
2. confirm the milestone still fits the sizing rule
3. implement one bounded milestone only
4. select the validation package from `validation-matrix.md`
5. run commands, inspect first failure, fix, and repeat
6. self-review against the active change and spec
7. apply persona review when required
8. update ADR content when durable decisions change
9. record what ran, what passed, and what remains unverified

## M1. Two-card product shell (reversible)

- [x] M1.1 Write BMM change record (README / proposal / design / tasks)
- [x] M1.2 Collapse launcher HTML to BrainBreak + AR cards
- [x] M1.3 Route launcher / deep-link / ready lean by product family
- [x] M1.4 Demote Studio quick-link from hero composition
- [x] M1.5 Update shell entry tests + deep-link tests; run web typecheck/tests

## M2. Taste polish (auto-decided via SCAMPER scores)

- [x] M2.1 Shell AR card branded **Supernova** (badge AR); Ready keeps Supernova Drop
- [x] M2.2 Hide MY GAMES from shell; Studio + `?game=` remain
- [x] M2.3 Ready lean stays Mirror/Strike/Duo; hide lean chevrons when family has ≤1 mode
- [x] M2.4 Soft-deprecate Random art (no hard-delete); keep NEXT-10 as roadmap

## M3. Destructive archive (deferred — hard-delete score 59 < 85)

- [ ] M3.1 Optional later: remove unreferenced `game_mode_random.*` when taste gate still clear
- [ ] M3.2 Do not delete NEXT-10.md (roadmap IP)

## M4. Sensory ritual zoom (scored in `sensory-ritual-zoom.md`)

- [x] M4.1 Score 3 scenarios → 2 shell games; defer Liquid + Garden
- [x] M4.2 Duo auto-invite when 2 bodies in BrainBreak Ready (lean-away respected)
- [x] M4.3 Supernova Drop imminent tremble + bigger flash/bass/haptic; on-beat hit SFX
- [x] M4.4 Shell/Ready copy → ritual arc language; update GAME-DESIGN
- [x] M4.8 Pull-to-Drop body metaphor (squat / hands-down) once `drop_imminent` + pre-Drop bass swell (AnalyserNode lowshelf + synth)
- [x] M4.9 BrainBreak downbeat sensory sync — kick micro-shake, road/sun wash, louder BeatPickup SFX, beat-rail pulse
- [x] M4.10 WASM build hygiene — `build-wasm.sh` copies from `CARGO_TARGET_DIR` so deploy ships the rebuilt artifact
- [x] M4.11 Ready-gate ritual pacing / copy polish — FRAME·RAISE arc, hold breathe juice (live with M4.14 deploy `wasm-ab00fd93ba72`)
- [x] M4.14 Fix WASM panic — remove Runner `BackdropCache` mid-frame `render_target` (miniquad RefCell re-entry at wasm.rs:34)
- [x] M4.12 Afterglow / share moment light polish — Runner GLOW/AFTERGLOW + SHARE THE GLOW; Supernova SHARE THE DROP + chime/haptic
- [ ] M4.5 Liquid Co-op engine (deferred — Cost gate)
- [ ] M4.6 Neon Growth Garden (deferred — Cost gate)
- [ ] M4.7 Infra proposals needing confirm: AudioWorklet, WebGPU particles, MediaPipe Hands
- [x] M4.13 Supernova/Runner sensory tighten (flash/particles/haptic) — shipped as WASM-safe shared juice (`openspec/changes/wasm-safe-juicy-feedback/`); lightmap/bloom still gated

## Deploy Log

| Slice | What | Live URL | Evidence |
|---|---|---|---|
| M4.1–4.4 | Duo invite + Drop juice + copy | https://brainbreak-motion-party.tuannx87.workers.dev | web 105 tests; core supernova 16 |
| M4.8 | Pull-to-Drop + bass swell | https://brainbreak-motion-party.tuannx87.workers.dev | core 18 supernova + hands-down; web 107 tests |
| M4.9 | BrainBreak downbeat sync tighten | https://brainbreak-motion-party.tuannx87.workers.dev | game 25 tests; web 107 tests |
| M4.10 | WASM deploy hygiene (`CARGO_TARGET_DIR`) | https://brainbreak-motion-party.tuannx87.workers.dev | build `wasm-f942bbfd6d0a`; Version `68020ebb-8475-45bf-afdd-ca85034057a9`; `/health` ok; fingerprinted wasm uploaded |
| M4.11+14 | Ready copy + WASM panic fix | https://brainbreak-motion-party.tuannx87.workers.dev | build `wasm-ab00fd93ba72`; Version `7e8a00c0-1734-4f6c-b8fd-c4c6a3599e0d` |
| M4.12 | Afterglow / share moment | https://brainbreak-motion-party.tuannx87.workers.dev | build `wasm-0222bdd12eed`; Version `e9b0e168-3fac-495c-9361-1f3bf392779c`; game 25; web 107 |

## Closeout Rule

- selected validation package: web TypeScript + Vitest + Rust core/game tests + production WASM deploy
- commands that ran:
  - `cargo test -p brainbreak-core --lib supernova` → **18 passed**
  - `cargo test -p brainbreak-core --lib -- both_wrists_below_hips_trigger_hands_down` → **1 passed**
  - `cargo test -p brainbreak-game` → **25 passed**
  - `npm run typecheck && npm test` → tsc clean, **107 passed**
  - `npm run deploy` / `bash scripts/deploy-worker.sh` → live build `wasm-f942bbfd6d0a`
- residual risk: browser/camera smoke not claimed; Hands/AudioWorklet/WebGPU not introduced; first M4.8 deploy briefly shipped stale WASM until M4.10 (now corrected)
- ADR: shell stays 2 rituals; Pull-to-Drop uses MoveNet body verbs only; build-wasm honors `CARGO_TARGET_DIR`; Liquid/Garden deferred by Cost gate

## Closeout Rule

- selected validation package: web TypeScript + Vitest (+ Rust core unit tests for Supernova) + production WASM deploy
- commands that ran:
  - `cd brainbreak && cargo test -p brainbreak-core --lib supernova` → **18 passed**
  - `cd brainbreak && cargo test -p brainbreak-core --lib -- both_wrists_below_hips_trigger_hands_down` → **1 passed**
  - `cd brainbreak && npm run typecheck && npm test` → tsc clean, **107 passed**
  - `cd brainbreak && npm run deploy` → live `https://brainbreak-motion-party.tuannx87.workers.dev` build `wasm-8426595f4362`
- residual risk: browser/camera smoke not claimed; Hands/AudioWorklet/WebGPU not introduced
- ADR: shell stays 2 rituals; Pull-to-Drop uses MoveNet body verbs only; Liquid/Garden deferred by Cost gate
