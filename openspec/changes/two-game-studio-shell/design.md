# Design: Two-Game Studio Shell

## Problem Statement

The shell markets six “games” that are really two engines + authoring debris. Players need a zoomed-out ritual choice, not an options grid.

## Player Experience Vision

### BRAINBREAK GAME — Neon Beat Runner

**Pitch:** Your body is the controller on a neon rhythm highway for 90 seconds.

**Session (60–90s):**
1. One tap camera → step back → raise hand
2. Lean / jump / squat / clap through beat obstacles
3. Rise → Peak → Release energy arc → positive end → clap to replay

**Sensory loop:** Body verb → quantized SFX (+1 semitone / 5 combo) → neon road pulse @ 126 BPM → combo juice.

**Why efficient:** One loop, inline teaching, Zero-Touch after camera, Duo auto-unlocks when two bodies appear.

**Taste:** Cyber neon party; glanceable shapes; zero-punishment end; hands-free replay.

### AR GAME — Supernova Ritual

**Pitch:** Your camera becomes a magic mirror — dance, freeze, and explode a star with your body.

**Session (~75s):**
1. Dance on-beat to charge a floating core (AR aura on skeleton)
2. Freeze still when music stops (perfect freeze = stars)
3. Core reaches critical → Supernova Drop particle celebration

**Sensory loop:** Move → energy glow around body → silence/freeze tension → bass drop + particle awe.

**Why magical:** Freeze-Dance familiarity + AR presence + singular WOW climax (the Drop).

**Taste:** Kid-friendly party awe; AR aura > HUD clutter; co-op charges faster; positive afterglow.

## ADR 1: Two Product Families, Two Engines Kept

Context:

- Runner modes share `RunnerGame`; Supernova owns `SupernovaGame`
- Deleting either destroys unique IP

Decision:

- Keep both engines. Collapse player-facing surface to product families `brainbreak` and `ar`.
- Mirror / Strike / Duo become in-ritual variants under BrainBreak (ready lean).
- Supernova is the sole AR family mode for M1.

Tradeoffs:

- (+) Clearer taste, faster choice, reversible
- (−) Power users lose Random card on primary surface (legacy deep-link kept)
- (−) Studio custom games demoted from hero grid

Validation impact:

- Shell HTML + launcher/ready unit tests; browser smoke for family scoping

Migration and follow-up:

- Do not delete NEXT-10 docs or Studio; archive only after confirmation
- Future AR catalog games must earn the second AR slot or replace Supernova, not re-bloat the shell

## Mapping Table

| Current surface | Disposition | Destination |
|---|---|---|
| Mirror Beat | merge | BrainBreak variant |
| Beat Strike | merge | BrainBreak variant |
| Duo Groove | merge | BrainBreak variant (2P capacity) |
| Random Fun! | soft-deprecate | legacy `?mode=random` → random BrainBreak variant; art kept unreferenced |
| Supernova Drop | keep as core | Shell card **Supernova** (AR badge) |
| Catch Test / MY GAME | hide from shell | Studio + `?game=` deep link only |
| BrainBreak Studio | keep tool | settings / `/studio.html`, not hero product |
| NEXT-10 catalog | roadmap only | no launcher entries; do not delete |

## ADR 2: SCAMPER Auto-Decide (see `scamper-scoring.md`)

Context: four open product questions blocked taste polish.

Decision (by weighted rubric, Taste ≥ 3, hard-delete ≥ 85):

1. Brand shell AR card as **Supernova** (score 93)
2. Hide MY GAMES from launcher (score 80)
3. Soft-deprecate Random art; no hard-delete (score 76 vs hard-delete 59)
4. Keep Ready lean Mirror/Strike/Duo; hide lean UI when ≤1 mode (87 + 91)

## Milestone Sizing Note

- M1 = shell routing
- M2 = scored taste decisions (this slice)
- M3 hard-delete = deferred until score ≥ 85
