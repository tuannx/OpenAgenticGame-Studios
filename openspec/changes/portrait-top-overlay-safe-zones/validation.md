# Validation

## Pre-change evidence

- The M32 production camera path at 390x844 showed one camera-evaluated player,
  but the 280px action beacon and 100px PIP shared the upper-right pixels. The
  settings trigger also touched the beacon's upper-left edge.
- Current Rust geometry centers a 280px non-compact cue at `x=55` on a 390px
  canvas. Current portrait CSS places the PIP at right 14px/top 66px and the
  settings trigger at left 14px/top 66px, proving the collision without relying
  on an inferred screenshot alone.
- At 667x375 and 1280x720, horizontal space already separates these owners; the
  problem is specifically narrow portrait.

## Final evidence

Pending implementation and validation.
