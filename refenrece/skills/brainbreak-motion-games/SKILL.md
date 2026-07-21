---
name: brainbreak-motion-games
description: "Build, extend, review, validate, and deploy camera-controlled BrainBreak games using the repository's Macroquad Rust/WASM stack. Use for pose/action mapping, camera-first evaluation, MoveNet, multiplayer action events, audio-reactive UX, content packs, or Cloudflare delivery under brainbreak/."
---

# BrainBreak Motion Games

Build motion games without weakening the distinction between a demonstrated body action and an ordinary fallback input. Keep deterministic gameplay in Rust, browser capabilities in TypeScript, and production claims tied to runtime evidence.

## Load the Required Context

1. Read the project `AGENTS.md`, `.codex/README.md`, and the routed workflow.
2. Read `refenrece/skills/macroquad-rust-wasm/SKILL.md` and `docs/engine-reference/macroquad/VERSION.md`.
3. Inspect only the BrainBreak surfaces affected by the task:
   - `brainbreak/crates/brainbreak-core/src/lib.rs`
   - `brainbreak/crates/brainbreak-game/src/main.rs`
   - `brainbreak/crates/brainbreak-game/src/visuals.rs`
   - `brainbreak/web/src/vision.ts`, `bridge.ts`, `audio.ts`, `network.ts`, and `main.ts`
   - `brainbreak/worker/src/index.ts`
4. Read the active record under `openspec/changes/` for non-trivial or cross-layer work.
5. Load [architecture-contract.md](references/architecture-contract.md) for layer or evaluation changes and [web-runtime-checklist.md](references/web-runtime-checklist.md) for browser or deployment work.

## Route the Change by Ownership

| Change | Primary owner | Evidence required |
|---|---|---|
| Actions, recognizers, scoring, runner rules | `brainbreak-core` | Rust unit tests and deterministic replayable inputs |
| Macroquad loop, rendering, effects | `brainbreak-game` | Native checks and release WASM build |
| Camera, MoveNet, audio, DOM, Rust imports | `brainbreak/web` | Typecheck, web tests, build, HTTP and browser smoke |
| P2P actions, signaling, rooms, TURN | `web/network.ts` and Worker | Protocol tests plus two-peer browser test |
| Caching, headers, production assets | Worker and deploy config | Content-type/cache probes against the deployed origin |

Do not move browser APIs, TensorFlow, networking transports, or Cloudflare types into `brainbreak-core`.

## Preserve the Runtime Contracts

- Treat `brainbreak-core` as an engine-independent deterministic domain. Pass pose snapshots, evaluation flags, elapsed time, and action events into it.
- Enforce camera-first evaluation inside the game runtime. Hiding a button or displaying a warning is not an evaluation boundary.
- When a local player is not tracked, do not award score, combo, lives, hit/miss judgment, or outbound multiplayer actions for that player.
- Evaluate remote action events only while the peer is connected. Send evaluated action events, never camera video or pose landmarks.
- Maintain stable tracked-player identities and independent recognizer state. Do not share calibration or temporal state between bodies.
- Keep raw video and landmarks local to the browser unless a future privacy design explicitly changes the contract.
- When changing JavaScript imports used by Rust, increment both the bridge plugin `version` in `web/src/bridge.ts` and `brainbreak_bridge_crate_version()` in Rust. Build and inspect the release WASM afterward.
- Start camera and audio only from a user gesture. Preserve a clear guide-only state when permission is denied or disabled.
- Store distributable music locally, retain license and attribution evidence, disclose modifications, and serve a fingerprinted asset.
- Clamp frame delta in the Macroquad loop and keep hot-loop effect storage bounded. The current reference clamp is `get_frame_time().min(0.05)`.
- Set immutable caching only after confirming that the response is the expected non-HTML asset. Never cache a SPA HTML fallback as JavaScript, WASM, or audio.

## Implement a Motion Mechanic

1. Define the motion vocabulary before UI polish: action name, landmark evidence, confidence threshold, dwell/cooldown, release condition, and multiplayer representation.
2. Add or extend a recognizer in the deterministic core. Test noisy frames, held poses, cooldowns, lost tracking, and multiple independent players.
3. Map actions into game state through an explicit adapter. Keep scoring and collision decisions out of TypeScript.
4. Add visualization that explains what the recognizer sees: skeleton, confidence/tracking state, current action, and actionable hint.
5. Make motion provenance visible. A keyboard/gamepad fallback may assist debugging, but do not describe fallback-triggered scoring as camera-detected motion.
6. Add audio-reactive rendering through stable analyzer values or a deterministic visual fallback. Do not couple game correctness to audio availability.
7. For multiplayer, transmit compact evaluated action events with player/time metadata; keep transport recovery separate from gameplay rules.

## Validate in Increasingly Real Environments

Run the smallest relevant checks first, then the complete skill gate before closeout:

```bash
bash refenrece/skills/brainbreak-motion-games/scripts/validate.sh brainbreak
```

Provide a production URL as the second argument to add safe delivery probes:

```bash
bash refenrece/skills/brainbreak-motion-games/scripts/validate.sh brainbreak \
  https://example.workers.dev
```

The gate validates formatting, Clippy, native tests, release WASM, TypeScript, web tests, the production bundle, audit status, artifact naming, and optional HTTP content types. It intentionally does not deploy.

After the automated gate, perform the real browser/device checks in [web-runtime-checklist.md](references/web-runtime-checklist.md). Keep these truth levels separate in the closeout:

1. native Rust behavior;
2. release WASM construction;
3. HTTP-served browser loading;
4. camera/audio interaction on a real browser;
5. deployed production behavior.

Do not claim a higher level from evidence at a lower level. If browser access or a second device is unavailable, state the exact unverified behavior.

## Close Out the Work

- Record non-trivial changes under `openspec/changes/{change-name}/`.
- Cite touched files and list exact validation commands.
- State camera, audio, multiplayer, and production evidence independently.
- Include the deployed commit SHA when publishing so `/health` and source history can be reconciled.
- Preserve known limitations such as missing TURN secrets, single-pose mobile fallback, or unavailable visual QA.

## Guardrails

- Do not make scoring depend solely on DOM state.
- Do not transmit raw pose or camera data as a shortcut for multiplayer.
- Do not silently change action thresholds without tests and calibration rationale.
- Do not claim music is "copyright free" without a redistribution-compatible license record.
- Do not use a successful Cargo build as browser-runtime proof.
- Do not deploy from an unreviewed dirty tree or report production success without probing the live origin.
