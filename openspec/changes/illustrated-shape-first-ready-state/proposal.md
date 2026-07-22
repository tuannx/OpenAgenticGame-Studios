# Proposal

## Problem

The launcher and camera setup use a consistent neon illustration set, but the
deterministic Ready screen drops to a flat text panel. Its long messages allocate
owned strings every frame and rely on emoji glyphs whose rendering varies by
platform. This weakens both style continuity and far-field no-touch guidance.

## Proposed change

Introduce a pure Ready-presentation mapper driven only by current mode,
guide-only state, tracked-player count, and the two deterministic ready flags.
Render its static copy beside the canonical selected-mode texture and draw
procedural camera/player silhouettes. A raised arm and diamond marker identify
ready state by shape, while outlined search framing identifies a missing player.

## Why now

M14 established a bounded shared mode-art owner for the deterministic canvas.
Ready can now gain visual continuity without another asset pipeline or decoder,
making it the lowest-risk next screen in the hands-free session path.

## Non-goals

- No change to runtime facts or transition thresholds.
- No new browser import, interaction, asset, or decode path.
- No inference that automated containment proves physical-room readability.
