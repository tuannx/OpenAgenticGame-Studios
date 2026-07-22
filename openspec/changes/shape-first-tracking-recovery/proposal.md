# Proposal

## Problem

Tracking loss currently renders through the same generic text component as the
start countdown and titles itself `RUN PAUSED`. That collides with the explicit
intentional `PAUSED` state, hides which Duo player is missing, relies on a bullet
glyph for partial readiness, and drops the mode illustration at a stressful
session boundary.

## Proposed change

Add a pure recovery presenter driven by the deterministic evaluated and ready
arrays. Render it through the same canonical art, responsive composition, and
procedural motion figures established by M15. Missing players receive a framed
search silhouette, evaluated players stand neutrally, and latched players raise
a hand with a diamond marker.

The generic status renderer becomes countdown-only. This makes initial/resume
countdown and sensor recovery two explicit presentation roles without changing
their deterministic state-machine ownership.

## Non-goals

- No recognizer, tracking, countdown, gameplay, or browser changes.
- No duplicated mode asset or new art style.
- No claim that automated geometry proves room-distance recovery timing.
