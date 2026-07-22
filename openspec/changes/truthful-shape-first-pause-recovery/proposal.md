# Proposal

## Problem

The deterministic pause state correctly requires evaluated claps and clears a
missing player's ready flag, but the renderer reads only readiness. A missing
single player still sees `CLAP TO RESUME`; Duo can see the same prompt even when
one required body is outside evaluation. Partial readiness is encoded in a long
bullet-separated sentence rather than the shape language now used elsewhere.

## Proposed change

Add a pure pause presenter over the core's evaluated and ready arrays. Retain
the established two-bar pause icon and render one/two shared motion figures
beneath one concise next action. Searching frames explain reacquisition,
neutral bodies mean tracked, and raised hands plus diamonds mean clap latched.

## Why no mode photo

Ready, tracking recovery, and results benefit from canonical mode imagery.
Intentional pause is a control/safety state with a universal icon that must win
the first glance. M12 explicitly excluded photographic art for this reason; M17
keeps that decision while reusing the same neon vector palette and player shapes.

## Non-goals

- No gameplay, recognizer, camera, audio, asset, or ABI change.
- No settings modal or touch resume control.
- No automated claim about physical pause/recovery ergonomics.
