# Proposal

## Problem

The deterministic countdown is correct, but its presentation is only a large
font character. It does not fulfill the taste guide's shape-dominant numeral
rule, offers no continuous timing cue within each second, and depends on font
rasterization at camera distance.

## Proposed change

Introduce a pure countdown presentation over remaining seconds. Use fixed
normalized line strokes for `2` and `1`, and twelve radial segments whose active
count follows the current numeral's remaining one-second fraction. The ring
refills exactly when the numeral changes, producing two visible preparation
beats without a second clock owner.

## Why no mode image

Canonical art is valuable in selection and recovery surfaces. During this
two-second transition, the player needs one dominant timing shape. The frozen
world and header preserve session continuity; a photo would compete with the
numeral and violate the established single-breath hierarchy.

## Non-goals

- No core, browser, asset, input, or audio change.
- No DOM animation or timer.
- No physical-readability claim from geometry alone.
