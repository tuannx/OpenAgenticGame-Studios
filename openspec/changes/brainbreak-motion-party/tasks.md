# Tasks

## M1 - Foundation

- [x] Scaffold Rust workspace and deterministic core.
- [x] Render all three game modes in Macroquad.
- [x] Build native tests and release WASM.

## M2 - Motion input

- [x] Add MoveNet browser capture for one mobile or two desktop players.
- [x] Add calibration, pose quality, keyboard, and gamepad fallbacks.
- [x] Keep the bridge bounded and camera data local.

## M3 - Content studio

- [x] Add graph/timeline editor, validation, and ZIP export/import.
- [x] Prove imported target sequences affect runtime without a Rust rebuild.

## M4 - P2P and Cloudflare

- [x] Add room-code signaling with a Durable Object.
- [x] Add WebRTC action transport and room-gated TURN credential endpoint.
- [x] Add security headers, room expiry, and health endpoint.

## M5 - Release

- [x] Run Rust, TypeScript, Worker, WASM, and HTTP/protocol gates.
- [x] Deploy the first production Worker to Cloudflare Workers.dev.
- [x] Verify the production URL and publish the feature branch.
