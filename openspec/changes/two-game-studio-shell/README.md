# Two-Game Studio Shell

## Status

- in_progress — M1–M2 shell done; M4 sensory zoom shipped; M3 hard-delete + Liquid/Garden deferred by score

## Harness Alignment

- follows `.codex/core/work-breakdown.md`
- follows `.codex/core/task-sizing.md`
- follows `.codex/core/milestone-design.md`
- follows `.codex/core/validation-matrix.md`
- follows `.codex/core/adr-rules.md`
- follows `.codex/core/persona-review.md`
- follows `.codex/core/closeout-loop.md`
- BMM-sized: product consolidation across launcher, deep links, ready navigation, and taste routing

## Source Context

- `game-shell.png`, `studio-initial.png`
- `brainbreak/GAME-DESIGN.md`, `brainbreak/NEXT-10-GAMES.md`
- `refenrece/skills/brainbreak-motion-games/references/taste-guide.md`
- `scamper-scoring.md` — auto-decide rubric + decision log
- `sensory-ritual-zoom.md` — 3 scenarios → 2 rituals mapping + infra proposals

## Why This Change Exists

- Player-facing shell presented too many equal “games.”
- Taste mandate: LÀM ÍT — two high-impact rituals own attention.
- Open product questions resolved by explicit SCAMPER scoring, not waiting on chat.
- Sensory brief: 60–90s Awe rituals — zoom Drop + Duo invite without new engines.

## Execution Model

1. M1 — Two product families on the shell
2. M2 — SCAMPER-scored taste: Supernova brand, hide MY GAMES, keep lean variants, soft-deprecate Random
3. M3 — Hard-delete only if score ≥ 85 (currently deferred)
4. M4 — Sensory zoom: Duo auto-invite, Drop tension/juice, ritual copy (Liquid/Garden deferred)

## Validation Strategy

- Vitest / TypeScript for shell + Duo invite
- Rust unit tests for Supernova drop_imminent / co-op charge
- Manual browser smoke still required for camera path
- No infrastructure change without explicit confirm
