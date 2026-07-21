# Tasks and Validation Evidence

## Milestone 1 — Reusable Skill

- [x] Scaffold `brainbreak-motion-games` with the repository skill creator.
- [x] Write the workflow-oriented `SKILL.md`.
- [x] Add architecture and web-runtime references.
- [x] Add a non-deploying validation script.
- [x] Run skill structure validation.
- [x] Run the skill validator against `brainbreak/`.

## Milestone 2 — Agent and Routing

- [x] Add `brainbreak-motion-game-specialist`.
- [x] Route BrainBreak tasks from root `AGENTS.md`.
- [x] Update skill and agent discovery catalogs.
- [x] Normalize current README/workflow counts.
- [x] Verify manifest counts against files on disk.
- [x] Review the final diff for runtime changes and stale placeholders.

## Validation Evidence

- `/usr/bin/python3 refenrece/skills/skill-creator/scripts/quick_validate.py refenrece/skills/brainbreak-motion-games` — passed.
- `bash -n refenrece/skills/brainbreak-motion-games/scripts/validate.sh` — passed.
- `bash refenrece/skills/brainbreak-motion-games/scripts/validate.sh brainbreak https://brainbreak-motion-party.tuannx87.workers.dev` — passed: 16 Rust tests, 4 TypeScript tests, release WASM, Vite production build, zero audited vulnerabilities, and live HTML/JavaScript/WASM/audio content-type probes.
- Catalog check — passed: 87 agent files, 74 skill directories/manifest entries, 73 executable `SKILL.md` packages, and one correctly marked legacy documentation bundle (`unified-agents`, `hasSkillMd: false`).
- Persona review — security, performance, readability, and runtime-regression lenses found no runtime mutation; the validator is non-deploying and the remaining interactive-browser gap is explicit.
- This change does not alter or deploy runtime code, so camera/audio/two-peer visual QA was not repeated.
