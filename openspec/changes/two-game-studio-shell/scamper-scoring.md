# Studio Mix — SCAMPER + Scoring (Auto-Decide)

## Inventory (Phase A)

| Surface | Type | Current disposition |
|---|---|---|
| BrainBreak Game card | Player ritual | Hero product → Runner engine |
| AR / Supernova card | Player ritual | Hero product → Supernova engine |
| Mirror / Strike / Duo | In-ritual variants | Ready lean under BrainBreak |
| Random Fun! | Convenience | Removed from shell; `?mode=random` kept |
| `game_mode_random.webp/.jpg` | Dead art | Unreferenced by shell |
| Supernova Drop | Unique IP | Sole AR mode |
| MY GAMES / custom cards | Studio tool output | Secondary grid when localStorage nonempty |
| BrainBreak Studio | Authoring tool | Settings → `/studio.html` |
| `?game=` deep link | Power-user | Loads custom config from localStorage |
| NEXT-10 catalog | Roadmap doc | Design only; no launcher stubs |

## Scoring Rubric (Phase C)

Scale **1–5** per dimension. Weighted score = `Σ(score × weight) / 5` → **0–100**.

| Dimension | Weight | What “5” means |
|---|---:|---|
| Taste fit | 25 | Zero-Touch, sensory ritual, LÀM ÍT |
| Player clarity | 20 | One job per surface; no launcher clutter |
| Unique IP value | 15 | Irreplaceable brand / mechanic |
| Cost / risk (inverted) | 15 | 5 = cheap + low regression risk |
| Retention / shareability | 15 | Strengthens 60–90s loop or invite |
| Reversibility | 10 | Easy undo via git / feature flag |

### Thresholds

| Action | Rule |
|---|---|
| **Adopt** | Score ≥ 70 and Taste ≥ 3 |
| **Hide / deprecate** | Score ≥ 75 and remaining mix Taste ≥ 3 |
| **Hard-delete** | Score ≥ 85 **and** Reversibility ≥ 4 **and** Unique IP ≤ 2 |
| **Defer** | Irreversible options within 5 points, or Taste < 3 |

## SCAMPER Findings (high-scoring only)

| Letter | Idea | Maps to | Score |
|---|---|---|---:|
| **S**ubstitute | Shell AR title → **Supernova** (badge stays AR) | AR ritual IP | **93** |
| **C**ombine | Keep Studio in settings only; rituals own first viewport | Studio tool vs rituals | 82 |
| **A**dapt | Ready `data-lean=single` when family has 1 mode | AR Ready clarity | **91** |
| **M**inify | Hide MY GAMES from launcher; keep Studio + `?game=` | Studio tool | **80** |
| **P**ut elsewhere | Random stays deep-link only | BrainBreak convenience | 78 |
| **E**liminate (soft) | Deprecate unreferenced random art; keep files | Dead art | **76** |
| **E**liminate (hard) | Delete random assets / NEXT-10 now | Dead art / roadmap | 59 ✗ |
| **R**earrange | Keep Ready lean Mirror↔Strike↔Duo | BrainBreak variants | **87** |

Rejected / low: rename everything “AR Game” (53); promote MY GAMES to third card (39); collapse Ready to 1 tempo only (75, loses Strike IP surface).

## Decision Log (Phase D)

### Q1 — AR brand

| Option | Score | Verdict |
|---|---:|---|
| Dual: shell “AR Game” + Ready Supernova | 80 | runner-up |
| Everything “AR Game” | 53 | reject |
| **Shell “Supernova” + AR badge; Ready “Supernova Drop”** | **93** | **WIN** |

### Q2 — MY GAMES on shell

| Option | Score | Verdict |
|---|---:|---|
| Keep secondary grid when present | 67 | below adopt |
| **Hide from shell; Studio + `?game=` only** | **80** | **WIN** |
| Promote to third hero card | 39 | reject |

### Q3 — Random / NEXT-10 / dead art

| Option | Score | Verdict |
|---|---:|---|
| Hard-delete assets + NEXT-10 | 59 | below hard-delete gate |
| **Soft-deprecate: no shell refs; keep files + NEXT-10 roadmap** | **76** | **WIN** |
| Move assets to archive folder now | 68 | defer |

### Q4 — Ready lean

| Option | Score | Verdict |
|---|---:|---|
| **Keep lean Mirror / Strike / Duo** | **87** | **WIN** |
| 1 tempo + Duo auto only | 75 | reject (hides Strike) |
| Capacity auto-pick only | 77 | reject (cost + surprise) |

### Extra adopted

| Decision | Score | Action |
|---|---:|---|
| Hide lean chevrons when family has ≤1 mode | 91 | Ready `data-lean` |
| Do not delete NEXT-10.md | — | roadmap IP; Supernova already shipped from it |

## Intentionally NOT done

- Hard-delete of `game_mode_random.*`
- Engine merges / Supernova deletion
- Cloudflare / Worker changes
- Building additional NEXT-10 games into the shell
