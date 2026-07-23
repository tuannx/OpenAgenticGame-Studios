# Proposal: Two-Game Studio Shell

## Intent

Consolidate the player-facing BrainBreak Party shell into exactly **two** games with distinct rituals:

1. **BRAINBREAK GAME** — Neon Beat Runner motion break (Mirror / Strike / Duo as in-ritual variants)
2. **SUPERNOVA** (AR badge) — Supernova Drop camera-magic freeze ritual

## Why This Change

- Six equal cards dilute taste and decision speed.
- Mirror / Strike / Duo share one core loop (`RunnerGame`); Supernova is a second loop (`SupernovaGame`).
- Studio / Catch Test / Random are authoring or convenience, not hero products.

## In Scope

- Player-facing launcher: two product cards only
- Deep-link aliases for legacy mode keys
- Ready-gate lean scoped inside the selected family
- SCAMPER + scoring auto-decide for remaining open questions
- Change record + vision for AR vs BrainBreak experiences

## Out Of Scope

- Hard-delete of Random art / NEXT-10 (score below gate)
- Cloudflare / Worker infra changes
- Building NEXT-10 catalog games into the shell
- Renaming production hostname

## Acceptance Criteria

1. Launcher shows exactly two primary game cards: BrainBreak + Supernova (AR)
2. Selecting BrainBreak launches a runner mode; lean cycles Mirror / Strike / Duo (capacity-aware)
3. Selecting Supernova launches AR; lean chevrons hide when only one family mode
4. Legacy `?mode=mirror|strike|duo|supernova|random` still resolves; `?game=` still loads Studio configs
5. MY GAMES cards do not appear on the player shell
6. No unique game IP hard-deleted (scoring gate)

## Success Criteria

- First viewport reads as one composition with two clear rituals
- Time-to-camera-tap drops (fewer choices)
- Both rituals remain Zero-Touch after the single camera tap
