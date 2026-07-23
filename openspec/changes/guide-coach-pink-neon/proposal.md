# Proposal: Guide coach + pink neon user skeleton

## Intent

At Zero-Touch distance (1.5–4m), kids need a large **bottom coach silhouette**
(~40% screen height, ~50% opacity navigation frame) that **moves with VO cues**,
plus a **pink neon** live skeleton overlay (~50% opacity + bloom) so hierarchy is
clear: coach = what to do, pink neon = you.

## Scope

- Both rituals: BrainBreak Neon Runner + Supernova Floor-is-Lava/Freeze
- WASM-safe soft-glow only (no mid-frame `render_target` / `set_camera`)
- Synced to existing Kokoro `/voice` + party-director moments

## Out of scope

- New Cloudflare infra
- YouTube / Danny Go IP rips
- Word-wall tutorials
