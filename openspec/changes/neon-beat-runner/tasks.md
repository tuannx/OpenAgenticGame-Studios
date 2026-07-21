# Tasks

## Progress Snapshot

- Base framework is tagged `brainbreak-base-framework-v0.1.0` at `f969c6e`.
- Music source and CC BY 4.0 license were verified from the official Incompetech and Creative Commons pages.

## Milestone Execution Loop

1. read the active change and relevant `.codex/core` docs
2. confirm the milestone still fits the sizing rule
3. implement one bounded milestone only
4. select the validation package from `validation-matrix.md`
5. run commands, inspect first failure, fix, and repeat
6. self-review against the active change and spec
7. apply persona review when required
8. update ADR content when durable decisions change
9. record what ran, what passed, and what remains unverified

## M1. Runner Domain

- [x] M1.1 Add deterministic runner player, obstacle, pickup, phase, and spawn state.
- [x] M1.2 Map framework actions and camera evaluation to runner outcomes.
- [x] M1.3 Add focused tests for lanes, collision, scoring, gating, and restart.

## M2. Beat-Synchronized Experience

- [x] M2.1 Build the perspective neon stage and pose-driven runner avatars.
- [x] M2.2 Add licensed music playback, analyser metrics, WASM bridge, and attribution.
- [x] M2.3 Add responsive HUD, camera-first onboarding, mute, reduced motion, and game-over retry feedback.

## M3. Browser And Production Closeout

- [x] M3.1 Run Rust, WASM, TypeScript, test, audit, and production build gates.
- [ ] M3.2 HTTP asset smoke passed; interactive browser/camera smoke is pending because no browser instance was available.
- [x] M3.3 Deploy to Cloudflare and verify health plus JS/WASM/audio asset delivery.
- [x] M3.4 Record validation evidence, persona review, residual risks, and final status.

## Closeout Rule

- record the selected validation package and exact commands
- record pass/fail status and first-failure diagnosis
- record browser/device coverage gaps
- keep music attribution and ADR decisions synchronized with the release
