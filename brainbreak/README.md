# BrainBreak Neon Beat Runner

An original neon endless runner built on the BrainBreak Macroquad/WebAssembly
motion framework. One or two people in a browser camera steer between three
lanes, jump hurdles, squat under gates, and clap to collect beat orbs. The
browser performs pose inference locally; only recognized action events are sent
through WebRTC.

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
    evaluation_enabled: [true, false, true, true],
    custom_target: None,
});

for player in runtime.players {
    // pose, active, triggered, hit, miss
}
```

This API has no Macroquad, TensorFlow.js, DOM, or Cloudflare dependency.
`brainbreak-game` is the Neon Beat Runner application adapter. It projects live
skeletons onto runner avatars while `RunnerGame` keeps lane changes, obstacles,
collision, lives, combo, scoring, and restart rules deterministic in the core.

The game is camera-first. Continuing without camera is a deliberately small
guide-only option: the rhythm highway and cues remain visible, while collision
evaluation, score, combo, lives, and outbound multiplayer actions stay disabled
for untracked local players. Keyboard and gamepad can preview actions but cannot
earn evaluated scores without camera tracking.

## Runner controls

| Camera action | Runner action |
| --- | --- |
| Lean or step left/right | Change one lane |
| Jump | Clear cyan hurdles |
| Squat | Pass under yellow gates |
| Clap | Collect purple beat orbs and restart after game over |

The first obstacle sequence teaches these verbs inline. Hazards combine distinct
shapes, colors, and text cues so critical information is not color-only.

## Music and audio-reactive lighting

The release locally hosts **“Special Spotlight” by Kevin MacLeod**, a 126 BPM
electronica track licensed under
[Creative Commons Attribution 4.0](https://creativecommons.org/licenses/by/4.0/).
The official source is
[Incompetech](https://incompetech.com/music/royalty-free/index.html?isrc=USUAN1600067&Search=Search).
The original MP3 was transcoded to 128 kbps for web delivery; full attribution,
modification notice, and SHA-256 are in `web/public/audio/ATTRIBUTION.md`.

Browser audio owns playback because autoplay requires a user gesture. The WASM
renderer reads a narrow beat-phase, beat-pulse, spectrum-energy, and playback
contract. Those metrics drive the neon sun, road grid, skyline windows, pylons,
obstacle glow, and particles. If music cannot start, gameplay continues with a
deterministic 126 BPM visual clock.

## Local development

```bash
npm install
npm run dev
```

Run the Cloudflare signaling API in another terminal when testing rooms:

```bash
npm run dev:worker
```

Open the HTTPS/localhost URL and press **Enable camera, music & run**. Keyboard
preview uses arrows/A/D to change lanes, Space/W to jump, Down/S to squat, and C
or Enter to clap. Standard gamepads map the left stick/D-pad to lanes and face
buttons to jump/squat/clap.

The visual authoring tool remains available at `/studio.html` for the underlying
motion framework.

## Validation

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
npm run typecheck
npm test
npm run build
```

Browser-facing changes also require an HTTP-served smoke test for JavaScript,
release WASM, TensorFlow model loading, camera permission, audio startup,
resize/orientation, focus recovery, and frame pacing.

## Cloudflare

The Worker serves static assets, creates expiring room codes through a Durable
Object, and brokers short-lived Cloudflare Realtime TURN credentials. Set
`TURN_KEY_ID` and `TURN_KEY_API_TOKEN` as Worker secrets before production
deployment. Pass `BUILD_SHA` as a deploy-time variable. TURN credentials are
issued only for a currently active room and are capped per room.
