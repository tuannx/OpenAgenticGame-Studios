# Design

## Capability contract

`motion-capability.ts` is a dependency-light policy boundary. It converts a
small set of navigator hints into an immutable local profile containing device
class, local pose capacity, ideal capture size, and inference cadence. iPadOS in
desktop-UA mode is detected through `MacIntel` plus multiple touch points.

Both `main.ts` and lazily imported `vision.ts` consume this contract. TensorFlow
is not pulled into the launcher chunk merely to decide mode availability.

## Fail-closed mode ownership

The same capacity constrains every entry into a game mode:

- launcher cards, keyboard, Random, saved recommendations, and guide fallback;
- motion-navigation cycling and raised-hand confirmation;
- JavaScript bridge reads/writes from the Rust runtime;
- Rust result-screen navigation before the next run request.

The bridge clamps an unavailable Duo write to Mirror. Rust independently skips
Duo when both local evaluation slots are not enabled, so gameplay remains safe
even if a web caller is stale. Neither change adds a WASM import, so bridge ABI
version 5 remains valid.

## Presentation

The Duo card remains visible to preserve mode discoverability and the consistent
illustrated card set. In a one-pose profile it is muted, removed from the focus
order, marked `aria-disabled`, and explains that two-person camera support is
required. Random and recommendations draw only from available modes.

## Camera geometry

After video metadata is available, `vision.ts` reports normalized intrinsic
width, height, aspect, and orientation. `main.ts` applies that aspect to the one
camera-card coordinate space shared by video, overlay, framing reticle, and
status. Both media layers use `contain`, making crop impossible even during a
transient orientation mismatch.

Portrait setup uses a narrower bounded card so its increased height does not
collide with the ready panel. Raw frames and landmark geometry remain ephemeral.
