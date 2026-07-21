# Design

## Boundaries

- `brainbreak-core` owns deterministic rules, pose-to-action recognition,
  scoring, and versioned action identifiers.
- `brainbreak-game` is the Macroquad composition root and renderer.
- `web/src` owns browser camera/model, Web Audio, DOM onboarding, and WebRTC.
- `worker` owns Cloudflare static delivery, room signaling, and short-lived TURN
  credential brokering.

The browser bridge exposes bounded numeric snapshots to Rust. No raw camera
frame or landmark is sent over the network. P2P packets contain recognized
actions only.

## Durable decisions

- Macroquad 0.4.15 remains on its standard miniquad loader.
- TensorFlow.js MoveNet Lightning is the v1 pose provider.
- Cloudflare Durable Objects coordinate signaling only; WebRTC data channels
  carry gameplay traffic.
- The visual studio exports a versioned ZIP pack and never uploads source audio.
