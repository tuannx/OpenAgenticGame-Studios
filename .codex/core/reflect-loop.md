<!-- input: trigger signal (closeout, finding, failure, smell, manual) -->
<!-- output: measured, gated, validated improvement applied to project instructions -->
<!-- pos: core continuous-improvement loop shared by Quick and BMM -->
# Reflect Loop

Continuous self-improvement engine. Triggered by any signal, governed by
score-driven gates, applied through the smallest correct owner.

## Architecture

Hexagonal: the core loop depends on two abstractions, not concrete files.

- **ReflectInput port**: any trigger source (closeout, Better Loop finding,
  validation failure, recurring smell, manual `/reflect`).
- **ImprovementOutput port**: any durable target (skills, rules, lessons,
  validation-matrix, harness, baseline.json, smell-catalog).

Strategies are pluggable (GOF Strategy + OCP): add new ones in
`.codex/core/improve/strategies.md` without modifying this loop.

## Trigger Signals (GOF Observer)

| Signal | Source | Example |
|--------|--------|---------|
| `closeout` | `closeout-loop.md` step 7 | Task finished, distill learning |
| `finding` | Better Loop report | Stale validation commands |
| `validation-failure` | Guardrail command exit ≠ 0 | `npm run lint` not found |
| `recurring-smell` | smell-catalog match count ≥ 2 | Same command missing twice |
| `manual` | User says `/reflect` or "improve the loop" | Ad-hoc inspection |

## Loop Steps (GOF Template Method)

Every trigger runs the same five steps. Individual steps may select different
strategies, but the sequence is fixed.

### 1. Measure Baseline

Read current state before proposing anything.

- Read `brainbreak/baseline.json` (or create if absent).
- Collect: `TasteScore`, `ArchitectureBalance`, `SelfImprovementLeverage`.
- If no baseline exists, measure and record one now.

### 2. Diagnose Smell

Match the trigger against `.codex/core/improve/smell-catalog.md`.

- Classify: `stale-reference`, `missing-enforcement`, `undocumented-boundary`,
  `taste-drift`, `process-gap`, or `new-unknown`.
- If match count ≥ 2 for same smell type → flag `recurring`.
- If `new-unknown` → add to catalog after this loop completes.

### 3. Propose & Gate (Arcade Agent)

Select strategy from `.codex/core/improve/strategies.md`.

- Compute: `Value = Gap × Leverage × TasteWeight / Effort`
- Gate (taste-aware delta):
  - 🟢 primary TasteScore gain > secondary taste loss × 2
  - 🟢 structural health ≥ current baseline
  - 🔴 if gate fails → reject proposal, log reason, stop
- Declare: baseline → target → rollback trigger.

### 4. Apply Change

Execute the smallest repair at the correct owner via ImprovementOutput port.

- One problem → one primary owner file.
- Do not expand scope beyond the diagnosed smell.
- Record the applied diff summary.

### 5. Validate Delta

Measure after the change and compare.

- Re-read affected metrics.
- Report: 🟢 improved / 🔴 regressed / ⚪ unchanged per metric.
- If 🔴 on any gate metric → rollback + log as `failed-attempt` in
  smell-catalog.
- If 🟢 → update `baseline.json` with new values.

## State Machine (GOF State)

```
detected → measured → proposed → gated → applied → validated → codified
                                    ↓ (gate fail)
                                 rejected
                                    ↓ (delta 🔴)
                              rolled-back → logged
```

## Codify (via ImprovementOutput port)

After `validated`, write the learning to its durable owner:

| Output type | Target |
|-------------|--------|
| New detection rule | `smell-catalog.md` |
| Process fix | `harness.md` or `validation-matrix.md` |
| Reusable procedure | `refenrece/skills/` |
| One-time lesson | `.codex/LESSONS-LEARNED.md` |
| Taste model update | `taste-gates.md` |

## Constraints

- Never propose without a baseline measurement.
- Never apply without passing the taste-aware gate.
- Never close the loop without a delta report.
- One loop per trigger; do not batch unrelated smells.
- The loop is mode-agnostic: Quick and BMM trigger identical quality.
