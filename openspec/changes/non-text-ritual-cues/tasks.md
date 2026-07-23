# Tasks — Non-text ritual cues

## Progress

- [x] Score format catalog → 2 rituals (proposal.md)
- [x] Supernova: strip JUMP/FLOOR IS LAVA/DROP/WOW word walls → pictograms + traffic lights + lava floor
- [x] Densify elemental VFX (wind / lava embers / ice snow / drum rings)
- [x] party-voice: guided VO cue map + SFX stingers (clips / SpeechSynthesis; duck music)
- [x] BrainBreak action cues: pictogram-first, hide verb/lane English
- [x] Shell/Ready: SVG gesture cues replace blurb walls
- [x] Validate + deploy Cloudflare

## Validation

- `cargo test -p brainbreak-game`
- `cargo test -p brainbreak-core --lib supernova`
- `npm run typecheck && npm test`
- `npm run deploy` / rebuild wasm

Follow-on: `openspec/changes/guided-voice-navigation/`
