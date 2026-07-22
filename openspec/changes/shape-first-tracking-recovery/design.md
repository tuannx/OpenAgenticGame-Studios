# Design

## Runtime truth boundary

The presenter accepts `GameMode`, `[bool; PLAYER_CAPACITY]` evaluated facts, and
the core's ready array. It does not use DOM visibility or raw pose count. For
single-player modes, the same `any evaluated` / `any evaluated-and-ready` facts
as the deterministic runtime select missing, restored, or signal-received
presentation. Duo reads required P1/P2 facts independently.

## Shared presentation language

M15's Ready-only visual value types are generalized into a motion-prompt
presentation, layout, tone, and figure vocabulary. Ready and TrackingHold each
own a pure state mapper while sharing only rendering primitives. This preserves
one visual grammar without merging the two state semantics.

Recovery reuses the selected mode texture through the existing bounded `ModeArt`
owner and center-cover helper. A missing texture keeps the procedural neon field
and all recovery figures/copy.

## State communication

- No required evaluation: `FIND YOUR FRAME` or the specific missing Duo player,
  with a corner-framed search figure.
- Required evaluation restored but not signaled: neutral figure plus one concise
  raised-hand/clap instruction.
- Ready latched: raised arm and diamond marker; the core normally advances to
  `Resuming` immediately, but the mapping remains exhaustive and testable.

Recovery never uses `PAUSED`, which remains exclusive to intentional pause.

## Performance and compatibility

- Presentations contain only static slices, enums, and fixed arrays.
- Mode textures remain loaded once and borrowed.
- The frame loop adds no string, collection, image, or texture allocation.
- No core or browser file changes; ABI remains v7.

## Validation impact

Run strict native Rust checks, semantic/layout tests, release WASM, production
web build, HTTP content-type probes, and browser startup/log inspection. A real
camera session that loses and reacquires one/two bodies remains required for
timing and physical-distance proof.
