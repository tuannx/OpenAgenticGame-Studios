# OpenAgenticGame Studios — Codex Instructions

This repository is Codex-native. Use this file as the primary project instruction source and use `.codex/` as the active execution layer.

## Start every task

1. Read `.codex/README.md` to choose the current workflow entrypoint.
2. Route with `.codex/workflows/router.md`:
   - **Quick** for scoped fixes, bounded features, and clear refactors.
   - **BMM** for unclear, architectural, cross-cutting, or multi-milestone work.
3. Load only the reference files needed for the current task from `refenrece/` and `.codex/reference/`.
4. Keep changes small enough to review, and record validation evidence before closeout.

## Codex feature mapping

- Use Codex's local file editing for implementation and refactors.
- Use Codex's shell execution for focused validation commands.
- Use Codex multimodal inputs when screenshots, diagrams, UI mockups, or visual QA are part of the task.
- Use Codex approval/sandbox modes according to risk: suggest for exploration, auto-edit for bounded documentation or refactors, and full-auto only for trusted local validation loops.
- Prefer repo-local workflow docs over legacy platform-specific assumptions.

## Studio references

- Agents: `refenrece/agents/`
- Skills: `refenrece/skills/`
- Rules: `refenrece/rules/`
- Templates: `refenrece/docs/templates/`
- Codex workflows: `.codex/workflows/`
- Codex core gates: `.codex/core/`

## Macroquad Rust/WASM

For Macroquad projects, load:

- Specialist: `refenrece/agents/programming/macroquad-specialist.md`
- Skill: `refenrece/skills/macroquad-rust-wasm/SKILL.md`
- Version reference: `docs/engine-reference/macroquad/VERSION.md`

Validate native Rust behavior and the release `wasm32-unknown-unknown` artifact
separately. Browser-facing changes also require an HTTP-served smoke test; a
successful Cargo build alone is not browser-runtime proof.

## Replacement policy

- Do not add new `.claude/` configuration. The former Claude Code integration has been replaced by `AGENTS.md` and `.codex/`.
- When older docs mention Claude-specific paths, prefer the matching universal reference path under `refenrece/`.
- Keep historical attribution intact in license and copyright files.

## Closeout expectations

- Summarize changed behavior and cite touched files.
- Run the smallest useful validation command set.
- If a change is non-trivial, leave a change record under `openspec/changes/{change-name}/` or the active workflow record requested by `.codex/workflows/bmm.md`.
