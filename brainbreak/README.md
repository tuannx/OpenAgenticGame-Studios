# BrainBreak Motion Party

Macroquad/WebAssembly party games controlled by one or two people in a browser
camera. The browser performs pose inference locally; only recognized action
events are sent through WebRTC.

Live build: <https://brainbreak-motion-party.tuannx87.workers.dev>

## Framework API

`brainbreak-core` is the renderer-independent framework layer. A host supplies
normalized poses plus fallback/remote actions and reads one stable snapshot:

```rust
let mut runtime = MotionRuntime::new(GameMode::MirrorBeat, RecognizerConfig::default());
runtime.update(delta_seconds, MotionInputFrame {
    local_poses: [camera_pose, None],
    fallback_actions: [keyboard_mask, 0],
    remote_actions: [remote_p1, remote_p2],
    custom_target: None,
});

for player in runtime.players {
    // pose, active, triggered, hit, miss
}
```

This API has no Macroquad, TensorFlow.js, DOM, or Cloudflare dependency. Motion
Reactor in `brainbreak-game` is the first application adapter: it projects live
skeletons into spatial target zones and turns recognized actions into score,
shockwave, and particle feedback.

## Local development

```bash
npm install
npm run dev
```

Run the Cloudflare signaling API in another terminal when testing rooms:

```bash
npm run dev:worker
```

Open the HTTPS/localhost URL, press **Start camera**, and use `1`, `2`, or `3`
to select Mirror Beat, Beat Strike, or Duo Groove. Keyboard fallback uses
arrows/WASD, Q/E, Space, and C. Standard gamepads map the left stick/D-pad to
movement, face buttons to jump/squat/clap, and shoulder buttons to raised hands.
The backing pulse is generated locally with Web Audio and does not require a
licensed audio asset.

The visual authoring tool is available at `/studio.html`. Exported
`.brainbreak.zip` packs can be imported from the game control panel.

## Validation

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
npm run typecheck
npm test
npm run build
```

## Cloudflare

The Worker serves static assets, creates expiring room codes through a Durable
Object, and brokers short-lived Cloudflare Realtime TURN credentials. Set
`TURN_KEY_ID` and `TURN_KEY_API_TOKEN` as Worker secrets before production
deployment. Pass `BUILD_SHA` as a deploy-time variable. TURN credentials are
issued only for a currently active room and are capped per room.
