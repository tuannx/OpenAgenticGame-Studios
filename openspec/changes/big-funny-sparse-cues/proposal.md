# Big funny sparse cues + baked VO

## Intent

Kids at 1.5–4m need **one huge funny hero symbol** per beat, not stacked
pictograms / HUD words. Spoken coach + music beds carry guidance.

## Size contract (cues AND buttons)

**Rule**: shorter edge ≥ **20%** of `min(viewport_w, viewport_h)`.

Applies to:

1. Active guidance pictograms / hero silhouettes (`juice::hero_edge`)
2. **All player-facing buttons / cards / soft-select targets** — shell ritual
   cards, CAMERA / PREVIEW CTAs, Ready hold + lean affordances, touch fallbacks,
   settings summary + control-grid controls, guide-demo CTAs, WASM result chips

**Ship**: `28%` of min edge (`HERO_EDGE_SHIP_FRACTION` / CSS `--zt-edge-ship`).

Chose **edge-length** (not area≥20%) so Ready compass + camera framing survive
short landscape while still reading at distance. Area≥20% would force ~45% sides
and crush the layout.

## Cull (Ít đồ)

Fewer controls so each can stay huge. Removed/gated simultaneous layers:
action-icon row, combo/freeze star glitter, dense wind/snow/equalizer,
traffic-light + cue stacking, word walls on Ready / pause / result / HUD labels.
Shell copy stays icon/shape + short labels (CAMERA / PREVIEW / START / BACK).

## Channels

- Picture (huge) + Kokoro `/voice/*.wav` + studio beds (duck under VO; freeze pauses bed).
- No player-facing instructional English on canvas/Ready.
