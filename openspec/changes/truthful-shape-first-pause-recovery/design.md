# Design

## Presentation boundary

`pause_presentation` accepts `GameMode`, evaluated flags, and ready flags. Single
modes use the deterministic `any evaluated` / `any evaluated-and-ready` rule.
Duo uses required local P1/P2 independently. The function returns the shared
stack-only motion-prompt value with static copy and fixed figure states.

## Hierarchy

The panel remains a dedicated pause composition rather than reusing the Ready
card:

1. two large outlined pause bars;
2. `PAUSED`;
3. one next action;
4. one centered figure or two stable left/right player figures.

Warning, active, and ready tone may reinforce state, but the corner frame,
neutral stance, and raised arm/diamond carry meaning without color. P1 remains
left and P2 right.

## Failure and performance behavior

No image or browser dependency is added. Presentations, layout, and figure
positions are fixed stack values; rendering reuses bounded procedural helpers.
Missing evaluation maps to a return-to-frame action before any clap guidance.

## Compatibility and validation

`RunnerGame::update`, pause dwell, clap readiness, countdown, and loss
destinations remain authoritative and unchanged. Run strict Rust checks,
semantic/layout tests, release WASM, web build, HTTP MIME probes, and browser
startup/log inspection. Physical one/two-person pause recovery remains attended
QA.
