# Proposal: Shared mid-playfield stage (not bottom 40%)

## Intent

Guide + live user were still reading as a **bottom strip**. Replace any leftover
bottom-40% coach band with one **mid-playfield shared stage** (~68% viewport
height, vertically centered) so both filled cartoon figures overlap mid-screen.

## Scope

- `guide_coach_layout` geometry + tests
- Runner (`visuals.rs`) and Supernova (`supernova_render.rs`) pink overlay anchors
- Validate + Cloudflare deploy (no commit)

## Numbers

| Param | Value |
|---|---|
| Stage height | **0.68** of viewport |
| Slack bias | **0.48** (near-centered) |
| Figure scale | **0.48** of stage height |
| Figure alpha | **0.40** each (unchanged) |

## Out of scope

- New Cloudflare infra
- Git commit
