# Design

## Presentation mapping

`countdown_presentation(remaining)` returns `CountdownNumeral::{Two, One}` and
a clamped fraction for the current numeral second:

- `remaining > 1`: numeral `Two`, fraction `remaining - 1` clamped to 0...1;
- `remaining <= 1`: numeral `One`, fraction `remaining` clamped to 0...1;
- non-finite/negative input fails safely to `One` at zero.

This avoids duplicating the core's total countdown-duration constant. The
presentation reads remaining time and never writes phase or time.

## Vector language

Each numeral owns a fixed array of normalized strokes. `Two` uses five strokes;
`One` uses a centered hook, stem, and base. Thick white strokes with rounded
endpoints remain recognizable independent of font coverage.

Twelve fixed radial ticks surround the numeral. Active tick count is
`ceil(fraction * 12)`, bounded to 0...12. No collection grows and no string is
formatted in the frame loop.

## Layout and hierarchy

A pure layout returns the centered panel, title baseline, ring center/radius,
and stroke scale. Compact landscape uses a shorter panel/ring. `GET READY` is the
only text; numeral and time are geometric. HUD, action cues, feedback, and beat
meter remain Running-only.

## Compatibility and validation

`RunnerGame` timing and Starting/Resuming/PauseResuming transitions remain
untouched. Run strict native checks, pure mapping/layout tests, release WASM,
production web build, HTTP MIME probes, and browser startup/log inspection. A
physical one/two-person entry/resume run remains required for timing feel.
