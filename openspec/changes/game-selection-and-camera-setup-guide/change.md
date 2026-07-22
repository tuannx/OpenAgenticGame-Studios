# Change: Hands-Free Game Selection and Camera Setup

**Date:** 2026-07-21  
**Target:** `brainbreak` web onboarding, motion navigation, runtime readiness, and browser delivery  
**Status:** implementation and automated/browser smoke complete; real-camera gesture QA pending

## Goal

Make the first BrainBreak screens readable at camera distance and reduce the play flow to one unavoidable camera-permission gesture followed by body-only navigation: lean to change game and hold a raised hand to confirm. Keep the art direction visually coherent and keep readiness authoritative in the deterministic Rust runtime.

## Milestone: M1 — One-Touch Permission, No-Touch Play Entry

### In scope

- Four image-led launcher cards: Random, Mirror Beat, Beat Strike, and Duo Groove.
- One neon cyber-party camera-placement image matching the game-card palette.
- Roving keyboard focus and arrow-key mode navigation as the accessible fallback.
- A deterministic TypeScript motion-navigation controller:
  - body lean changes mode once, then requires a neutral re-arm;
  - one-second raised-hand dwell confirms;
  - tracking loss resets dwell;
  - Duo requires two tracked players with both hands raised.
- A live-camera stance screen with large distance-readable instructions and progress feedback.
- Runtime readiness keyed to the selected `GameMode`; Rust does not start a run from tracking presence alone.
- Bridge schema/plugin version 4 with `bb_game_mode()` on both JavaScript and Rust sides.
- Macroquad 0.4.15 loader hardening for its strict-mode `quad_net` declaration defect.
- Responsive launcher layouts for phone portrait and desktop/TV landscape.

### Out of scope

- Changing recognition thresholds used for authoritative gameplay scoring.
- Deploying a new production build.
- Re-validating TURN, two-peer WebRTC, or production cache headers.
- Session-to-session personalization and learned theme/game ordering; those remain later milestones in the active taste objective.

### Touched systems

- Deterministic runtime: `brainbreak-core::RunnerGame`
- Macroquad adapter and bridge ABI: `brainbreak-game`, `bridge.ts`
- Browser capability/UI adapters: `main.ts`, `motion-navigation.ts`, `vision.ts`
- Launcher and responsive presentation: `index.html`, `style.css`, `web/public/assets/`
- Release tooling: `scripts/build-wasm.sh`, BrainBreak validation skill

### Acceptance criteria

1. The launcher shows four coherent neon image cards and a clear camera-placement visual without viewport overflow at 1440×784 and 390×844.
2. After the explicit camera/audio permission gesture, a player can change mode by leaning and confirm by holding a raised hand for one second; no second touch is required.
3. Tracking loss cancels confirmation, and Duo cannot confirm or start with only one tracked player.
4. Rust remains the evaluated gameplay boundary and does not start a session merely because a player is visible.
5. JavaScript and Rust agree on bridge version 4 and selected game mode.
6. The release loader parses in a modern browser, the current WASM loads over HTTP with `application/wasm`, and the boot overlay exits.

### Fallback

- A small touch-to-start button remains available for accessibility or recognition failure.
- Guide-only mode remains available and cannot score, evaluate collisions, build combo, or emit multiplayer actions.

## Art and payload

- Used assets are JPEG files with matching file/MIME types:
  - `game_mode_random.jpg`
  - `game_mode_mirror.jpg`
  - `game_mode_strike.jpg`
  - `game_mode_duo.jpg`
  - `phone_placement_guide.jpg`
- Assets were resized from 1024px to 640px. The five used files total about 800 KiB instead of about 4 MiB.
- Four unused pastel setup/ready variants were removed because they conflicted with the neon art direction and added about 2.5 MiB to the public payload.

## Validation evidence

### Automated gates

- `cargo fmt --all -- --check` — passed.
- `cargo test --all-targets` — 19 Rust tests passed.
- `npm run typecheck` — passed with strict TypeScript.
- `npm test` — 10 web tests passed across 4 files, including motion navigation and bridge registration.
- `npm run build:web` — release WASM and Vite production bundle passed.
- `node --check web/public/mq_js_bundle.js` — passed after strict-mode hardening.
- `bash refenrece/skills/brainbreak-motion-games/scripts/validate.sh brainbreak` — all four gates passed after adding the loader regression gate; output artifact `brainbreak-game-1e76c5fb3e76.wasm`.
- `git diff --check` — passed at closeout.

### HTTP and browser smoke

- Local HTTP responses:
  - HTML: `text/html`
  - launcher/guide art: `image/jpeg`
  - fingerprinted WASM: `application/wasm`
- Chrome at 1440×784: all four cards, setup image, camera CTA, no-touch hint, and guide-only fallback fit in one launcher card.
- Chrome at 390×844: no horizontal/body overflow; launcher card measured 366px wide and 701px high within the viewport.
- ArrowRight moved the checked/roving focus state from Random to Mirror (`aria-checked=true`, `tabindex=0`).
- A first browser pass exposed the Macroquad loader strict-mode error. After generator hardening and a clean server restart, the origin emitted no browser error/warn, the WASM loaded, and `#boot-screen` became hidden.

### Residual verification gap

The browser session did not grant camera permission, so real-person lean direction, one-second dwell feel, mirrored skeleton alignment, one-to-two-player transition, camera/audio unlock, and physical distance readability remain unverified. This is an explicit interactive QA gap, not inferred success from tests.

## Next dependency

M2 should add session feedback capture and taste learning: persist chosen mode/theme, track setup retries and time-to-ready without raw pose data, and use those signals to simplify the next session while keeping privacy local-first.
