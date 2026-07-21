# BrainBreak Architecture Contract

Use this reference when a change crosses Rust core, Macroquad, browser adapters, or multiplayer delivery.

## Layer Map

| Layer | Owns | Must not own |
|---|---|---|
| `brainbreak-core` | `PoseFrame`, actions, recognizers, `MotionRuntime`, deterministic game state such as `RunnerGame` | Macroquad drawing, DOM, TensorFlow.js, WebRTC, Cloudflare APIs |
| `brainbreak-game` | Macroquad composition loop, rendering, visual effects, Rust-to-browser import calls | Camera permission flows, pose inference, signaling |
| Browser adapters | MoveNet inference, normalized poses, ABI bridge, Web Audio, controls, action transport | Authoritative score/collision rules |
| Worker | Static delivery, security/cache headers, room signaling, optional TURN credentials | Pose recognition or game simulation |

The intended data flow is:

```text
camera -> MoveNet -> normalized pose snapshots -> Rust bridge
       -> MotionRuntime recognizers + evaluation mask -> game state -> render/SFX
       -> evaluated action events only -> WebRTC peer
```

## Evaluation Invariants

- A player contributes evaluated gameplay only while that player's camera tracking quality passes the runtime threshold.
- Lost tracking clears or suppresses held recognizer state before it can create repeated judgments.
- Players have independent calibration, temporal history, cooldowns, and action masks.
- An untracked local player receives no score, combo, lives, hit/miss judgment, or multiplayer action emission.
- A remote player is evaluated only while the data channel is connected and only from received evaluated events.
- The browser warning and disabled-control appearance explain the state; the Rust evaluation boundary enforces it.

The current game combines keyboard/gamepad fallback with camera actions after camera evaluation becomes active. Therefore fallback input can still cause evaluated gameplay while the camera tracks a player. Do not claim every scored action is camera-derived until source provenance is represented separately in the core input model.

## ABI and Versioning

Macroquad discovers browser imports through a JavaScript plugin. A bridge import change is complete only when all of these move together:

1. the import implementation and plugin version in `brainbreak/web/src/bridge.ts`;
2. the Rust `extern` declaration and `brainbreak_bridge_crate_version()` value;
3. TypeScript tests for registration/import behavior;
4. a release `wasm32-unknown-unknown` build;
5. an HTTP browser load with no missing import or plugin-version error.

A native build cannot exercise this ABI.

## Player Identity and Capacity

- Preserve stable tracking IDs across frames; array position alone is not a durable identity when bodies cross.
- Allocate recognizer state per identity and expire it deliberately after loss.
- The current product target supports up to two local people in one camera and up to two remote participants. Treat device/model limitations as explicit capacity degradation rather than silently merging bodies.
- Mobile devices may require a single-pose model or lower inference rate. Keep gameplay timing stable by decoupling render cadence from pose update cadence.

## Testing Seams

- Test recognizers from synthetic `PoseFrame` sequences without Macroquad or a browser.
- Test game state from explicit actions, evaluation flags, and delta time.
- Test bridge and network serialization in TypeScript.
- Test content types and cache policy at the Worker boundary.
- Reserve a real browser for permission, camera framing, skeleton alignment, audio unlock, WebRTC, resize, and frame-pacing truth.
