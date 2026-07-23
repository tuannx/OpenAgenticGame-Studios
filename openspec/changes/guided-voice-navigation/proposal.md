# Proposal: Guided voice navigation (VO + music)

## Intent

Kids cannot read on-canvas English. After non-text pictogram/SFX cues, add
**spoken coach VO** at key moments so navigation and in-ritual guidance stay
hands-free — combined with music beds/stingers (call-and-response mix).

## Player-facing rule

Primary channels = **picture + voice + music**. No word walls. ≤1 short English
kid-coach sentence per cue. Pace leaves space (no VO tip spam).

## Tech

- Cue map: `party-voice-cues.ts`
- Playback: optional `/voice` Kokoro clips → Web Speech API fallback
- Mix: `musicEngine.duck()` under VO; freeze still **pauses** the bed
- Fallback: SFX + pictograms when speech unavailable

## Out of scope

- YouTube / Danny Go vocal rips
- New Cloudflare services
- BackdropCache changes
- Constant chatter / bilingual walls
