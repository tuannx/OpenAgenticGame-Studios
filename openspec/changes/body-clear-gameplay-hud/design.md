# Design

## One running-surface layout

Derive `RunningSurfaceLayout` from viewport dimensions and the already computed
displayed-player count:

- `hud` retains the existing bounded responsive player-card layout;
- `stage` ends a fixed visual gap above the HUD's first row;
- `deck` fills the remaining lower canvas with a quiet opaque control plane and
  one low-emphasis neon boundary.

`RunnerStage` receives the reduced stage rectangle only during `Running`.
Ready, countdown, pause, tracking hold, and result continue using the full canvas
because their player HUD is hidden and their overlays already own hierarchy.

## Invariants

- The nearest grounded runner, its glow, and its foot identity remain above the
  first HUD row at 390x844, 667x375, and 1440x784 for one through four cards.
- HUD semantics, typography, life shapes, combo threshold, and displayed-player
  policy remain unchanged.
- The deck is passive presentation: no input, state, allocation, timer, asset,
  browser import, or deterministic rule is introduced.
- Crash shake keeps bounded clearance and does not move the HUD.
- Camera PIP and action beacon remain unchanged and are not claimed fixed.

## Fallback

If a target viewport cannot keep the runner and HUD separated, reduce the world
height or HUD card height within existing readability bounds. Do not restore an
overlapping glass card or hide score/lives during active play.
