# Design

## Ownership

- `brainbreak-core` owns active elapsed time, outcome, result transition, and the
  one-shot result event.
- `brainbreak-game` consumes deterministic progress/outcome for presentation and
  forwards the event through one browser import.
- `brainbreak/web/src/bridge.ts` owns import registration and typed handler
  normalization.
- `taste-profile.ts` owns bounded local aggregates and recommendation weighting.

## Deterministic ordering

After pause and tracking gates prove that the phase remains `Running`, advance
the active clock with the already-clamped delta. If it reaches 90 seconds, call
one `finish_run(BreakComplete)` transition and return immediately. This ordering
prevents the boundary frame from applying actions, spawning/advancing hazards,
awarding passive score, or consuming energy.

Energy depletion uses the same finish function with `EnergySpent`. The function
clears hazards, marks personal bests, restores the current result selection, and
queues one immutable result event. `take_result_event()` provides single-owner,
exactly-once consumption at the composition root.

## Presentation

Keep the current beat rail and add a quieter, shape-only session rail next to it
rather than introducing another instruction sentence. The result title is
`BREAK COMPLETE` for the full time-box and `NICE RUN` for depleted energy. The
existing three mode chips and lean/clap contract do not change.

## Aggregate learning and ABI

Bridge v7 adds `bb_record_run_outcome(mode, outcome)`. The browser accepts only
known mode/outcome values, then increments bounded per-mode completions and one
coarse outcome counter. Recommendation keeps the existing two-start stability
threshold but gives completed runs more weight than starts.

The taste schema remains backward-compatible: missing new fields restore safe
zero defaults while existing theme, overlay, opacity, and setup history survive.

## Fallback

If the new browser import fails, the synchronized ABI check must fail rather
than silently dropping learning. Native gameplay and outcome tests remain
independent from browser storage. A storage read/write failure continues in
memory and never blocks result navigation or replay.
