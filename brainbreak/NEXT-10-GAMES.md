# BrainBreak Next 10 — Sensory Ritual Game Catalog

> **Philosophy**: Transform 60–90 second motion breaks into **multi-sensory awe rituals**.
> Not gamification (exercise → points). **Sensoryfication** (movement → wonder).

**Design Mandate**: ĐƠN GIẢN → WOW → DỄ VIRAL → BUỘC PHẢI CHƠI CÙNG NHAU

---

## Architecture Constraints (All 10 Games Must Respect)

| Constraint | Rule |
|-----------|------|
| Zero-Touch | After camera tap, 100% hands-free (lean, raise, clap, pinch) |
| Time-Box | 60–90s active play, deterministic core owns the clock |
| Privacy | Pose processing local-only, only action events cross network |
| One Dominant Action | Each moment shows ONE thing to do (shape > text) |
| Positive End | No punishment. "BREAK COMPLETE" or "NICE RUN!" always |
| Social Default | 2nd player in frame = co-op auto-unlock |
| Tech Stack | Rust WASM (deterministic) + TF.js MoveNet + WebRTC + Cloudflare |
| Body Verbs | Lean, Jump, Squat, Clap, Raise Hand, Pinch, Grab, Tilt, Step |

---

## The 10 Games (Priority-Ranked by Viral × Invite × WOW)

### 1. 🌟 Supernova Drop (Đập Tan Siêu Tân Tinh)

| | |
|---|---|
| **Core** | Pump energy into a floating core with punches/squats, then PULL THE DROP |
| **Verbs** | Punch (L/R), Squat, Grab+Pull |
| **Players** | 1–2 (co-op pumps faster) |
| **Arc** | Build-up (0–50s) → Tension (50–58s) → **THE DROP** (58–60s) → Afterglow |
| **WOW** | Core explodes into 10K particles filling the room. Bass drops. Screen flashes. Haptic slam. |
| **Viral** | The Drop moment = screenshot/recording bait. "I made a star explode with my fists." |
| **Invite** | 2 players pump 2× faster → bigger explosion → "help me make it BIGGER" |
| **Online Duel** | Async: "My supernova was 12,000 lumens. Beat that." Share explosion replay. |
| **Tech** | Particle system in WASM renderer. AudioWorklet for bass-synced deformation. Simplex noise on core mesh. |

---

### 2. 🌟 Beat Duel (Thách Đấu Nhịp)

| | |
|---|---|
| **Core** | 1v1 rhythm battle — mirror the beat sequence faster than your opponent |
| **Verbs** | All 6 (Lean, Jump, Squat, Clap, Raise L, Raise R) |
| **Players** | 2 (same camera OR online WebRTC) |
| **Arc** | Round 1 easy (20s) → Round 2 fast (20s) → Round 3 insane (20s) → Winner pose |
| **WOW** | Split-screen neon VS. Loser's side "cracks" like glass. Winner gets crown particle burst. |
| **Viral** | "I beat my friend 3-0 in Beat Duel. Rematch?" — natural challenge loop |
| **Invite** | **THE invite mechanic**: "Challenge a friend" → generates room code → share link |
| **Online Duel** | Real-time WebRTC sync. Both see same beat sequence. Latency-compensated scoring. |
| **Tech** | Extend existing `RunnerGame` FSM with `DuelPhase`. WebRTC action sync already exists. Split-screen render. |

---

### 3. 🌟 Liquid Balance (Thăng Bằng Giọt Nước)

| | |
|---|---|
| **Core** | 2 players tilt bodies to balance a giant water bubble between them |
| **Verbs** | Tilt (lean L/R), Arms spread (width control), Squat (lower) |
| **Players** | 2 (mandatory co-op — single player = tutorial mode) |
| **Arc** | Calm (0–30s) → Waves (30–60s) → Storm (60–80s) → **Shatter** (80–90s) |
| **WOW** | Perfect balance → bubble glows golden. Completion → shatters into 1000 droplets floating around both players (TeamLab moment). |
| **Viral** | "We kept the water calm for 90 seconds!" — co-op achievement, no competition pressure |
| **Invite** | REQUIRES 2 bodies. "I need a partner to hold this water." Natural invite reason. |
| **Online** | Sync tilt data via WebRTC. Both see same bubble. "Long-distance balance challenge." |
| **Tech** | Fluid shader (2D metaball approximation in macroquad). Split render textures. Co-op score = stability × time. |

---

### 4. 🌟 Neon Growth Garden (Vũ Điệu Sinh Số)

| | |
|---|---|
| **Core** | Your body draws holographic light trails that grow into a unique AR tree |
| **Verbs** | All full-body movement (arms draw branches, steps grow roots, jumps bloom flowers) |
| **Players** | 1–4 (each player grows a different tree → shared forest) |
| **Arc** | Seed (0–10s) → Sprout (10–40s) → Canopy (40–70s) → **Bloom** (70–90s) |
| **WOW** | Final bloom: entire tree erupts in bioluminescent flowers synced to music crescendo. Your tree is UNIQUE (generative seed from movement data). |
| **Viral** | "Look at my Energy Tree!" — shareable generative art. No two trees alike. |
| **Invite** | "Let's grow a forest together." 2+ players = shared ecosystem. 4 players = full forest. |
| **Online** | Async: contribute your tree to a shared "Global Forest" (server stores generative seed only). |
| **Tech** | L-system generative algorithm in Rust. Movement smoothness → branch quality. Temporal smoothing on keypoints. |

---

### 5. 🌟 Pulse Wall (Bức Tường Nhịp)

| | |
|---|---|
| **Core** | A wall of neon obstacles approaches — contort your body through the shaped holes |
| **Verbs** | Jump, Squat, Lean L/R, Arms Up/Down (body shape matching) |
| **Players** | 1–2 (side-by-side, each gets own wall) |
| **Arc** | Slow walls (0–30s) → Double walls (30–55s) → **Rapid fire** (55–75s) → Final wall (75–90s) |
| **WOW** | Perfect pass-through → wall shatters like glass with your body silhouette glowing in the debris. |
| **Viral** | "I fit through a star-shaped hole!" — physical comedy + achievement. |
| **Invite** | Duel mode: who passes more walls in 60s? "I got 23 walls. Beat me." |
| **Online Duel** | Same wall sequence sent to both players. Compare pass rates. Ghost overlay. |
| **Tech** | Wall = 2D shape mask. Pose keypoints checked against mask holes. Deterministic sequence in Rust core. |

---

### 6. 🌟 Shadow Clone (Bóng Ma Tốc Độ)

| | |
|---|---|
| **Core** | Race against your own ghost (or a friend's ghost) through a pose obstacle course |
| **Verbs** | All verbs (course requires full vocabulary) |
| **Players** | 1 (vs ghost) or 2 (vs friend's ghost, async) |
| **Arc** | Learn course (0–30s) → Ghost appears (30s) → Race (30–80s) → **Photo finish** (80–90s) |
| **WOW** | Beating your ghost → it "shatters" into light particles that flow INTO you (power-up visual). |
| **Viral** | "I beat my own shadow by 3 seconds!" — self-improvement loop, infinite replayability. |
| **Invite** | "Send me your ghost!" — async challenge. Friend records run → you race their ghost. |
| **Online** | Async ghost sharing via Cloudflare KV (store action sequence + timestamps, ~2KB per run). |
| **Tech** | Record action timeline → replay as ghost avatar. Deterministic core ensures ghost is frame-accurate. |

---

### 7. 🌟 Gravity Storm (Bão Trọng Lực)

| | |
|---|---|
| **Core** | You ARE gravity. Tilt your body to steer a ball through a neon maze while avoiding black holes. |
| **Verbs** | Tilt (full body lean), Jump (ball bounces), Squat (ball shrinks to fit gaps) |
| **Players** | 1–2 (P2 controls wind/storms to help or hinder) |
| **Arc** | Calm maze (0–30s) → Gravity shifts (30–55s) → **Black hole chase** (55–80s) → Escape (80–90s) |
| **WOW** | Escaping the black hole → gravitational lensing visual effect + triumphant bass drop. |
| **Viral** | "The black hole almost got me!" — tension + relief = shareable moment. |
| **Invite** | P2 as "Storm Master" creates obstacles for P1. Asymmetric co-op/versus. |
| **Online** | P2's storm inputs synced via WebRTC. "I'm the storm. You survive." |
| **Tech** | Physics sim in Rust (simple 2D gravity + collision). Tilt angle from shoulder/hip keypoints. |

---

### 8. 🌟 Echo Arena (Đấu Trường Vọng)

| | |
|---|---|
| **Core** | Perform a move sequence → it echoes back harder. Each round adds one move. Memory + speed. |
| **Verbs** | Progressive: starts with 1 verb, adds one per round (max 6 by round 6) |
| **Players** | 1–∞ (round-robin: each player adds to the chain) |
| **Arc** | Round 1–2 easy (0–30s) → Round 3–4 tricky (30–60s) → Round 5+ **chaos** (60–90s) |
| **WOW** | Completing a 6-move chain perfectly → all 6 moves fire simultaneously as a "combo explosion" |
| **Viral** | "I remembered a 6-move chain at speed!" — cognitive + physical flex. |
| **Invite** | Round-robin: P1 does 2 moves, P2 must copy + add 1. "Simon Says" but with your whole body. |
| **Online** | Turn-based WebRTC. Each player's sequence sent to opponent. "Copy THIS." |
| **Tech** | Sequence buffer in Rust core. Playback system for "echo" visualization. No new pose verbs needed. |

---

### 9. 🌟 Particle Storm (Bão Hạt)

| | |
|---|---|
| **Core** | You're a neon avatar in a particle field. Your movements create shockwaves that push particles into target shapes. |
| **Verbs** | Sweep arms (push), Clap (shockwave), Jump (updraft), Squat (gravity well) |
| **Players** | 1–2 (co-op: both push particles toward same shape) |
| **Arc** | Simple shape: circle (0–30s) → Complex: star (30–55s) → **Impossible: phoenix** (55–90s) |
| **WOW** | Completing the phoenix → 5000 particles ignite into a fire-bird that flies across the screen. |
| **Viral** | "We sculpted a phoenix out of pure particles!" — creative + physical. |
| **Invite** | Co-op required for complex shapes. "I need your gravity well to hold the left wing." |
| **Online** | Shared particle field synced via WebRTC (send force vectors, not positions — bandwidth-efficient). |
| **Tech** | Particle sim in WASM (bounded 5K particles). Force fields from pose velocity. Shape-match scoring. |

---

### 10. 🌟 Party Overdrive (Bão Tiệc)

| | |
|---|---|
| **Core** | ALL games combined. Random 15s micro-challenges from games 1–9. Fastest, most chaotic, most social. |
| **Verbs** | Everything (random verb set per micro-challenge) |
| **Players** | 1–4 (more players = more chaos = more fun) |
| **Arc** | 6 micro-challenges × 15s each = 90s. Each from a different game. Final = "ALL AT ONCE" |
| **WOW** | Final 15s: all effects combine — particles + fluid + generative art + walls + gravity = sensory overload. |
| **Viral** | "Party Overdrive is INSANE with 4 people!" — the ultimate party mode. |
| **Invite** | "We need 4 people for Party Overdrive." — explicit group invite mechanic. |
| **Online** | 4-player WebRTC mesh. Each player's actions affect shared chaos. |
| **Tech** | Orchestrator FSM that sequences mini-games. Reuses all game cores. Shared particle/render budget. |

---

## Viral & Invite Mechanics (Cross-Game Systems)

### Challenge System (Thách Đấu)

```
┌──────────────────────────────────────────────────────────┐
│  CHALLENGE FLOW                                          │
│                                                          │
│  1. Finish any game → Result screen shows "CHALLENGE"    │
│  2. Raise hand on CHALLENGE → generates 6-char code      │
│  3. Share code (speak it, text it, show it)              │
│  4. Friend opens same URL → enters code → JOINS          │
│  5. Both play same seed → compare scores                 │
│  6. Winner gets "CROWN" badge on next result             │
│  7. Loser gets "REMATCH?" prompt (clap = yes)            │
└──────────────────────────────────────────────────────────┘
```

### Ghost System (Bóng Ma)

- Every completed run stores: action timeline + timestamps (~2KB)
- "Race my ghost" = async challenge (no real-time sync needed)
- Ghost appears as translucent neon silhouette
- Beating a ghost → it shatters into you (visual power-up)
- Cloudflare KV stores ghost data (TTL 7 days, privacy-safe: actions only)

### Party Room (Phòng Tiệc)

- Extend existing room system: 4-char code, up to 4 players
- Room host picks game → all players see same challenge
- Shared countdown (existing 2…1 system)
- Results show all 4 scores side-by-side
- "CROWN" rotates to highest scorer each round

### Viral Hooks Per Game

| Game | Shareable Moment | One-Line Invite |
|------|-----------------|-----------------|
| Supernova Drop | Explosion screenshot | "Help me make it BIGGER" |
| Beat Duel | Win streak counter | "Rematch. Now." |
| Liquid Balance | Golden bubble glow | "I need a partner" |
| Neon Garden | Unique tree image | "Let's grow a forest" |
| Pulse Wall | Body-silhouette shatter | "I got 23 walls. You?" |
| Shadow Clone | Ghost-beat time delta | "Race my shadow" |
| Gravity Storm | Black hole escape | "I'm the storm. Survive." |
| Echo Arena | Chain length record | "Copy THIS sequence" |
| Particle Storm | Phoenix completion | "Hold the left wing" |
| Party Overdrive | 4P chaos screenshot | "We need 4 people" |

---

## Technical Architecture Extensions

### New Rust Core Modules

```
brainbreak-core/src/
├── lib.rs              (existing: MotionRuntime, PoseRecognizer)
├── runner.rs           (existing: RunnerGame — Neon Beat Runner)
├── game_state.rs       (existing: GameState — Mirror/Strike/Duo)
├── supernova.rs        (NEW: energy pump + drop FSM)
├── duel.rs             (NEW: 1v1 round-based scoring)
├── fluid_balance.rs    (NEW: tilt physics + stability scoring)
├── garden.rs           (NEW: L-system generative seed)
├── pulse_wall.rs       (NEW: shape-mask collision)
├── ghost.rs            (NEW: action timeline record/replay)
├── gravity.rs          (NEW: 2D physics + tilt steering)
├── echo.rs             (NEW: sequence memory + playback)
├── particle.rs         (NEW: force-field particle sim)
└── party.rs            (NEW: micro-challenge orchestrator)
```

### Browser Layer Extensions

```
web/src/
├── vision.ts           (existing: pose detection)
├── audio.ts            (existing: WebAudio beat sync)
├── network.ts          (existing: WebRTC rooms)
├── challenge.ts        (NEW: challenge code gen + join)
├── ghost-store.ts      (NEW: Cloudflare KV ghost upload/download)
├── particle-render.ts  (NEW: GPU particle system via WebGL2)
├── fluid-shader.ts     (NEW: 2D metaball/fluid approximation)
└── generative.ts       (NEW: L-system rendering from Rust seed)
```

### Multiplayer Topology

```
SAME CAMERA (local co-op):
  Camera → TF.js → 2 pose streams → Rust core (player[0], player[1])

ONLINE (WebRTC):
  P1 Camera → TF.js → action events → DataChannel → P2 Rust core
  P2 Camera → TF.js → action events → DataChannel → P1 Rust core
  (Only action masks cross network. Never poses/frames.)

ASYNC (Ghost/Challenge):
  Run complete → action timeline → Cloudflare KV (2KB, TTL 7d)
  Challenger → download ghost → replay as translucent avatar
```

---

## Implementation Priority (Score-Driven)

| Priority | Game | Value = Viral × Invite × Feasibility | Effort |
|----------|------|--------------------------------------|--------|
| **P0** | Beat Duel | 10 × 10 × 9 = 900 | Low (extends existing runner) |
| **P0** | Supernova Drop | 10 × 8 × 8 = 640 | Medium (particle system) |
| **P1** | Pulse Wall | 8 × 8 × 9 = 576 | Low (shape mask + existing verbs) |
| **P1** | Shadow Clone | 7 × 9 × 9 = 567 | Low (record/replay existing actions) |
| **P1** | Echo Arena | 8 × 9 × 8 = 576 | Low (sequence buffer) |
| **P2** | Liquid Balance | 9 × 10 × 6 = 540 | High (fluid shader) |
| **P2** | Neon Garden | 9 × 8 × 7 = 504 | Medium (L-system + rendering) |
| **P2** | Gravity Storm | 7 × 8 × 7 = 392 | Medium (physics sim) |
| **P3** | Particle Storm | 8 × 8 × 5 = 320 | High (5K particle sim in WASM) |
| **P3** | Party Overdrive | 10 × 10 × 4 = 400 | High (orchestrates all others) |

**Build order**: Beat Duel → Supernova Drop → Pulse Wall → Shadow Clone → Echo Arena → Liquid Balance → Neon Garden → Gravity Storm → Particle Storm → Party Overdrive

---

## Session Flow (Universal for All 10 Games)

```
LAUNCHER (pick game)
  → CAMERA TAP (one touch)
    → FRAMING COACH (step back, get in frame)
      → READY CONFIRM (raise hand 1s)
        → COUNTDOWN 2…1
          → GAMEPLAY (60–90s)
            → RESULT (score + challenge prompt)
              → CLAP = replay
              → LEAN = challenge/invite
              → BOTH HANDS = back to launcher
```

**Every game follows this exact flow.** Zero learning curve between games.
The ONLY thing that changes is what your body does during the 60–90s gameplay window.

---

## Quality Gates (All 10 Games)

| Metric | Target | Applies To |
|--------|--------|-----------|
| Time-to-play | ≤ 15s | All |
| Zero-touch rate | ≥ 90% | All |
| Session completion | ≥ 70% | All |
| Replay rate | ≥ 50% | All |
| Challenge sent | ≥ 30% of results | Duel, Pulse, Shadow, Echo |
| Co-op discovery | Auto at 2 players | Liquid, Garden, Particle, Party |
| Ghost raced | ≥ 20% of solo runs | Shadow, Pulse, Beat Duel |
| WOW screenshot | ≥ 10% of sessions | Supernova, Garden, Particle |
