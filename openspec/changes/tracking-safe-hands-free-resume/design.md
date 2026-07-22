# Design

## Ownership

- `brainbreak-core` owns the `TrackingHold` and `Resuming` states, required-player
  readiness, countdown, and the freeze/clear invariants.
- `brainbreak-game` renders those states and keeps HUD, beat meter, and next cue
  hidden while progression is frozen.
- `bridge.ts` owns page-activity freshness. It clears local poses on inactivity
  and prevents the WASM evaluation import from reading stale snapshots.
- `main.ts` translates browser visibility lifecycle events into that bridge state.

## State machine

```text
Running -- required evaluation lost --> TrackingHold
TrackingHold -- evaluated ready signal(s) --> Resuming(2.0s)
Resuming -- countdown complete --> Running
Resuming -- required evaluation lost --> TrackingHold
```

`Ready` remains the first-run stance gate and `GameOver` remains the result
navigation state. Hold readiness uses the same evaluated action masks as the
existing ready flow; no browser-only shortcut can resume authoritative gameplay.

## Freeze and hazard policy

The core returns before action application, beat advancement, distance/scoring,
and obstacle collision while held or counting down. Entering hold clears the
fixed obstacle array and resets readiness. Clearing, rather than preserving,
trades a small amount of deterministic challenge continuity for physical safety:
the player never returns directly into a hazard they could not observe.

## Required evaluation

- Mirror and Strike require at least one evaluated player.
- Duo requires evaluated local players P1 and P2.
- Duo resume readiness requires a signal from both of those players.

## Browser freshness

`setPageActive(false)` clears `latestPoses`. `bb_evaluation_enabled` requires
camera evaluation, page activity, and current pose quality. Returning to a
visible page does not restore evaluation by itself; vision must publish a new
pose. This changes import behavior but not the ABI surface, so bridge version 5
remains valid.

## Fallback

Keyboard/gamepad actions continue through the same core masks but only count
while the corresponding player is camera-evaluated. If tracking cannot return,
the run remains safely frozen and the existing DOM camera controls remain the
explicit recovery surface.
