# Proposal: Stacked character overlays + slow guide / fast music

## Intent

Stop leading with stick-skeleton chrome. Put **guide** and **live user** as
filled cartoon character silhouettes in the **same screen region**, each at
**~40% opacity**, with juicy stage VFX. Music can pump harder (rock/dance);
guide telegraphs stay slower so kids can follow.

## Scope

- Both shell rituals (Neon Jump Runner + Supernova Lava/Freeze)
- WASM-safe soft glow only (no mid-frame `render_target` / `set_camera`)
- Studio beds regenerated at higher BPM; VO/callouts follow guide tempo

## Numbers

| Layer | Value |
|---|---|
| Guide + user figure alpha | **0.40** each |
| Shared stage | center overlap (no bottom/top split) |
| Music BPM | **140** (was 126) |
| Guide anim rate | **0.42×** wall clock |
| Dance verb hold | **~5.2s** (was ~3.2s) |

## Out of scope

- New Cloudflare infra
- Commit (deploy only after validate)
- Text-wall tutorials
