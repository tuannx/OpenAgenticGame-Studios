# Design

## Ownership

`RunnerGame` remains the sole owner of judgment and emits the existing
`RunnerFeedback` value for one update. `RunnerStage` may retain that immutable
event only as bounded visual presentation state. It cannot affect score, lives,
combo, collision, readiness, or outbound actions.

The Rust composition root owns the bridge call and sends the judged player's
current combo with the feedback kind. The browser audio adapter owns synthesis.
Because the import signature changes, the JavaScript plugin version and
`brainbreak_bridge_crate_version()` move together from v5 to v6.

## Visual hierarchy

The action beacon remains the dominant “what next” signal. Outcome markers form
a temporary centered row between it and the HUD: check + `NICE`, spark + `BEAT`,
or cross + `NEXT`. Player color identifies ownership, but outcome meaning also
exists in geometry and text. Reduced-motion mode keeps scale fixed while
preserving the static shape and fade.

The HUD removes prose. Each player card retains a readable player/score line,
three outlined life pips whose lost state has an X, and a diamond combo badge
only from x2. This keeps persistent state subordinate to the next action.

## Audio timing

`combo_pitch_ratio(combo)` is a pure clamped mapping: one semitone per five
combo, maximum four semitones. It multiplies only the success feedback oscillator
frequencies. The music element is never rate-shifted, preserving the declared
126 BPM and analyzer/beat-phase contract.

## Milestone M10

Goal:

- Make evaluated outcomes glanceable and rhythmically rewarding without adding
  cognitive load or changing deterministic rules.

Execution slices:

- Add bounded typed feedback pulses and shape-first markers.
- Simplify the persistent HUD into score, life pips, and meaningful combo.
- Migrate feedback ABI to v6 and correct combo pitch ownership.
- Add focused Rust/TypeScript tests and persist the learned taste rule.

Out of scope:

- Core rule/balance changes, new art assets, deployment, or fake camera input.

Touched systems:

- Macroquad rendering, Rust WASM composition root, browser bridge/audio, tests,
  and BrainBreak taste guidance.

Acceptance criteria:

- Distinct bounded feedback semantics and responsive containment.
- Simplified HUD with non-color life state and conditional combo.
- Synchronized bridge v6 carrying actual combo.
- Combo-aware SFX with invariant backing-track playback rate.

Validation package:

- Focused Rust and Vitest checks, full native/Clippy gates, release WASM, web
  build, and an HTTP-served browser load with import/console/content-type checks.

Persona review:

- Security, performance, readability, accessibility, and runtime regression.

Fallback note:

- If simultaneous markers crowd the smallest landscape viewport, preserve the
  typed pulse and audio ABI but reduce marker diameter; never convert outcomes
  back to color-only particles.

Next dependency:

- A permissioned camera/audio session can tune visual scale, SFX mix, and pulse
  duration without reopening deterministic ownership.
