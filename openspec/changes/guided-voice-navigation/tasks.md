# Tasks — Guided voice navigation

## Progress

- [x] Cue map for Shell/Ready + Supernova + BrainBreak key moments
- [x] Restore PartyVoice speak path (clips → SpeechSynthesis)
- [x] Music duck under VO; freeze pause unchanged
- [x] Wire Ready + launcher ritual selection + gameplay start
- [x] Update generate-voice.mjs phrase list
- [x] Tests for cue map + director ducking/hold announce
- [x] Validate + deploy Cloudflare

## Validation

- `npm run typecheck && npm test` (brainbreak) — 117 passed
- `npm run deploy` → https://brainbreak-motion-party.tuannx87.workers.dev (`wasm-1d2fbe34f3f7`)
