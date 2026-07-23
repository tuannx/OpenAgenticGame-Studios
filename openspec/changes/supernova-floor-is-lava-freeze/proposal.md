# Proposal: Supernova Floor-is-Lava / Freeze ritual

## Intent

Align Supernova with the classic kids freeze-dance party loop
(dance / move → danger callout → 5-4-3-2-1 → FLOOR IS LAVA / FREEZE →
resume), without redistributing copyrighted commercial kids audio.

## Principle — taste / format only, original per game

User clarification: apply Danny Go (and similar kids dance-along) **taste +
format** only; **proactively create original tracks** for each shell ritual.
Never rip YouTube audio, copy melodies/lyrics, or claim “official Danny Go.”

Also ship FUN, Zero-Touch-slow interaction, music that **drives** gestures, a
tiny story arc, elemental atmospheres (ice / fire / wind / smoke / ocean), and
everyday movement verbs + light king/queen pretend — all inside the **existing
two shell games**, not new launcher cards.

## Decision — audio (license stance)

Players cited commercial YouTube kids dance tracks as *genre* references only:

| Requested reference | Status |
|---|---|
| The Kiboomers — “The Floor is Lava Freeze Song” (`wbNAiN8FTfc`) | **Refused** — commercial; no license; not ripped/embedded |
| Danny Go! channel / compilations | **Refused** — commercial; no license; inspiration only; no Danny Go branding in-game |

**Shipped instead (studio original / CC0):**

| Ritual | Track | Notes |
|---|---|---|
| BrainBreak runner | **Neon Jump Party** | Bright major, jump/clap pulses, wind-rise drops |
| Supernova AR | **Lava Freeze Party** | Ocean/wind → smoke/fire → ice breath phrase map |

Generator: `brainbreak/scripts/generate-studio-beds.mjs`.

## Elemental story chapters (one journey, not five games)

| Phase | Atmosphere | Body cue |
|---|---|---|
| Countdown / Ready | Smoke clear | Open the party |
| Dance | Wind + ocean | Move / verb tips |
| LavaWarning | Smoke + fire | 5→1 readable |
| Freeze | Ice | Stillness (music pause) |
| Drop | Fire celebrate | Squat / hands-down release |
| Result | Ocean cool | Afterglow |

## Everyday verbs + roles (LÀM ÍT)

| Verb / role | Sense | Maps to |
|---|---|---|
| Jump, Clap | detect | `Jump`, `Clap` |
| Paint, Reach, Royal wave (king/queen) | detect | `LeftUp` / `RightUp` |
| Tiptoe | flavor≈rise | announce; Jump-adjacent |
| Wiggle, March | flavor | announce; lean optional |
| Freeze | detect via stillness | music pause + no motion |

Slow mid-dance voice tips (~3.2s) + rotating on-canvas cues.

## Out of scope

- Bundling Kiboomers, Danny Go, or any commercial kids catalog
- New Cloudflare infrastructure / WebGPU / Hands pipeline
- WASM BackdropCache mid-frame render targets
- Extra launcher cards for elements or verbs
