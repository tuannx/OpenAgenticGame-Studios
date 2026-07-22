# Proposal

## Problem

The ready screen is entered after the player has chosen camera intent and begun
moving away from the device. Its current action layer still uses font emoji,
three competing instructions, a linear progress strip, and a visible percentage
rebuilt on pose updates. The canonical mode image is consistent, but the physical
action is not first-glance geometry.

## Proposed change

Introduce a typed ready-navigation presenter and a single action compass. The
presenter gives framing corrections priority, then exposes left/right change,
raised-hand hold, or neutral-ready as a closed visual state. The DOM renders two
CSS chevrons around a central vector raised-hand ring, while the existing mode
image preserves visual continuity from launcher to setup.

## Why now

M14-M18 made result, runtime Ready, recovery, pause, and countdown shape-first.
The browser camera-ready handoff is now the remaining high-frequency surface
where the player is physically distant but the UI still depends on emoji and
sentence parsing.

## Non-goals

- No recognizer, framing, timing, capability, or gameplay changes.
- No additional tutorial, modal, camera touch, image file, or analytics signal.
- No replacement of the real camera preview with static illustration.
