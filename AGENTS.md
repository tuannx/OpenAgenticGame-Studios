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

## Core Design & Collaboration Mindset

- **Self-Improvement & Continuous Skillization**: In every session, actively distill, refine, and codify learned experience, patterns, and principles into persistent, reusable Skills (`refenrece/skills/`) and Rules (`refenrece/rules/`).
- **LÀM ÍT - ĐƠN GIẢN - NGẮN GỌN - CHẤT LƯƠNG CAO (Do Less, Keep Simple & Concise, Deliver High Quality)**: Avoid feature bloat and visual clutter. Prioritize high-taste, high-impact, ultra-clean execution.
- **LUÔN TRẢ LỜI ĐỦ LÝ DO MỚI BẮT ĐẦU LÀM (Always Explain Rationale Before Executing)**: Never make blind code edits. Explain the strategic design rationale (WHY, WHAT, HOW) and confirm alignment before implementation.

## Continuous Reflect & Improve

Every task closeout and every improvement signal triggers the **Reflect Loop**
(`.codex/core/reflect-loop.md`) — a score-driven, taste-gated self-improvement
engine built on GOF patterns, SOLID principles, Hexagonal Architecture, and the
Arcade Agent optimization framework.

**How it works:**
1. **Measure** baseline (TasteScore + structural metrics) before proposing.
2. **Diagnose** the smell against `.codex/core/improve/smell-catalog.md`.
3. **Gate** via TasteScore delta rules (`.codex/core/improve/taste-gates.md`).
4. **Apply** the smallest repair through a pluggable strategy.
5. **Validate** delta and codify the learning to its durable owner.

**Triggers:** closeout, Better Loop findings, validation failures, recurring
smells, or manual `/reflect`.

**Key files:**
- Core loop: `.codex/core/reflect-loop.md`
- Strategies: `.codex/core/improve/strategies.md`
- Smell catalog: `.codex/core/improve/smell-catalog.md`
- Taste gates: `.codex/core/improve/taste-gates.md`

## BrainBreak Motion Games

For changes under `brainbreak/` or camera-controlled motion gameplay, also load:

- Specialist: `refenrece/agents/programming/brainbreak-motion-game-specialist.md`
- Skill: `refenrece/skills/brainbreak-motion-games/SKILL.md`
- Architecture contract: `refenrece/skills/brainbreak-motion-games/references/architecture-contract.md`
- Taste & Zero-Touch guide: `refenrece/skills/brainbreak-motion-games/references/taste-guide.md`

### Core Motion Game Rules
1. **Zero-Touch UI Contract**: Users standing 1.5m – 2.5m away must NEVER be forced to touch the screen. All menu picks, stance ready checks, starts, and retries MUST be 100% hands-free via pose gestures (🖐️ Hand Hold 1s, 👏 Double Clap, ↔️ Body Lean).
2. **Multiplayer Stance Check**: In 2-player / co-op modes, BOTH players must be detected in frame AND BOTH must perform ready gestures before starting.
3. **Audio-Rhythm-Lighting Synergy**: Downbeat motion triggers quantized SFX, pitch scaling (+1 semitone per 5 combo), and neon visual pulses synced to track BPM.
4. **Sensory Ritual Philosophy**: Treat 60-90 second motion breaks as a multi-sensory Awe-inducing ritual combining EDM drops, AR particle aura, fluid dynamics, and generative art.

## Replacement policy

- Do not add new `.claude/` configuration. The former Claude Code integration has been replaced by `AGENTS.md` and `.codex/`.
- When older docs mention Claude-specific paths, prefer the matching universal reference path under `refenrece/`.
- Keep historical attribution intact in license and copyright files.

## Closeout expectations

- Summarize changed behavior and cite touched files.
- Run the smallest useful validation command set.
- If a change is non-trivial, leave a change record under `openspec/changes/{change-name}/` or the active workflow record requested by `.codex/workflows/bmm.md`.
