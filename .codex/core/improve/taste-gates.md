<!-- input: baseline metrics and proposed improvement from reflect-loop step 3 -->
<!-- output: pass/reject gate decision with taste-aware delta computation -->
<!-- pos: TasteScore model and delta gates for the Arcade Agent improvement engine -->
# Taste Gates

Score-driven quality model governing all self-improvement proposals.
No optimization proceeds without a baseline and a passing gate.

## TasteScore Model

```
TasteScore = w₁×EmotionalArc + w₂×UXRefinement + w₃×SimplicityIndex
```

| Sub-score | Definition | Weight |
|-----------|-----------|--------|
| `EmotionalArc` | User-reported joy/frustration delta per session | 0.4 |
| `UXRefinement` | Task success rate × time-to-flow | 0.35 |
| `SimplicityIndex` | 1 − (feature_count / core_feature_target); cap at 1.0 | 0.25 |

**Scale:** 0–100. Baseline recorded in `brainbreak/baseline.json`.

## Supporting Metrics

| Metric | Formula | Target |
|--------|---------|--------|
| `ArchitectureBalance` | 1 − (max_file_LOC / target_LOC) | ≥ 0.7 |
| `PrincipleSignalStrength` | decisions_with_2+_signals / total_decisions | ≥ 0.6 |
| `SelfImprovementLeverage` | (new_rule_coverage − old_coverage) / effort | ≥ 0.1 |
| `DocToProductRatio` | workflow_LOC / product_LOC | ≤ 0.3 |

## Delta Gate Rules

Every proposal must pass ALL applicable gates before apply:

### Gate 1: Primary Gain Dominance

```
primary_taste_gain > secondary_taste_loss × 2
```

If the improvement helps one dimension but hurts another, the gain must
be at least double the loss.

### Gate 2: Structural Floor

```
post_change_structural_health ≥ baseline_structural_health
```

No improvement may degrade structural metrics below the current baseline.

### Gate 3: Simplicity Budget

```
post_change_SimplicityIndex ≥ baseline_SimplicityIndex − 0.05
```

Allow at most 5% simplicity erosion for a significant taste gain.

### Gate 4: Recurrence Prevention

If the same smell type has count ≥ 2 in smell-catalog:

```
proposal MUST include a detection rule or automation
```

One-time fixes are insufficient for recurring smells.

## TasteWeight per Domain

Applied in Value computation: `Value = Gap × Leverage × TasteWeight / Effort`

| Domain | TasteWeight | Rationale |
|--------|-------------|-----------|
| UI / Player-facing | 1.3 | Direct emotional impact |
| Workflow / Instructions | 1.0 | Agent effectiveness |
| Backend / Infrastructure | 0.7 | Indirect user impact |
| Meta-improvement | 1.2 | High leverage on future work |

## Baseline Protocol

1. On first use: measure all metrics, write to `brainbreak/baseline.json`.
2. Before each proposal: read baseline, confirm freshness (< 30 days or
   last improvement cycle).
3. After validated improvement: update baseline with new values.
4. Never delete historical baselines; append with timestamp.

## Integrity Check

Before closing any improvement loop, answer:

> "Which taste dimension degrades with this change?"

If the answer is "none" → proceed.
If the answer names a dimension → apply Gate 1 strictly.
If primary gain ≤ secondary loss × 2 → **reject and log**.

## Evolution

This model evolves via the `evolve-taste-model` strategy. Weight changes
require:
- Evidence from ≥ 2 improvement cycles.
- User confirmation of new priorities.
- Delta report showing improved ranking accuracy.
