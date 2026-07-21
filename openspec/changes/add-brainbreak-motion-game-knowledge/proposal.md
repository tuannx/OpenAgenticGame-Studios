# Proposal

## Problem

The BrainBreak implementation contains reusable engineering decisions across deterministic Rust motion evaluation, Macroquad WASM, MoveNet, browser audio, P2P actions, and Cloudflare delivery. Without a routed skill and specialist, later games can repeat solved mistakes: UI-only camera gating, bridge ABI drift, pose-data leakage, unsupported music claims, and immutable caching of an HTML fallback.

## Proposed Change

Add a `brainbreak-motion-games` skill layered on the generic `macroquad-rust-wasm` skill, add a `brainbreak-motion-game-specialist` agent, and route `brainbreak/` work through them from `AGENTS.md`. Include a project validator and split detailed architecture/browser knowledge into progressive references.

## Scope

- Capture only behavior validated in the current repository or expressed as an explicit guardrail.
- Preserve generic Macroquad guidance in the existing skill instead of duplicating it.
- Update current discovery counts and catalogs.
- Do not change game runtime behavior or deploy production code in this change.

## Success Criteria

- The new skill passes the repository skill validator.
- Its validation script passes against `brainbreak/`.
- Agent and skill catalogs match files on disk.
- Project instructions route future BrainBreak tasks to both the generic engine and specialized product knowledge.
