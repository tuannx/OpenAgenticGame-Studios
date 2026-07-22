# BrainBreak — Neon Beat Runner

> **One line**: Your body is the controller. Stand up, move to the beat, feel the neon. 90 seconds. Zero touch.

**Live**: https://brainbreak-motion-party.tuannx87.workers.dev

**What makes this different**: No controller. No app. No account. Open a link, turn on camera, step back — your body IS the game. A 90-second neon rhythm runner that turns any room into a motion party.

---

## Play Anywhere

| Context | Distance | Setup | Best For |
|---------|----------|-------|----------|
| Living room / TV | 1.5–2.5m | Phone sideways on table | Family party, 2-player |
| Classroom / Kids | 2.0–4.0m | Projector + laptop | Group energizer, big moves |
| Desk micro-break | 0.8–1.2m | Laptop webcam | Solo stretch, small moves |

The framing coach adapts to your distance automatically. Just get your full body in frame.

---

## Player Journey (5 Steps, 15 Seconds to Play)

```
OPEN LINK → PICK MODE → TAP CAMERA → STEP BACK + RAISE HAND → PLAY
```

| Step | User Action | System Response |
|------|-------------|-----------------|
| 1 | Open URL | Launcher shows 4 big picture cards |
| 2 | Pick mode (or Random) | Card highlights, setup illustration appears |
| 3 | **One tap**: "TURN ON CAMERA" | Camera activates, vision model loads |
| 4 | Step back 1.5–2.5m, raise a hand | Framing coach → hold-ring fills → countdown 2…1 |
| 5 | Move! | Neon runner begins, music drops |

**After this one tap, everything is hands-free:**
- Lean ← → to choose
- Raise hand (hold 1s) to confirm
- Clap to replay
- Both hands up to pause

---

## Game Modes

| Mode | Badge | Players | Tempo | Feel |
|------|-------|---------|-------|------|
| **Mirror Beat** | CLASSIC | 1 | 1.45s/beat | Groovy, follow-the-leader poses |
| **Beat Strike** | SPEED | 1 | 1.0s/beat | Fast reactions, lane dashes |
| **Duo Groove** | 2 PLAYERS | 2 (same camera) | 1.25s/beat | Co-op sync, Party Overdrive ×2 |
| **Random Fun!** | RANDOM | 1–2 | Mixed | Surprise mode pick |

Duo unlocks when camera detects 2 players in frame.

---

## Controls (Body = Controller)

| Body Move | Game Action | Visual Cue |
|-----------|-------------|------------|
| Lean / step left-right | Change lane | Cyan arrow |
| Jump | Clear hurdle | Cyan hurdle shape |
| Squat | Pass under gate | Yellow gate shape |
| Clap | Collect beat orb / replay | Purple orb |
| Raise hand (hold 1s) | Confirm / start | Green hold-ring |
| Both hands up | Pause | — |

First obstacle sequence teaches all verbs inline. No tutorial screen.

---

## Session Design (90-Second Energy Arc)

```
 RISE (0–55s)  →  PEAK (55–75s)  →  RELEASE (75–90s)  →  POSITIVE END
 speed builds      max intensity      wind-down           result + replay
```

- **Time-boxed**: exactly 90s active play (setup/pause don't consume budget)
- **Positive ending**: "BREAK COMPLETE" (time up) or "NICE RUN!" (energy spent) — never punishment
- **Instant replay**: clap on result screen → same 2…1 countdown → go
- **Combo pitch**: +1 semitone per 5 combo (cap +4), backing track stays 126 BPM
- **3 lives**: outline+X pips (not color-only), collision resets combo pitch

### Result Screen (Replay Engine)

```
┌──────────────────────────────────────┐
│         BREAK COMPLETE               │
│         1240  •  BEST 1580           │
│    NEW PERSONAL BEST / COMBO x12     │
│                                      │
│  [MIRROR]  [STRIKE]  [DUO 🔒]       │
│                                      │
│    ← LEAN to pick • CLAP to replay   │
└──────────────────────────────────────┘
```

- Lean ← → selects next mode (Duo locked unless 2 players detected)
- Clap = instant replay with selected mode
- No touch required, no menu, no extra gesture
- Score + best + combo always visible (drives "one more" impulse)

---

## Social Loop (Invite Friends)

### The Invite Pitch (3 Seconds)

> "Open this link on your phone. Stand in front of the camera. That's it."

No app store. No download. No account. Just a URL + a camera.

### Local Co-op (Same Screen, Zero Setup)
1. Friend walks into camera frame → system detects 2 players
2. Duo Groove mode **auto-unlocks** (no menu diving)
3. Both raise hands → shared countdown → play together
4. Synchronized actions = **Party Overdrive ×2 score**
5. Result shows both scores side-by-side → instant rivalry/fun

### Remote Multiplayer (WebRTC Room)
1. Player 1: Settings → "Create room" → gets 6-char code
2. Share code via any messenger ("Room: XKCD42")
3. Player 2: enters code → "Join" → WebRTC connects
4. Actions sync in real-time via data channel
5. Cloudflare TURN handles NAT traversal automatically

### Why People Invite
| Hook | Mechanism |
|------|-----------|
| "Beat my score" | Best score visible on result, persists across sessions |
| "Try this mode" | Duo Groove requires 2 bodies — natural reason to invite |
| "No install needed" | Removes the #1 friction for casual players |
| "60 seconds" | Low commitment — "it's just a brain break" |
| "Your body is the controller" | Novelty factor — people want to show it off |

---

## Returning Player (System Learns You)

The game adapts locally after 2+ sessions — no account, no cloud:

| Signal | Adaptation |
|--------|------------|
| Favorite mode (2+ starts) | Pre-selects your most-completed mode |
| Fast gesture starts (≤8s avg, 2+ samples) | Compact setup — skips verbose instructions |
| Difficulty ratio > 25% | Restores full framing guidance |
| Theme/overlay preference | Remembers your visual style |

**First visit**: full guidance, setup illustration, gesture explanation.
**Third visit**: mode pre-selected, compact ready check, straight to play.

---

## Privacy Contract

- Camera processing is **100% local** (TensorFlow.js MoveNet in-browser)
- Only recognized action events cross the network (never frames/keypoints)
- Preferences stored on-device only (`brainbreak.taste.v1`)
- No photos, poses, body geometry, or identity ever persisted
- "LOCAL ONLY" badge always visible on camera card
- Storage failure (private browsing, quota) = graceful first-session fallback

---

## Visual & Audio Identity

| Layer | Detail |
|-------|--------|
| Themes | Cyber Trunk (default) · Pink Girl Kawaii · Purple Vaporwave |
| Camera overlays | Glass PIP · Transparent Cutout · Cyber Hologram · Neon Skeleton |
| Music | "Special Spotlight" — Kevin MacLeod, 126 BPM, CC BY 4.0 |
| Audio-reactive | Neon sun, road grid, pylons, particles pulse to spectrum energy |
| HUD | Score, lives (3 pips), combo (from ×2). Quiet persistent, loud transient |

---

## Tech Stack (One Glance)

```
┌─────────────────────────────────────────────────┐
│  Browser (React + Vite)                         │
│  ├─ TensorFlow.js MoveNet (pose, local only)    │
│  ├─ WebAudio (126 BPM sync + spectrum)          │
│  └─ WebRTC DataChannel (multiplayer actions)    │
├─────────────────────────────────────────────────┤
│  WASM (Rust → macroquad)                        │
│  ├─ brainbreak-core: MotionRuntime + RunnerGame │
│  │   (deterministic FSM, scoring, collision)    │
│  └─ brainbreak-game: renderer + audio-visual    │
├─────────────────────────────────────────────────┤
│  Cloudflare Worker                              │
│  ├─ Static asset cache (fingerprinted WASM)     │
│  ├─ Room codes (Durable Object, expiring)       │
│  └─ TURN credentials (short-lived, per-room)    │
└─────────────────────────────────────────────────┘
```

---

## Quality Gates (Score-Driven)

| Metric | Target | How Measured |
|--------|--------|--------------|
| Time-to-play | ≤ 15s from URL open | Manual + Lighthouse |
| Zero-touch success | ≥ 90% gesture starts | `taste-profile` counters |
| Session completion | ≥ 70% reach 90s | `breaksCompleted / sessionsStarted` |
| Replay rate | ≥ 50% clap-replay | Result→start events |
| Duo discovery | Auto-unlock at 2 players | Camera framing test |
| Privacy | 0 frames leave device | Network audit |

---

## Complete Flow Diagram

```
┌─────────┐     ┌──────────┐     ┌─────────────┐     ┌─────────┐
│ LAUNCHER │────▶│  READY   │────▶│  COUNTDOWN  │────▶│ RUNNING │
│ pick mode│     │ framing  │     │   2…1 GO    │     │  90s    │
└─────────┘     │ + confirm│     └─────────────┘     └────┬────┘
     ▲          └──────────┘                              │
     │               ▲                              time up│ or
     │               │                              lives = 0
     │          ┌────┴─────┐                             │
     │          │ TRACKING │◀── lose camera ──┐          │
     │          │  HOLD    │                  │          ▼
     │          └──────────┘            ┌─────┴───┐ ┌────────┐
     │               │                │ PAUSED  │ │ RESULT │
     │          regain camera         │ (hands  │ │ score  │
     │               │                │  up)    │ │ + mode │
     │               ▼                └─────────┘ └───┬────┘
     │          resume ──▶ RUNNING                    │
     │                                          clap = replay
     └──────────── choose another ◀──────────────────┘
```

**Every transition is hands-free** (except the initial camera tap).

---

## Design Principles (Priority-Ranked)

1. **Zero-Touch or Die** — standing at 2m, you must NEVER need to touch the screen after camera start
2. **One Dominant Action** — each screen shows exactly ONE thing to do next (shape > text)
3. **Positive Always** — no punishment language, no fail state without instant recovery
4. **Body Before Brain** — physical correction (framing) always outranks navigation
5. **90 Seconds Sacred** — the time-box is a gameplay contract, not a timer
6. **Privacy Is a Feature** — "LOCAL ONLY" badge visible at all times
7. **Social by Default** — co-op is not a mode you find, it's what happens when a friend shows up
8. **Learn Locally** — the system gets faster for you without sending data anywhere

---

## Quick Start (Developer)

```bash
cd brainbreak
npm install
npm run dev          # game at localhost:5173
npm run dev:worker   # signaling API (multiplayer)
```

Validation:
```bash
cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test --all
npm run typecheck && npm test && npm run build
```
