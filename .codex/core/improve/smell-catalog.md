<!-- input: trigger signals from reflect-loop step 2 -->
<!-- output: smell classification and recurrence count -->
<!-- pos: living catalog of detected process and instruction smells -->
# Smell Catalog

Detection rules for recurring instruction and process problems.
Each entry defines: pattern, detection method, and recurrence count.

When a smell recurs (count ≥ 2), the Reflect Loop escalates it from
one-time fix to systemic prevention (new gate, rule, or automation).

## Smell Types

### stale-reference

**Pattern:** A documented command, path, version, or script does not match
actual project state.

**Detection:**
- Run the documented command → `command not found` or non-zero exit.
- Compare documented path with filesystem → path absent.
- Compare documented version with lockfile/manifest → mismatch.

**Recurrence count:** 1
**First seen:** 2026-07-22 (Better Loop finding: `npm run lint`, `check:syntax`
absent from brainbreak/package.json)

---

### missing-enforcement

**Pattern:** A documented quality gate has no mechanical backing (no CI,
no hook, no required check).

**Detection:**
- Gate declared in `executable-guardrails.md` or `validation-matrix.md`.
- No corresponding CI workflow, git hook, or pre-commit entry exists.

**Recurrence count:** 1
**First seen:** 2026-07-22 (Better Loop finding: no `.github/workflows/`)

---

### undocumented-boundary

**Pattern:** A delivery, permission, or recovery boundary exists in practice
but is not written in any instruction file.

**Detection:**
- Feature branch diverges from base with no documented merge criteria.
- Deploy script exists but no approval or rollback documentation.
- Permission mode is implicit rather than declared.

**Recurrence count:** 1
**First seen:** 2026-07-22 (Better Loop finding: no acceptance boundary)

---

### taste-drift

**Pattern:** TasteScore sub-weights no longer reflect project priorities.
Improvements score high structurally but feel wrong to the user.

**Detection:**
- User rejects a high-score proposal.
- Two consecutive improvements optimize the same dimension while another
  dimension visibly degrades.

**Recurrence count:** 0

---

### process-gap

**Pattern:** A recurring situation has no codified guidance; agents rediscover
the same solution each session.

**Detection:**
- Same question or friction appears in ≥ 2 sessions.
- A LESSONS-LEARNED entry exists but is not wired into the execution path.

**Recurrence count:** 0

---

## Escalation Rule

| Count | Action |
|-------|--------|
| 1 | One-time fix via selected strategy |
| 2 | Add detection rule + prevention gate |
| 3+ | Systemic refactor of the owning instruction surface |

## Failed Attempts Log

| Date | Smell | Strategy | Why it failed |
|------|-------|----------|---------------|
| — | — | — | No failed attempts yet |

## Extension

To add a new smell: copy a section template, define pattern + detection,
set recurrence count to 0. The Reflect Loop will populate first-seen on
initial match.
