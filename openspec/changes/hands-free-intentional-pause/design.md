# Design

## Motion vocabulary

`Pause` is a meta/control action, not a scoreable gameplay action. It becomes
active only when both wrists are confidently above their corresponding
shoulders and their separation is at least the clap threshold for 1000 ms.

The dwell rejects a short arms-up jump. Requiring separated wrists rejects an
overhead clap. Releasing either hand or losing tracking resets the dwell. The
existing rising-edge/cooldown machinery emits one trigger per hold.

## Ownership

- `PoseRecognizer` owns pause evidence and dwell state.
- `MotionRuntime` exposes the action but filters it before `GameState` scoring.
- `RunnerGame` owns pause transitions, freeze, hazard clearing, and resume
  readiness.
- `brainbreak-game` owns the vector pause icon and responsive presentation.
- Browser pose transport already carries landmarks and action masks, so no ABI
  or bridge version changes are required.

## State machine

```text
Running -- evaluated Pause --> Paused
Paused -- evaluated clap(s) --> PauseResuming(2.0s)
PauseResuming -- countdown complete --> Running
PauseResuming -- required evaluation lost --> Paused

Running -- required evaluation lost --> TrackingHold
TrackingHold -- evaluated ready signal(s) --> Resuming(2.0s)
Resuming -- required evaluation lost --> TrackingHold
```

Intentional pause and tracking hold share freeze safety but not recovery copy or
loss destinations. Entering `Paused` clears fixed-capacity hazards, resets ready
flags, and returns before gameplay progression.

## Duo readiness

Paused readiness accepts a clap trigger only. In Duo, P1 and P2 readiness is
latched independently while each remains evaluated; a missing player clears
that player's readiness. Both must be ready before `PauseResuming` starts.

## Presentation

The paused overlay uses two large outlined bars, `PAUSED`, and a single clap
instruction. Duo may replace that instruction with one short player-specific
prompt after the first clap. The icon and text reuse the existing cyan/purple
neon vector language; a photographic mode image would add visual competition to
a safety/control state.

## Fallback

If the pause pose proves unreliable in attended QA, retain the typed paused
state and tune only the dwell/geometry thresholds with new fixtures. Do not
replace it with a DOM timer or weaken camera evaluation.
