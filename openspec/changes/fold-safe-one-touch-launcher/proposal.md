# Proposal

## Problem

The launcher promises that enabling the camera is the player's one required
touch. At 667x375, the mobile two-column mode grid becomes two rows and pushes
the camera CTA entirely below the fold. The player must discover and perform an
unexplained scroll before the promised single action.

## Proposed change

Treat short landscape as a dedicated launcher composition. Keep all four
canonical mode images in a one-row rail, remove the redundant placement image
that is already hidden at this height, tighten only secondary copy, and reserve
an always-visible action region for the camera CTA and guide-only escape route.

## Why now

M19 proved camera-ready composition at this same viewport. The next upstream
boundary is launcher entry, where a hidden primary action prevents the player
from reaching every no-touch surface that follows.

## Non-goals

- No mode-selection, recommendation, capability, camera, or gameplay changes.
- No new image, tutorial, modal, persistence signal, or camera permission call.
- No removal of the honest guide-only path or local-only privacy copy.

