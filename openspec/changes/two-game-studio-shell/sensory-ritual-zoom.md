# Sensory Ritual Zoom — 3 Scenarios → 2 Shell Games

**WHY**: Shell already ships two rituals. The sensory brief asks for Awe (not points). Zoom into each ritual’s UX / input / juice; do not add a third card or greenfield engines.

**WHAT**: Score Supernova Drop / Liquid Co-op / Neon Growth Garden onto the existing two products; ship the smallest wow + simplify slice.

**HOW**: Same SCAMPER rubric as `scamper-scoring.md`. Auto-decide by score. Defer heavy tech behind infra proposals.

## Rubric (unchanged)

Weighted 0–100. Adopt ≥ 70 and Taste ≥ 3. Defer when Cost ≤ 2 unless the change is documentation-only.

| Dimension | Weight |
|---|---:|
| Taste fit | 25 |
| Player clarity | 20 |
| Unique IP value | 15 |
| Cost / risk (inverted) | 15 |
| Retention / shareability | 15 |
| Reversibility | 10 |

## Scenario Scores

### 1. Supernova Drop → **AR ritual (WIN)**

Enhance existing Freeze→Drop engine; Drop = viral share beat.

| Dim | Score | Note |
|---|---:|---|
| Taste | 5 | Zero-Touch sensory ritual; Rise→Peak→Release |
| Clarity | 5 | Already sole AR card |
| Unique IP | 5 | Drop moment is product IP |
| Cost | 5 | Engine + renderer exist; juice only |
| Retention | 5 | Share the Drop |
| Reversibility | 5 | Visual/SFX only |

**Score: 100 → ADOPT as AR shell ritual**

### 2. Liquid Co-op → **DEFER (roadmap DNA → Duo invite)**

| Dim | Score | Note |
|---|---:|---|
| Taste | 5 | “Must play together” is core mandate |
| Clarity | 4 | Clear co-op job |
| Unique IP | 4 | TeamLab shatter is distinct |
| Cost | 1 | Fluid/metaball/split/refraction = new engine |
| Retention | 5 | Invite coworker/family |
| Reversibility | 3 | Large surface |

**Score: 77** — above adopt for *roadmap*, but Cost=1 blocks build-now.

**Decision**: Do **not** add a Liquid engine this slice. Absorb invite DNA into BrainBreak Duo auto-invite. Roadmap: future BrainBreak co-op layer / Duo evolution.

### 3. Neon Growth Garden → **DEFER (creative afterglow)**

| Dim | Score | Note |
|---|---:|---|
| Taste | 5 | Generative awe, no Start button |
| Clarity | 3 | Softer verb story than Drop/Runner |
| Unique IP | 5 | Ownership / Octalysis creativity |
| Cost | 1 | L-system + trails + fog dismiss |
| Retention | 4 | Shareable unique art |
| Reversibility | 3 | Large surface |

**Score: 73** — roadmap only.

**Decision**: Keep off shell. Future share/afterglow layer (result card art seed), not a third ritual.

## Shell Mapping (locked)

| Shell card | Engine | Primary sensory arc | Absorbs from brief |
|---|---|---|---|
| **BrainBreak Game** | Neon Beat Runner | Rise → Peak → Release (90s) | Liquid’s “must play together” via Duo auto-invite |
| **Supernova** (AR) | Supernova Drop | Build → Tension → **DROP** → Afterglow | Scenario 1 as-is (zoom Drop wow) |

Mirror / Strike / Duo lean: **keep** (prior score 87). Do not collapse to one tempo.

| Option | Score | Verdict |
|---|---:|---|
| Keep Mirror/Strike/Duo lean | 87 | **WIN** (prior) |
| 1 tempo + Duo only | 75 | reject — hides Strike IP |
| Replace Runner with Liquid now | 52 | reject — Cost + clarity loss |
| Replace Runner with Garden now | 48 | reject — Cost + softer ritual |

## This-Slice Adopt List

| Change | Score | Action |
|---|---:|---|
| Duo auto-invite when 2 bodies in BrainBreak Ready | **92** | Implement |
| Drop imminent micro-shake + bigger Drop flash/bass/haptic | **94** | Implement |
| Wire Supernova on-beat hits → combo-pitch SFX | **90** | Implement |
| Shell/Ready copy: fewer words, ritual arc | **88** | Implement |
| Ready-gate ritual pacing / hold juice | **90** | **Shipped (M4.11)** |
| Afterglow / share moment (Garden DNA, no engine) | **84** | **Shipped (M4.12)** |
| Extra Runner/Supernova flash density pass | 68 | Skip — Drop/downbeat already ship; diminishing ROI |
| Pinch/Grab pull-to-drop verb | 61 | Defer Hands path — **M-S2 body metaphor shipped**: squat / both-wrists-below-hips |
| AudioWorklet → shader bass sync | 58 | **Infra proposal** — confirm first |
| WebGPU compute particles (10K) | 55 | **Infra proposal** — confirm first |
| Liquid fluid engine | 52 | Roadmap task |
| Neon Garden L-system | 48 | Roadmap task |

## Infra Proposals (need explicit user confirm)

1. **AudioWorklet bass bus** — zero-latency band energy → WASM/uniforms. Current AnalyserNode (~12ms poll) is “good enough” for neon pulse; Worklet unlocks true Drop deformation. Pre-Drop lowshelf + synth swell already ships without Worklet.
2. **WebGPU / compute particles** — 10K Drop particles + metaball core. Macroquad confetti (~150) is the shipped wow ceiling without new stack.
3. **MediaPipe Hands (Workers)** — Pinch/Grab for pull-to-drop. MoveNet body verbs remain primary; Hands is additive infra. **Body release (squat / hands-down) ships without Hands.**
4. **Dynamic split / metaball merge invite** — Liquid visual language; stub only until Liquid milestone.

## Phased Roadmap Hooks

1. **M-S1 (this slice)**: Duo invite + Drop tension/juice + hit SFX + copy
2. **M-S2**: Pull-to-drop body metaphor (squat / hands-down) + pre-Drop bass swell — **shipped**
3. **M-S2b**: BrainBreak downbeat sync tighten — **shipped**
4. **M-S2c**: WASM `CARGO_TARGET_DIR` deploy hygiene — **shipped**
5. **M-S2d**: Ready-gate ritual pacing / copy — **shipped**
6. **M-S2e**: WASM panic fix — remove Runner BackdropCache `render_target` — **shipped**
7. **M-S2f**: Afterglow / share moment light polish (Garden DNA) — **shipped**
8. **M-S3**: Liquid Co-op as BrainBreak Duo evolution (fluid core, shatter end)
9. **M-S4**: Neon Growth Garden as shareable afterglow / Studio generative pack
