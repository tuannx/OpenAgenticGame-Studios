# Design

## Framework boundary

`brainbreak-core::MotionRuntime` is the reusable application layer. It owns
local pose recognizers and deterministic game state, accepts plain pose/action
frames, and exposes immutable player feedback. It has no Macroquad, browser,
TensorFlow.js, or networking dependency.

Browser adapters continue to produce normalized `PoseFrame` values. The
Macroquad composition root forwards remote actions and renders the returned
runtime state. A game can replace the renderer or feed alternative sensors
without changing recognition and scoring.

## Motion Reactor application

- Skeleton bones and joints are projected into a responsive stage.
- The current action maps to a spatial target zone.
- Active recognized actions light the player avatar before a score event.
- Hits emit bounded particles and shockwaves; misses emit a short red pulse.
- Camera pixels and landmarks never leave the local browser.

## Performance constraints

- No per-frame heap allocation in the framework update path.
- At most two local poses and four action players.
- Visual particles use a fixed-capacity pool.
- Frame delta remains clamped to 50 ms.

## Camera-first evaluation gate

The web adapter marks evaluation per player only while the camera is running
and that player has a sufficiently confident pose. `MotionRuntime` continues
the cue timeline for guide-only users but masks their actions before scoring,
hit/miss feedback, and P2P emission. This is an application policy expressed
through the framework input contract, not a renderer-only warning.
