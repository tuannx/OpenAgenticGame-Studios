# Proposal: Neon Beat Runner

## Intent

- Deliver an original endless runner controlled by camera-detected body motion.
- Make music, lighting, scenery, particles, hazards, and feedback move as one coherent rhythm experience.

## Why This Change

- Motion Reactor demonstrates framework capability but does not yet provide a familiar game loop or strong replay hook.
- A three-lane runner naturally maps lateral movement, jump, squat, and clap to understandable full-body verbs.

## In Scope

- Three-lane automatic runner with deterministic obstacle patterns, scoring, combo, lives, game over, and immediate restart.
- One or two people in one camera plus existing remote-player event inputs.
- Live pose visualization mapped into neon runner avatars.
- Locally hosted CC BY 4.0 dance track with in-product and repository attribution.
- Beat/audio-energy synchronized palette, lights, road grid, particles, and obstacle pulses.
- Guide-only enforcement, sound control, reduced-motion control, and responsive HUD.
- Cloudflare deployment and runtime asset verification.

## Out Of Scope

- Copied Temple Run characters, environments, names, logos, music, or level layouts.
- Accounts, purchases, persistent progression, global leaderboard, or production TURN provisioning.
- Artist-authored 3D models or a full 3D physics engine.

## Acceptance Criteria

1. Camera actions move lanes, jump, slide, and activate a clap pulse against readable runner obstacles.
2. Players without camera-backed evaluation cannot earn score or emit evaluated multiplayer actions.
3. Music starts only after a user gesture and the scene visibly reacts to its 126 BPM clock and analyzed energy.
4. The UI exposes license attribution and supports mute plus reduced-motion modes.
5. Native Rust checks, release WASM, web tests/build, HTTP smoke, and production checks pass.

## Success Criteria

- A new player can understand the primary actions from the first obstacle sequence without a blocking tutorial.
- Camera movement produces immediate visual avatar/action feedback and changes gameplay outcome.
- The deployed build loads its JavaScript, WASM, model, and audio assets without MIME or cache fallback errors.
