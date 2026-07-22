<!-- input: diagnosed smell type and context from reflect-loop step 2 -->
<!-- output: selected improvement strategy with value computation -->
<!-- pos: GOF Strategy registry — extend here without modifying reflect-loop.md -->
# Improvement Strategies

Pluggable strategies selected by the Reflect Loop at step 3 (Propose & Gate).
Each strategy owns its selection condition, procedure, and validation route.

To add a new strategy: append a section below. Do not modify `reflect-loop.md`.

## Strategy: fix-stale-reference

**Select when:** smell = `stale-reference` (documented command, path, or version
conflicts with actual project state).

**Procedure:**
1. Identify the declaring file and the actual truth (package.json, Cargo.toml, filesystem).
2. Replace the stale reference with the current truth.
3. If the truth no longer exists, remove the reference and note the gap.

**Validation:** Run the corrected command; confirm exit 0.

**TasteWeight:** 0.8 (structural clarity, low emotional impact).

---

## Strategy: add-enforcement

**Select when:** smell = `missing-enforcement` (documented gate has no mechanical
backing — no CI, hook, or required check).

**Procedure:**
1. Identify the gate and its intended trigger point.
2. Choose the smallest enforcement: CI workflow > git hook > required status check.
3. Implement the enforcement with a deliberate-failure test path.

**Validation:** Trigger a deliberate violation; confirm the gate blocks.

**TasteWeight:** 0.9 (safety and trust amplification).

---

## Strategy: codify-lesson

**Select when:** smell = `process-gap` or trigger = `closeout` with a new
non-trivial learning that does not match an existing rule or skill.

**Procedure:**
1. Distill the learning into one actionable statement.
2. Route to the correct owner:
   - Recurring pattern → `refenrece/skills/` or `refenrece/rules/`
   - One-time insight → `.codex/LESSONS-LEARNED.md`
   - Workflow change → `.codex/core/` or `.codex/workflows/`
3. Keep it under 10 lines. No prose walls.

**Validation:** A future agent encountering the same situation finds the
guidance within one file read.

**TasteWeight:** 0.7 (knowledge compounding).

---

## Strategy: evolve-taste-model

**Select when:** smell = `taste-drift` (current TasteScore sub-weights no longer
reflect project priorities or user feedback).

**Procedure:**
1. Collect evidence: which taste dimension was under/over-weighted?
2. Propose new weights with rationale.
3. Gate: primary gain > secondary loss × 2 (standard taste gate).
4. Update `taste-gates.md` weights.

**Validation:** Re-score the last 3 improvements with new weights; confirm
ranking aligns better with observed outcomes.

**TasteWeight:** 1.2 (meta-improvement, high leverage).

---

## Strategy: document-boundary

**Select when:** smell = `undocumented-boundary` (delivery acceptance, permission,
or recovery boundary exists in practice but not in instructions).

**Procedure:**
1. Identify the implicit boundary and its current owner.
2. Write the explicit rule in the owner's file (validate.md, harness.md, etc.).
3. Add a verification route (command, check, or evidence path).

**Validation:** An agent reading the instruction can discover and follow the
boundary without verbal context.

**TasteWeight:** 0.85 (safety + clarity).

---

## Extension Rule (OCP)

To add a strategy:

1. Copy the section template above.
2. Define: select-when, procedure, validation, TasteWeight.
3. Append below. No other file needs modification.
