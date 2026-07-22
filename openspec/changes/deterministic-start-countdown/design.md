# Design

## Ownership

- `brainbreak-core` owns readiness, the `Starting` and `Resuming` phases,
  countdown time, tracking-loss transitions, and progression freeze.
- `brainbreak-game` renders one shared countdown treatment for both phases and
  keeps gameplay-only presentation hidden.
- The browser setup gate continues to own camera permission, framing, and mode
  selection. It does not own or mirror the runtime countdown.

## State machine

```text
Ready -- evaluated ready signal(s) --> Starting(2.0s)
Starting -- countdown complete --> Running
Starting -- required evaluation lost --> Ready

Running -- required evaluation lost --> TrackingHold
TrackingHold -- evaluated ready signal(s) --> Resuming(2.0s)
Resuming -- countdown complete --> Running
Resuming -- required evaluation lost --> TrackingHold

GameOver -- evaluated clap/replay --> Starting(2.0s)
```

`Starting` is intentionally distinct from `Resuming`. This makes the recovery
destination explicit: a run that has not begun returns to its initial ready
gate, while an interrupted active run returns to the safety hold that clears
in-flight hazards.

## Timing and freeze contract

The shared countdown duration stays at two seconds. Entering either countdown
returns immediately. Countdown frames only decrement bounded time and return;
the frame that reaches zero changes phase to `Running` but still does not apply
actions or advance the world. Progression begins on the next frame.

## Presentation

Both countdown phases render `GET READY` plus a shape-dominant `2` or `1` in the
existing centered status panel. HUD cards, next-action cues, outcome markers,
and beat meter remain gameplay-only. No new image or text layer is introduced,
which preserves the established neon visual language and reduced-motion path.

## Fallback

If required tracking disappears before the first run, reset readiness and wait
in `Ready`. If it disappears during tracking recovery, retain the existing
`TrackingHold` behavior. Guide-only mode remains non-evaluated and cannot start
the countdown through keyboard or gamepad input alone.
