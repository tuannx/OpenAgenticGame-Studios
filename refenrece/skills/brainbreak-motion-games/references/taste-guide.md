# BrainBreak Motion Games — TASTE & Experience Design Guide

> **Ingested & Optimized Taste Architecture for Motion-Controlled Brain Break Games**  
> *Derived from TikTok micro-challenges, GoNoodle classroom energizers, Just Dance pose feedback, Nintendo WarioWare/Ring Fit micro-interactions, and Web AI camera experiments.*

---

## 1. Cross-Platform Brain Break Research & Analysis

| Platform / Genre | Key Motion Mechanics | Sensory & UX Hooks | Identified TASTE Value |
|---|---|---|---|
| **TikTok / Shorts Micro-Challenges** *(Filter Games, Face/Body Music)* | Rapid tilt/jump, gesture speedruns, 15-30s sprint challenges. | Real-time pose overlay, countdown timers, instant replay/record feel. | **Frictionless Onboarding & Hyper-Short Loop**: Player must be active within 2 seconds. Zero tutorial screens. |
| **GoNoodle & Classroom Kids Breaks** *(Freeze Dance, Floor is Lava, Blazer Fresh)* | Pattern mirroring, sudden posture freeze, high-energy jump/squat cues. | Silly playful audio feedback, high contrast visuals, zero death/shame mechanics. | **Zero-Punishment Framing**: Mistakes produce funny bounce/slip SFX rather than depressing "Game Over" screens. |
| **Just Dance / Just Dance Now** *(Ubisoft)* | 2-beat lead pictogram cues, pose accuracy scoring (Perfect/Great/Good/Ok). | Rising audio chords per combo tier, screen pulsing, neon particle trails, beat sync. | **Telegraphed Rhythm Feedback**: Pictograms prepare players before movements happen. Feedback builds emotional euphoria. |
| **WarioWare / Ring Fit** *(Nintendo Micro-Games)* | Rapid verb prompts ("SQUAT!", "JUMP!", "CLAP!"), posture stamina holds. | Punchy squash-and-stretch UI, micro-narratives, physical feedback intensity. | **Juicy Micro-Verbs & Weight**: Motion feels physically impactful through visual/audio squash & stretch. |
| **Web Motion AI & Roblox Micro-Games** *(Google AI, Red Light Green Light)* | Spatial dodging, multi-player side-by-side camera tracking, pose freeze. | Holographic skeletal overlays, multi-person bounding box separation. | **Multi-Player Camera Isolation**: Clear visual separation of multi-person bodies and camera mode flexibility. |

---

## 2. Play Contexts, Zero-Touch Contract & AR Rules

### Play Context Taxonomy (Bối Cảnh Chơi Phổ Biến)

1. **Classroom / Kids Energizer (Lớp học / Mầm nông & Tiểu học)**:
   - **Distance**: 2.0m – 4.0m từ thiết bị/màn hình lớn.
   - **Environment**: Đông người, náo nhiệt, không gian rộng.
   - **Required Verbs**: Động tác biên độ lớn (Giơ tay cao, vỗ tay lớn, nhảy nhẹ tại chỗ).
2. **Living Room / Family Party (Phòng khách Gia đình)**:
   - **Distance**: 1.5m – 2.5m từ TV/iPad/Bàn trà.
   - **Environment**: 1 hoặc 2 người chơi song song (Cha mẹ & Bé hoặc 2 anh em).
   - **Required Verbs**: Nghiêng né trái/phải, vỗ tay tạo nhịp, High-five đồng đội.
3. **Desk Micro-Break (Góc làm việc Văn phòng)**:
   - **Distance**: 0.8m – 1.2m từ Webcam Laptop.
   - **Environment**: Ngồi hoặc đứng tại chỗ, tiếng ồn thấp.
   - **Required Verbs**: Động tác tay từ xa (Giơ 1 tay, vỗ tay nhẹ, nghiêng đầu/vai).

---

### Zero-Touch UX Rules (Quy Tắc Không Chạm Màn Hình)

1. **Hợp đồng Không Chạm từ xa (Zero Screen Touch Contract)**:
   - Sau khi bật camera, người chơi đứng xa **1.5m - 2.5m** và **KHÔNG BAO GIỜ phải đi lại chạm vào màn hình**.
   - Mọi thao tác: Chọn trò chơi, Xác nhận sẵn sàng, Tạm dừng, và Chơi lại đều thực hiện **100% bằng cử chỉ tay / tư thế từ xa**.
2. **Bộ 3 Động Tác Tay Tinh Tế & Nhận Diện Cực Nhạy**:
   - 🖐️ **Giơ tay cao / Hold Hand (Xác nhận / Chọn Menu)**: Giơ 1 tay qua vai trong 1.0 giây để lấp đầy vòng tròn AR Radial Gauge.
   - 👏 **Vỗ tay (Double Clap)**: Kích hoạt Bắt đầu chơi / Chơi lại ngay lập tức / Nhận điểm thưởng nhịp điệu.
   - ↔️ **Nghiêng thân người (Body Tilt / Lean)**: Đổi làn né tránh hoặc di chuyển con trỏ menu.
3. **Đồng Bộ Nhịp Điệu - Âm Thanh - Ánh Sáng (Audio-Rhythm-Lighting Synergy)**:
   - **Âm thanh (Audio)**: Động tác tay vỗ đúng phách (Downbeat ±100ms) phát ra tiếng vỗ tay giòn giã + Pitch tăng `+1 semitone` per 5 combo.
   - **Ánh sáng (Lighting)**: Nhịp thở mặt trời neon, đường đua sáng bừng đồng bộ với phách nhạc 126 BPM.
   - **AR Aura (Đồ họa AR)**: Hào quang sáng rực quanh bàn tay và bộ xương skeleton, biến động tác tay thành hiệu ứng ánh sáng lung linh.

---

## 3. The 6 Pillars of BrainBreak TASTE Architecture

```
                  ┌─────────────────────────────────────────┐
                  │        BRAINBREAK TASTE MATRIX          │
                  └────────────────────┬────────────────────┘
                                       │
      ┌──────────────────┬─────────────┼─────────────┬──────────────────┐
      ▼                  ▼             ▼             ▼                  ▼
┌───────────┐      ┌───────────┐ ┌───────────┐ ┌───────────┐      ┌───────────┐
│  JUICE &  │      │ PACING &  │ │ ERGONOMICS│ │ ART & HUD │      │ AUDIO &   │
│ FEEDBACK  │      │ FLOW ARC  │ │ SAFETY    │ │ IDENTITY  │      │ SYNERGY   │
└───────────┘      └───────────┘ └───────────┘ └───────────┘      └───────────┘
```

### Pillar 1: Juice & Physical Responsiveness (Phản Hồi & Visual Juiciness)
- Sub-50ms Optical Mirror Feedback.
- Dynamic pitch scaling audio SFX (+1 semitone per 5 combo).
- Micro screen shakes & 3D spring score popups.

### Pillar 2: Micro-Pacing & Frictionless Flow (Nhịp Độ 90s & Trải Nghiệm Mượt Mà)
- 90-second energy peak arc.
- Zero-punishment resiliency: mistakes reduce energy bar, gesture clap to instant retry.

### Pillar 3: Motion Ergonomics & Physical Safety (Động Học & An Toàn Cơ Thể)
- Adaptive spatial calibration (desk vs full standing room).
- Multi-player camera isolation: stable body ID tracking for P1 and P2 side-by-side.

### Pillar 4: Art Direction & Neon Cyber-Party Visuals (Mỹ Thuật & Đồ Họa Modern)
- Cyber Neon, Pink Girl Kawaii, and Vaporwave Sunset color palettes.
- Glassmorphism & Hologram PIP camera modes.

### Pillar 5: Audio-Reactive Synergy (Tương Tác Âm Nhạc)
- Real-time WebAudio API spectral analysis driving neon lighting & pylon pulses.
- Quantized downbeat action bonus (+50% score + chromatic aura effect).

### Pillar 6: Social & Micro-Viral Handoff (Tính Lan Truyền Xã Hội)
- Co-Op Synchronized Actions ("Party Overdrive" x2 score bonus).
- Downloadable/Shareable Victory Posture Card.

---

## 4. Implementation Hook Mapping in Repository

| TASTE Element | Implementation File | Key Struct / Function |
|---|---|---|
| **Deterministic Action Recognition** | `brainbreak/crates/brainbreak-core/src/lib.rs` | `MotionRuntime`, `RecognizerConfig`, `PlayerState` |
| **Juice, Rendering & Audio Sync** | `brainbreak/crates/brainbreak-game/src/visuals.rs` | `VisualEngine`, `BeatPhase`, `ParticleSystem` |
| **Lane Runner Rules & Scoring** | `brainbreak/crates/brainbreak-core/src/lib.rs` | `RunnerGame`, `RunnerObstacle`, `RunnerPhase` |
| **Macroquad HUD & Phase Overlays** | `brainbreak/crates/brainbreak-game/src/main.rs` | `draw_player_hud`, `draw_phase_overlay` |
| **Camera Inference & Pose Bridge** | `brainbreak/web/src/vision.ts` & `bridge.ts` | `startVision`, `updatePoseBridge`, `setOverlayConfig` |
| **Styling & Glassmorphic HUD** | `brainbreak/web/src/style.css` | `--theme-*` tokens, `.mode-glass-pip`, `.mode-hologram` |

---

## 5. Checklist for BrainBreak Game TASTE Compliance

- [ ] **Zero Screen Touch**: All in-game choices and ready triggers executed via far-distance gestures.
- [ ] **Instant Feedback**: Pose overlay updates seamlessly with optical mirror feel (<50ms).
- [ ] **Audio-Lighting Synergy**: Movement actions trigger quantized SFX with beat-matched lighting pulses.
- [ ] **Multiplayer Stance Check**: Both P1 and P2 must be detected and ready before multiplayer starts.
- [ ] **Minimalist High-Taste UI**: Big visual thumbnails, clear icons, minimal clutter text.

---

## 6. Local Session Learning Without Camera Profiling

Repeat-session adaptation must make the next session shorter without turning
camera evaluation into analytics.

### Persist only aggregate boundary events

- Allowed: completed game mode, camera-attempt count, gesture/touch/guide-only
  outcome, bounded time-to-ready mean, theme, overlay mode, and opacity.
- Forbidden: frames, images, video, keypoints, pose snapshots, body geometry,
  per-frame history, identity, or network sync.
- Write only at meaningful UI boundaries. Never serialize from the pose callback
  or render loop.

### Adapt conservatively and reversibly

- Require at least two completed starts before recommending a mode.
- Condense physical setup guidance only after two fast gesture-confirmed starts.
- Keep a compact placement reminder visible; learned familiarity is not proof
  that the physical room or device position is unchanged.
- Restore full guidance when the latest outcome is guide-only, touch fallback,
  camera/setup failure, or explicit setup exit.
- Treat corrupt, blocked, or quota-limited storage as a normal first-session
  state; it must never block camera setup or gameplay.

---

## 7. Hands-Free Run Completion

The zero-touch contract continues through game over. A player who has stepped
back to camera distance must not walk forward merely to replay or change mode.

### Deterministic navigation

- Keep result selection inside the deterministic runtime, filtered by the same
  evaluated-player mask as scoring.
- Default selection to the current mode so one clap means immediate replay.
- Use exclusive left/right motion to cycle modes; simultaneous opposing input
  from multiple players cancels rather than causing selection jitter.
- Emit a one-shot next-run request to the composition root. Browser code may
  synchronize mode/taste state but must not decide the result transition.
- Re-check Duo readiness when switching from a one-player mode; a result-screen
  shortcut must never bypass the two-player requirement.

### Camera-distance result design

- Show the replay/change choice immediately. Do not insert a death animation,
  arbitrary delay, interstitial, or return-to-main-menu step.
- Use positive framing (`NICE RUN`), score, personal-best distance, and best run
  combo. Avoid punishment language.
- Prefer three large mode chips over a nested menu. Selected state needs fill,
  border weight, and a shape marker so it works without color.
- Split instructions into two large lines: `LEAN TO CHOOSE` and `CLAP TO PLAY`.
  Do not compress both actions into one 11px sentence.
- Keep HUD secondary to motion cues: mode, score, life, and combo are enough;
  hide player cards during Ready and Result overlays to avoid duplicate data.

---

## 8. Tracking-Safe Hold and Resume

AR gameplay must stop before it explains tracking loss. A warning layered over
an advancing world is not a pause and can return the player directly into harm.

### Deterministic safety boundary

- Model tracking hold and resume countdown as explicit core states, not DOM
  booleans. Missing evaluation must return before beat, distance, score, life,
  combo, or collision progression.
- Clear in-flight hazards when hold begins. Preserving obstacle continuity is
  less important than preventing an unseen immediate collision after reacquire.
- Require a fresh evaluated movement or clap to ready, then keep the world
  frozen through a short two-second countdown.
- If evaluation drops during countdown, return to hold and reset readiness.
- Duo requires both local players visible and both ready; one player cannot
  resume a shared physical run for the other.

### Browser freshness and presentation

- Page inactivity must clear the latest pose snapshot. Visibility alone cannot
  re-enable evaluation; wait for a fresh visible-page pose callback.
- Render one large `RUN PAUSED` title plus one instruction. Do not expose a
  settings menu or touch-only pause dialog at camera distance.
- Hide HUD cards, next-action cue, and beat meter while frozen. The countdown is
  the only dominant visual during `Resuming`.
- Use static copy in the render loop; an indefinite hold state must not allocate
  a new instruction string every frame.

---

## 9. Camera-Intent Lazy Capability Handoff

A dynamic import is not lazy if startup preference restoration calls it. Prove
the boundary from observed browser assets, not from source syntax alone.

### Keep the launcher light

- Theme, overlay mode, and opacity are plain preference data. Updating them
  before camera intent must not import TensorFlow, MoveNet, or camera adapters.
- Load the vision module only after the explicit camera action. Do not replace
  accidental eager loading with speculative idle prefetch on Save-Data or slow
  mobile connections.
- Deduplicate concurrent imports, apply the latest preferences after resolution,
  and clear the pending promise on failure so retry remains possible.
- Cleanup must be `stop-if-loaded`; never import a capability merely to stop a
  setup attempt whose import just failed.

### Make waiting truthful and no-touch first

- Emit typed capability phases such as permission, initializing, calibrating,
  tracking, and recoverable error. Browser adapters must not encode localized
  copy that the DOM later parses.
- During the one allowed camera touch, keep the selected same-style mode image
  visible and show one concise live status over it.
- De-emphasize gesture instructions until camera/model readiness, then let pose
  guidance own the status so backend messages cannot overwrite lean/hold cues.
- Keep touch fallback for accessibility, but collapse it behind one semantic
  disclosure and disable its actions until tracking can safely honor them.

### Evidence package

- On a fresh origin, inventory launcher assets before any camera interaction;
  no generated `vision-*.js` asset should be present.
- Trigger camera intent without granting permission when unattended testing is
  required; vision should appear at that boundary, and the UI should identify
  the permission phase without claiming camera/model readiness.
- Report the deferred chunk size separately. Removing it from the initial path
  does not make its later camera-start cost disappear.

---

## 10. Framing Before Gesture Navigation

A pose detector returning a person is not proof that motion input is usable.
Aggregate confidence can remain high while a wrist, hip, or the top of the head
is cropped, producing a setup screen that asks for gestures it cannot read.

### Coach one physical correction at a time

- Assess the landmarks required by the active recognizer, camera margins, and
  body scale before forwarding a pose into no-touch setup navigation.
- Prefer one actionable instruction: enter frame, move closer/back, center, show
  the needed body landmarks, or invite the next player. Do not show a checklist
  that must be read while the player is moving away from the screen.
- Require a short stable tracked-identity dwell before gestures become active.
  Any framing loss must cancel an in-flight confirm hold immediately.
- Keep framing local and ephemeral. Do not persist landmark coordinates, body
  measurements, screenshots, or inferred biometric traits as taste signals.

### Preserve mode-navigation escape routes

- Framing quality and mode readiness are different contracts. One valid player
  may still lean away from Duo, while Duo confirmation continues to require two
  valid players.
- Put the framing cue on the actual mirrored camera preview, using the active art
  tokens; a separate generic tutorial illustration cannot prove live alignment.
- Keep touch fallback collapsed and secondary. Enable it only after a stable
  tracked setup state or a recoverable capability error.

---

## 11. Capability-Truthful Modes and Camera Geometry

A polished card is still a broken promise when the active detector cannot
evaluate its mode. Availability must follow measured local capability through
every navigation boundary, not infer support from screen width alone.

### Fail closed across the whole mode flow

- Define pose capacity before loading the vision model and use the same value
  for detector choice, maximum poses, launcher availability, Random selection,
  gesture navigation, bridge normalization, and deterministic result replay.
- A one-pose profile must never enter Duo through a saved recommendation,
  keyboard focus, touch, lean, random choice, runtime write, or replay shortcut.
- Keep an unavailable illustrated mode visible and visually related to the rest
  of the set. Mute it, remove it from focus, and explain the physical requirement
  in one line instead of silently hiding it.
- Restore a mode only when inference performance supports its capacity. A phone-
  sized viewport on a desktop and a desktop-UA iPad are not capability evidence.

### Make the preview match the evaluation frame

- Let intrinsic video metadata own the shared video/canvas card aspect. A fixed
  4:3 shell with `cover` can crop a portrait stream and make framing instructions
  disagree with what the detector receives.
- Use one coordinate space for the mirrored video, pose overlay, framing reticle,
  and status. Prefer `contain` as a no-crop guard during orientation transitions.
- Bound portrait preview width so its increased height leaves room for the ready
  panel and safe areas; verify portrait, landscape, and desktop overflow.
- Pure geometry tests prove normalization, not physical alignment. Report actual
  landmark alignment, camera distance, and rotation behavior as pending unless a
  real permissioned camera session was observed.

---

## 12. No-Camera Fallbacks Must Not Become Dead Ends

Guide-only safety is a runtime invariant, not a reason to strand the player on
a canvas that only says camera is required. A fallback must make its limitation
and next useful action visible without opening settings.

### Promise only what the fallback can deliver

- Use `demo` or `preview` when evaluation is disabled; do not label the path as
  playable motion if body input cannot steer or score.
- Keep score, collision judgment, combo, and multiplayer emission disabled in
  the deterministic runtime. A more attractive fallback must never weaken the
  camera-first boundary.
- Reuse the selected mode artwork and title so the fallback feels like the same
  game, not a generic error or a second visual language.

### Keep recovery obvious and physically reachable

- Put one dominant camera CTA directly in the fallback, at least 60 px tall and
  in the phone's bottom thumb zone. Do not bury it in a collapsed settings menu.
- Keep return-to-modes visible but quieter, with at least a 48 px target and a
  predictable keyboard focus order.
- Resolve Random once before showing a demo, then carry that concrete mode into
  camera setup. Never preview one image and silently reroll another game.
- Clear guide-only presentation before permission/setup begins so warnings,
  camera-card sizing, and readiness copy have one active owner.

---

## 13. Camera-Distance Action Cues Are Shape-First

A cue that is technically visible at arm's length may disappear once the player
steps back into the camera frame. During active motion, reading a sentence is a
competing task; the next action needs a silhouette before it needs explanation.

### Telegraph one action through redundant channels

- Give dodge, jump, squat, and clap distinct line-drawn silhouettes. Do not rely
  on emoji, font glyph coverage, text, or hazard color as the only identifier.
- Keep one concise uppercase verb as a redundant label. The pictogram, not the
  sentence, must carry the first-glance meaning from 1.5-2.5 metres.
- Show an obstacle lane as a three-slot spatial diagram. A `LEFT` arrow can be
  misread as “move left” even when it only means the hazard occupies that lane.
- Encode the selected lane with position, fill, outline weight, and height so it
  remains distinguishable under color-vision differences and projector washout.

### Make timing visible without adding another instruction

- Map deterministic obstacle distance to one clamped, monotonic proximity rail:
  empty at cue entry and full at the collision line.
- Keep the beacon presentation-only. Obstacle generation, evaluation, scoring,
  collision, and multiplayer decisions stay in the deterministic runtime.
- Avoid per-frame string formatting and unbounded animation state in the hot
  render loop; static labels and bounded primitives are sufficient.
- Layout tests prove containment, not physical readability. Record a real
  camera-distance run as pending until a permissioned session reaches gameplay.

---

## 14. Outcome Feedback Must Preserve the Beat Clock

The next action and the previous outcome compete for the same attention. Keep
the action beacon dominant, retain outcome feedback just long enough to register,
and never make a reward effect silently change the timing contract underneath
the run.

### Retain judgment only as bounded presentation

- The deterministic core may emit a one-frame success, beat pickup, or collision
  event. The renderer may retain that immutable event for 0.6-0.9 seconds, but
  the presentation pulse must not feed score, lives, combo, or network actions.
- Give outcomes distinct shapes plus short labels: check + `NICE`, spark +
  `BEAT`, and cross + positive recovery copy such as `NEXT`. Color and sound are
  redundant channels, not the sole explanation.
- In simultaneous play, use one bounded row with stable player identity accents.
  Do not stack blocking praise cards over the next action or let markers overlap
  each other on the smallest landscape viewport.
- Reduced-motion mode removes scale punch and screen shake but preserves the
  static outcome shape. A stationary AR screen cannot rely on device haptics;
  visual and audible siblings are required even when vibration is available.

### Keep persistent HUD quieter than transient action

- Persistent gameplay HUD should carry only player identity, score, remaining
  lives, and a meaningful combo. Hide `x0` and `x1`; reveal combo from `x2`.
- Encode lost lives with outline plus an X, not color alone. Prefer three compact
  pips over an 11px sentence such as `LIFE 2/3` at camera distance.
- Let temporary outcome feedback own celebration. Do not make every score tick,
  life value, and combo value animate continuously.

### Pitch feedback, never the backing track

- Pass the judged player's combo across the Rust/browser feedback boundary.
  Treat an import signature change as an ABI change and bump both version values.
- Apply one semitone per five combo, capped at four semitones, to the short
  success oscillator only. A collision resets to the unpitched recovery sound.
- Keep backing music playback rate at `1.0`. Rate-shifting the track changes its
  effective BPM while the beat clock still assumes 126 BPM, breaking obstacle,
  lighting, and audio alignment.
- Build and HTTP-load the release WASM after every feedback ABI change. A native
  test cannot prove browser imports or audible mix quality.

---

## 15. Every Run Entry Needs One Deterministic Breath

A camera-backed ready gesture leaves the player in motion and often away from a
stable neutral stance. Entering hazards on that same frame makes the game feel
unfair even when recognition is correct. Give first launch and replay one short,
visible preparation beat owned by the same deterministic runtime as gameplay.

### Separate start intent from world progression

- An evaluated ready signal should enter an explicit start countdown, never
  apply that same input to movement, collision, score, or beat judgment.
- Keep the world frozen for a two-second `2 / 1` countdown and begin progression
  on the following frame. This is a preparation beat, not a tutorial sequence.
- Route hands-free replay through the same start countdown. “Instant replay”
  means no menu or extra gesture; it does not require hazards to move on the
  clap frame.
- Duo starts only after both required players are evaluated and ready, then both
  receive the same shared countdown.

### Keep interruption semantics explicit

- Distinguish an initial `Starting` countdown from a tracking `Resuming`
  countdown even when they share presentation. If tracking disappears before a
  run begins, return to `Ready`; if it disappears during recovery, return to the
  hazard-clearing tracking hold.
- Keep countdown authority in the deterministic core. Browser setup may own
  permission, framing, and mode selection, but a DOM timer must not race the
  Rust phase or claim that gameplay has begun.
- Show one large `GET READY` panel with shape-dominant numerals. Hide HUD, next
  action, outcome markers, and beat meter until `Running` so the player sees one
  dominant instruction at camera distance.

### Make the breath a time shape, not another tutorial

- Draw `2` and `1` with fixed vector strokes instead of font glyphs so the
  preparation signal survives missing fonts, projector washout, and viewing
  distance.
- Derive a fixed twelve-tick ring directly from the authoritative remaining
  time. Drain monotonically inside each one-second numeral stage and reset only
  at the `2 -> 1` boundary; never add a renderer or DOM timer.
- Keep the ring, numeral, and one static `GET READY` label as the entire visual
  hierarchy. Do not add photography during this two-second safety breath even
  when canonical mode art is used on adjacent surfaces.
- Use fixed arrays and bounded loops in the render path. Geometry tests prove
  containment and stage mapping, not physical tempo or camera-distance
  legibility; those still require an attended start and resume run.

---

## 16. Control Gestures Need Their Own Deliberate Channel

An AR player should not have to break tracking or walk back to the device to
pause. At the same time, a control gesture must not be confused with a short
jump, rhythm clap, or scoreable pose. Treat pause as a deliberate meta action
with stronger temporal evidence than ordinary gameplay verbs.

### Make pause intentional and non-scoreable

- Use a one-second two-hand hold with both wrists high and separated. The dwell
  rejects a jump-sized transient; separation rejects an overhead clap.
- Reset dwell on release, low-quality tracking, player-identity change, or
  tracking loss. Emit one rising-edge pause trigger per completed hold.
- Reserve the entire matching dwell for the control channel. Suppress its
  constituent gameplay actions from the first matching frame; filtering only
  the eventual `Pause` bit still lets both-hand-up cues score during the hold.
- Keep `Pause` in the evaluated action stream for deterministic runner and P2P
  control, but filter it out before mirror/rhythm scoring, miss, and combo logic.
- Enter pause before applying any action or advancing beat, distance, score,
  collision, or hazards. Clear in-flight hazards so a long break cannot resume
  directly into a stale threat.

### Separate chosen pause from sensor failure

- Model `Paused` separately from `TrackingHold`. They may both freeze the world,
  but their cause, copy, and recovery destination are different.
- Resume an intentional pause with an evaluated clap and the same two-second
  preparation countdown used by other safe run entries. If tracking disappears
  during that countdown, return to `Paused`, not to a sensor-failure message.
- In Duo, latch each evaluated player's clap independently and wait for both.
  One player must never restart a shared physical run for the other.
- Show a large two-bar pause shape, `PAUSED`, and one clap instruction. Do not
  open settings, add a touch CTA, or cover the frozen state with another image.

### Let evaluation gate the resume instruction

- A paused run can outlive current tracking. Read evaluated-player facts before
  telling anyone to clap; a missing player first needs a short return-to-frame
  action.
- In single modes, follow the core's `any evaluated` rule. In Duo, keep P1 left
  and P2 right and require both local evaluation facts; remote participants do
  not replace either body.
- Show missing, present, and clap-ready with corner frame, neutral stance, and
  raised hand plus a geometric marker. A present player is not ready until the
  real clap flag latches.
- Keep one concise action under `PAUSED`: back into frame, clap to resume, or get
  ready. Avoid bullet-joined status sentences at camera distance.
- Preserve the no-photo pause hierarchy. Reuse the shared neon vector language,
  but let the universal two-bar safety symbol outrank mode illustration.

---

## 17. A Brain Break Needs a Positive Time Boundary

A skilled player should not have to fail deliberately or walk back to the
device to end a brain break. Treat session length as a deterministic gameplay
contract, not a DOM timeout or an analytics timestamp.

### Count only active physical play

- Use a bounded 60-90 second active-time clock owned by the deterministic core.
  Setup, ready countdowns, intentional pause, tracking hold, and recovery
  countdowns do not consume that budget.
- Complete before applying action, beat, distance, passive score, collision, or
  hazard progression on the boundary frame. Clear stale hazards as part of the
  result transition.
- Emit one immutable result event per run and consume it exactly once at the
  composition boundary. Replay resets session time/outcome but preserves bests.

### End positively and learn only aggregates

- Distinguish a full time-box (`BREAK COMPLETE`) from depleted energy
  (`NICE RUN`) without punishment language. Preserve lean-to-choose and
  clap-to-replay; completion adds no new touch step.
- Show session progress as a quiet shape rail beside the beat rail. Do not add a
  countdown sentence that competes with the next physical action.
- Persist only bounded per-mode completion and coarse outcome counters. Give
  completed runs more recommendation weight than starts, but never store frames,
  landmarks, body geometry, identities, or per-frame history.

---

## 18. Carry One Canonical Mode Image Through the Whole Journey

A mode should not change visual identity between launcher, camera setup, and
result choice. Reusing the same illustration reduces re-reading at camera
distance and makes a no-touch choice feel like returning to a known place rather
than entering another menu.

### Reuse ownership, not image copies

- Keep one canonical asset URL per mode and let every presentation surface load
  that source. Do not create result-only copies or embed already-served images in
  WASM; duplicate bytes weaken startup and cache behavior.
- Load render textures once outside the frame loop. Image decoding, path
  construction, and texture allocation are never per-frame work.
- Treat illustration as enhancement. A missing image falls back to a readable
  procedural card and must never block camera setup, gameplay, or replay.
- Use a center-cover crop rather than stretching square mode art into a wide or
  narrow result slot. Test crop math independently from the GPU.

### Keep capability truth above visual symmetry

- An illustrated choice that runtime will skip must look unavailable. Dim the
  art and add a lock shape plus a short reason; color or opacity alone is not
  sufficient.
- Derive availability from the same evaluated/capability facts used by
  deterministic navigation. Presentation may explain the rule but must not
  create a second mode-selection policy.
- Draw the selected outline after artwork so border weight and the geometric
  marker remain dominant in grayscale and under projector washout.
- Keep the existing far-distance verbs. Art continuity should remove cognitive
  load, not add a touch CTA, carousel pagination, or another instruction.

---

## 19. Sensor Recovery Is Not Intentional Pause

A player who deliberately pauses and a player who disappears from camera are in
different states even though both freeze gameplay. Reusing one `PAUSED` panel
makes the recovery action ambiguous and hides which required body the camera
still needs.

### Present runtime truth through a dedicated recovery presenter

- Derive recovery presentation from the same evaluated-player and latched-ready
  facts used by the deterministic runtime. Raw pose-array length and DOM
  visibility are weaker evidence and must not create a second policy.
- In single modes, honor the runtime's `any evaluated` capacity rule. In Duo,
  read required local P1/P2 independently; connected remote players never make a
  missing local Duo body look recovered.
- Keep `Searching`, `Present`, and `Ready` separate. A tracked neutral body is
  not ready until the real action flag latches.

### Explain reacquisition by shape, not error prose

- Use a corner-framed silhouette for a missing player, a neutral body for a
  restored player, and raised arm plus a geometric marker for readiness. Color
  reinforces these facts but never owns them.
- Use short recovery copy such as `FIND YOUR FRAME`, `TRACKING RESTORED`, and one
  next action. Reserve `PAUSED` and the two-bar pause glyph for deliberate pause.
- Carry the selected mode's canonical art through the frozen recovery surface,
  but keep the body cue dominant and retain a procedural fallback.
- Share bounded rendering primitives with Ready; keep Ready and TrackingHold as
  separate pure presenters so visual consistency does not merge semantics.
- Automated mapping/layout tests prove truth and containment. A physical
  lose/reacquire run is still required for timing and camera-distance proof.

---

## 20. Camera Intent Needs One Shape-First Action Compass

After the one allowed camera touch, the player is already stepping away from the
device. Setup navigation is therefore a camera-distance motion surface, not a
settings form. The selected mode image may own identity and the live preview may
own framing truth, but neither should compete with a sentence-heavy action bar.

### Let physical correction outrank navigation

- Map framing, lean, and confirmation through one typed presenter. Any missing,
  cropped, near/far, off-center, or unstable body correction must win before
  mode-change or raised-hand guidance.
- Use one short static correction in the live region. Do not rebuild visible
  percentages or emoji sentences on every pose callback; numeric hold progress
  can remain separate for geometry and accessibility.
- Dim navigation geometry during permission, model initialization, and framing
  recovery. A player cannot act on lean/hold cues until the camera path can
  evaluate them.

### Preserve image identity and make actions geometric

- Keep the same canonical neon mode image used by launcher and result. Add no
  setup-only illustration or duplicate image bytes.
- Use left/right chevrons for lean and a fixed raised-hand silhouette inside a
  radial hold ring. Color and text reinforce these shapes but never replace
  them; font arrows, emoji hands, and a linear percentage sentence are weaker at
  1.5-2.5 metres.
- Derive the ring only from the existing confirmation progress. Presentation
  must not create another timer, threshold, latch, or confirmation policy.

### Prove composition, not only containment

- A camera card and action card can both be inside the viewport and still cover
  each other. Responsive QA must check ownership regions and overlap, not merely
  rectangle bounds.
- In portrait, reserve an upper live-camera region and a lower action-card
  region. In short landscape, split horizontally so the action compass and live
  preview remain simultaneously visible.
- Use real CSS viewports for portrait and short landscape after the production
  build. Permission-state screenshots prove layout only; tracking, radial timing,
  and room-distance readability still require an attended camera session.

---

## 21. The One Required Touch Must Stay Above the Fold

A zero-touch journey can still fail before it begins. When camera permission is
the one deliberate touch, hiding that CTA behind a scroll turns scrolling into
an undocumented second interaction and makes every later hands-free guarantee
irrelevant.

### Reserve action space before decorating the launcher

- Validate the primary camera CTA at the shortest supported landscape height,
  not only on portrait phones and desktop. A mobile two-column grid can become
  two tall rows and push the only required action outside the initial viewport.
- Keep the CTA, one-line no-touch promise, local-only privacy truth, and honest
  guide-only escape visible together. Do not solve the fold by hiding permission
  truth or removing the fallback path.
- Treat an empty error live region as zero-height in the healthy state, while
  preserving its place and visibility when an error actually exists.

### Compact canonical art without inventing navigation

- At short landscape heights, place the existing canonical mode images in one
  equal-width rail. Prefer this direct four-choice overview to a carousel,
  pagination, horizontal scroll, or an additional mode-selection screen.
- Keep selected and unavailable states shape-backed. Generated illustrations
  carry mode identity; a geometric check or lock carries state more reliably
  than emoji whose weight and color vary by platform font.
- Remove only illustration that duplicates nearby setup truth at that viewport.
  The compact launcher may omit the large placement image when its concise
  distance/action reminder remains visible.

### Prove the actual first action

- Build the production CSS, load it over HTTP, and show portrait and 667x375
  launcher viewports side by side. Source rules alone do not prove that media
  query order, intrinsic image size, and copy wrapping preserve the fold.
- Verify the guide-only escape and mode selection still work in the compact
  composition. A screenshot with a visible CTA is incomplete evidence if the
  secondary recovery route or radio behavior was covered or disabled.
- Layout evidence does not prove camera permission, model startup, or attended
  distance readability. Report those runtime levels separately.

---

## 22. A Recovery Dialog Needs One Visual Owner

`aria-modal` describes interaction semantics; it does not automatically remove
the camera card, settings drawer, attribution badge, HUD, or canvas status that
remain visible behind a translucent dialog. A truthful fallback can still feel
like a broken overlay when several unrelated surfaces compete through its scrim.

### Let the recovery state own the viewport

- Use one explicit dialog-lifetime state to suppress unrelated interactive and
  status chrome. Do not independently guess visibility from camera, audio, or
  gameplay booleans on each hidden component.
- Keep the selected canonical mode image, one truthful limitation, one dominant
  recovery action, and one quieter exit. The ambient game canvas may remain
  dimmed, but it must not expose a competing instruction or control.
- Center a compact portrait recovery card when the full content fits. Bottom
  alignment that leaves settings and inactive camera status visible above the
  card creates two false focal regions.

### Make fallback actions platform-independent

- Use plain mode titles; the canonical illustration already owns identity.
  Platform emoji in a heading changes weight, baseline, and color across phones
  and is weaker than the image carried from launcher and result.
- Give camera return and mode return fixed geometric symbols while retaining
  semantic button text. Pseudo-element geometry stays present during async text
  changes and does not require another image request.
- Preserve a 60px dominant camera target and a 48px secondary return target.
  Guide-only is an accessibility/recovery route, not permission to shrink touch
  affordances or bury camera recovery.

### Prove modality as well as layout

- Browser evidence must check that technical chrome is absent from the visible
  and accessible dialog snapshot, not merely covered by a translucent layer.
- Tab must cycle only between the intended dialog actions, and Escape must
  return to the launcher with its selected mode focused. Visual cleanup cannot
  weaken keyboard recovery.
- Multiple WebGL iframes can produce screenshot-compositor artifacts. When a
  side-by-side capture looks clipped but the semantic tree is intact, rerun each
  viewport in isolation before diagnosing CSS or shipping a workaround.

## 23. Optional Technical Controls Must Collapse to One Shape

Advanced camera, audio, accessibility, and party controls are important recovery
paths, but their closed entry point is not a camera-distance gameplay action.
Carrying a two-line technical title into active play creates prose competition
with the body-scale cue even when the panel itself is closed.

### Keep the closed state quiet

- Collapse optional technical controls to one platform-independent geometric
  shape in a 48×48 target. Do not use an emoji whose weight, color, and baseline
  vary by platform.
- Keep the semantic label in the accessibility tree while visually hiding it
  only in the closed state. When the panel opens, show the plain label again so
  the technical surface has an explicit owner and heading.
- Do not hide or remove camera stop/start, audio, reduced-motion, opacity, or
  multiplayer recovery to make the interface look cleaner. Quiet entry and
  deleted capability are different product decisions.

### Let native disclosure own interaction

- Prefer native `details`/`summary` when it already owns the panel. Preserve
  pointer, Enter, Space, sequential focus, and a visible focus outline instead
  of adding a second JavaScript state machine.
- Closed width must stay 48px across desktop, compact portrait, and short
  landscape. Open width may expand to the readable panel contract and use
  bounded vertical scroll when height is scarce.
- A settings shape must remain below launcher, Ready, guide-only, and camera
  action owners in the visual hierarchy; it is optional support chrome, not a
  second navigation system.

### Validate both representations

- Measure computed panel and summary geometry at the target portrait and short-
  landscape viewports. A screenshot alone cannot prove the touch target.
- Exercise pointer, Enter, and Space against the production CSS, then verify the
  keyboard focus outline. Native semantics should be proven, not assumed.
- Check that opening reveals the complete existing control set and that short
  landscape scrolls without shrinking the panel back into an unreadable rail.

## 24. License Truth Needs One Technical Owner

A redistribution credit is required product truth, but a permanently floating
license card is not a camera-distance gameplay action. Keeping it on top of the
run makes important legal metadata unreadable at play distance while still
competing with body cues, camera state, and results.

### Place attribution beside the capability it explains

- Keep track title, tempo, artist, license, source link, and modification notice
  together with the audio control inside the existing technical surface. Do not
  create a second disclosure or a separate floating card.
- Preserve stable element identifiers when browser code updates playback status.
  Presentation ownership can move without changing audio engine ownership.
- Keep the authoritative attribution file, fingerprinted local audio, source
  URL, license URL, and modification evidence unchanged unless the actual asset
  changes. UI cleanup must not weaken release evidence.

### Hide chrome, not evidence

- When native settings are closed, all of their content—including attribution—
  leaves the gameplay hierarchy together. Opening settings must show the full
  credit, not an abbreviated label that requires another navigation step.
- Give external artist and license links usable touch targets and preserve safe
  new-tab attributes. Technical links remain operable even though they are not
  primary gameplay actions.
- Short landscape should scroll the same complete technical surface instead of
  applying a special rule that hides attribution entirely.

### Prove release truth and presentation separately

- Use production-CSS browser evidence to compare closed and open states at
  portrait and short landscape. Confirm native disclosure visibility, link
  target size, and bounded scroll.
- Boot the real production entry after moving a status node. A typecheck cannot
  prove a non-null DOM query still resolves at runtime.
- Probe the fingerprinted audio MIME and verify its attribution document remains
  in the built bundle. A clean layout screenshot is not license-delivery proof.

## 25. `aria-modal` Is A Claim, Not A Boundary

A launcher can look like a full-screen dialog and still leak interaction to a
background WebGL canvas. Semantic attributes describe intent; they do not hide
technical chrome, remove focus targets, or coordinate sibling state machines.

### Give the shell one presentation owner

- Map launcher, Ready, guide, and gameplay to one explicit owner state. That
  owner should decide all gate visibility and body presentation classes instead
  of letting each handler add and remove siblings independently.
- Block the canvas and optional settings for every pre-game modal, then remove
  the boundary only when gameplay owns the shell. This prevents fixing launcher
  focus while accidentally reopening the same leak in Ready.
- Carry first-paint owner state in HTML. A module cannot prevent technical chrome
  flashing or receiving early focus before it has executed.

### Separate visibility, layout, and interaction

- Use inert attributes for interaction exclusion and toggle the attributes
  directly when runtime DOM-property support is uncertain. Test attribute removal
  as well as insertion so gameplay does not inherit a permanent first-paint lock.
- Hide launcher-owned canvas visibility without `display:none`. Macroquad keeps a
  real layout box and rendering surface while stale gameplay labels cannot bleed
  through a translucent scrim.
- Keep Ready's camera surface visible even while canvas/settings remain inert.
  Modal ownership is state-specific, not a blanket rule to hide every sibling.

### Prove the boundary with input and delayed resources

- Record repeated Tab and Shift+Tab sequences on the real production entry.
  Checking only `aria-modal` or a semantic snapshot cannot expose a focusable
  canvas underneath.
- Exercise cross-modal transitions: launcher to guide and back, Ready failure to
  launcher, and confirmed play to gameplay. One clean initial screenshot is not
  evidence that ownership stays coherent.
- Observe fresh-origin requests before any input. If a delayed vision/audio load
  appears after prior QA, reproduce on a new origin before changing capability
  code; automation input can be the real trigger.

## 26. Secondary Hierarchy Must Not Depend On Illegibility

A camera-first launcher needs one honest route for denial, missing hardware, or
preview. Making that route faint does not simplify the interface; it hides the
only truthful recovery action and turns hierarchy into a visual-accessibility bug.

### Use shape and area to establish priority

- Let the primary action own full width, filled color, larger type, and dominant
  placement. Keep the fallback compact, transparent, underlined, and later in
  reading order.
- Do not reduce secondary text below readable contrast or apply opacity to the
  whole control. Icon, text, underline, and focus affordance all fade together.
- Preserve a 48px secondary target even when its visual ink stays quiet. Small
  visual weight and small interactive area are different decisions.

### State the consequence before navigation

- Name the path and the evaluation truth in one short label: demo, no camera,
  and no score. Do not wait until the next dialog to reveal that the path is not
  evaluated play.
- Keep one secondary action rather than adding separate “learn more”, “skip”,
  and “deny” choices. Honest copy can remove decision ambiguity without adding
  another touch.
- Preserve the same guide action and focus owner behind the label. Presentation
  clarity must not fork camera/audio/capability state.

### Measure effective presentation

- Include element opacity when calculating contrast; checking the raw hex token
  alone can overstate readability substantially.
- Validate computed font size, weight, opacity, target geometry, scroll geometry,
  and forward/reverse focus order on the production bundle.
- Render the final label at portrait and short landscape. A longer truthful label
  is only an improvement if it stays complete, bounded, and visibly secondary.

## 27. Ambient Backdrops Must Stop Before Runtime Chrome

A live game canvas can provide valuable visual continuity behind a setup or
guide dialog, but every readable label, avatar, and status silhouette makes it a
second content owner. Preserve ambience by geometry, not by lowering the opacity
of the whole runtime until its messages are merely difficult to read.

### Separate the visual jobs

- Use a centered radial layer for the useful world motif, such as a sun, lane,
  or horizon. Keep that motif subordinate to the canonical guide illustration.
- Use an independent bottom fade for players, hazards, and HUD silhouettes. A
  radial stop that hides corners can still reopen the lower center.
- Use symmetric side masks for corner labels. Radial geometry changes with
  aspect ratio, so a clean desktop result does not prove narrow viewports.

### Preserve lifecycle while enforcing ownership

- Keep the canvas laid out and rendering when the next transition depends on it;
  make the shell inert and cover semantic chrome instead of adding a duplicate
  Rust backdrop state.
- Let exactly one dialog own guide truth and actions. Ambient pixels may connect
  worlds, but they must not compete as instructions, status, or controls.
- Restore focus after a hidden parent becomes visible. When `display:none`
  changes in the same tick, use the next animation frame and prove the real
  active element rather than trusting the intended `.focus()` call.

### Validate the geometry iteratively

- Capture desktop, portrait, and short landscape. Treat faint readable labels or
  recognizable player heads as leaks, not acceptable decoration.
- Retry known dual-WebGL compositor clipping before editing layout. A clean
  delayed capture distinguishes tooling artifacts from deterministic CSS bugs.
- Observe the fresh-origin resource log as well as the pixels: guide ambience
  must not turn preview intent into a vision/camera load.

## 28. Invisible Loading UI Is Still A Screen

A loading layer hidden behind a higher-z launcher can be visually irrelevant
while remaining a second heading and status surface for assistive technology.
Opacity and pointer-event suppression change pixels and hit testing; they do not
remove semantic ownership.

### Delete dead surfaces instead of stacking hiding mechanisms

- If the launcher already owns first paint, covers the viewport, preserves the
  canvas layout box, and starts the real loader, a lower loading screen has no
  product role. Remove its markup, styles, animation, and timeout together.
- Do not add `aria-hidden`, inert, or another delayed class merely to preserve a
  screen nobody can see or use. Each extra mechanism creates lifecycle drift.
- If loading feedback is genuinely required later, model it as an explicit shell
  owner with truthful progress/error transitions—not as a sibling behind the
  active owner.

### Prove visual and semantic first paint separately

- Capture the accessibility snapshot immediately and after every hiding timer.
  A clean screenshot cannot prove that duplicate headings/status are absent.
- Inspect the actual canvas dimensions, inert state, and WASM/loader requests
  after removal. Dead UI can be deleted only when its claimed layout/startup role
  is already owned elsewhere.
- Recheck portrait and short landscape even for deletion-only changes. Removing
  a positioned layer should not alter the illustrated launcher composition.

### Treat deletion as measurable simplification

- Record removed timers, animation rules, and shipped HTML/JS/CSS bytes. Simpler
  UI is strongest when it reduces both cognitive and runtime surface.
- Add a small entry contract test so legacy copy cannot silently return outside
  the shell-owner state model.

## 29. Camera Failure Copy Must Preserve The Next Action

A safe camera failure can still strand a family when every rejection collapses
to a generic technical sentence. Recovery copy should explain the likely cause
at the browser boundary while keeping the existing retry and honest preview
paths obvious, bounded, and keyboard reachable.

### Translate exceptions into product reasons

- Map permission/security, missing-device, busy-device, and unknown startup
  errors to a small typed reason set. Never render adapter exception messages,
  browser internals, device labels, or stack detail.
- Phrase each message as cause plus next action: allow and retry, close the other
  camera app and retry, or use the no-camera/no-score preview.
- Keep the unknown case honest. Offer retry or preview without inventing a cause
  the runtime cannot prove.

### Reuse recovery paths instead of adding controls

- Present failure as one informational rail, not a new dialog or button. The
  existing filled camera action remains retry; the existing guide-only action
  remains truthful fallback.
- Fail closed before changing presentation: stop partial tracks, disable camera
  evaluation, clear pending state, then reveal the launcher.
- Restore focus to the retry action on the next animation frame. This is required
  when failure began inside a guide whose focused control becomes hidden.

### Prove the longest failure, not only the happy path

- Exercise deterministic permission, missing, busy, and unknown errors at the
  adapter boundary. Assert the retry call count and no-stream/evaluation state,
  not only the displayed words.
- Measure the longest localized message at 390x844 and 667x375 with safe lower
  tolerance. Do not recover space by shrinking a 48px action or hiding privacy
  and no-touch truth.
- Verify forward/reverse focus after failure and the guide-originated retry path.
  Keep physical permission prompts as explicit attended QA; a mocked
  `DOMException` proves application recovery, not operating-system UI.

## 30. Framing Guidance Belongs On The Live Body Surface

Canonical game art explains which activity was chosen; it cannot explain where
the player's body is relative to the current camera frame. When actionable
framing copy is placed on mode art, the player must look away from the evidence
they are trying to correct—and compact layouts may hide the copy with the art.

### Give the preview one live status owner

- Put permission, initialization, calibration, framing, and hold/navigation
  status on the actual mirrored preview. Keep one polite status node rather than
  exposing generic camera text plus a second actionable live region on game art.
- Before tracking, prefer the action the family can take over backend detail.
  After tracking begins, yield status ownership to pose/framing navigation so a
  later generic player-count callback cannot overwrite the physical correction.
- Keep local-only privacy truth visible beside the camera status. Consolidating
  copy must not make the camera boundary less explicit.

### Let canonical art own identity only

- Preserve the selected mode image and title in Ready as visual continuity, but
  do not attach camera permission, framing, or tracking truth to that image.
- Compact short landscape may hide decorative art for height, but it must never
  hide the actionable camera status with it. Prove the status has a non-zero box
  when the art container is intentionally zero-size.
- Reuse existing art and action geometry; status ownership does not justify a
  new illustration, tutorial screen, button, timer, or touch step.

### Bound the two physical reference surfaces

- On wide screens, cluster the Ready card and live preview inside one centered
  stage. Two contained cards separated by a thousand empty pixels are not one
  readable interaction.
- Preserve stacked portrait and split short-landscape regions where they keep
  body view and action compass simultaneously visible. Do not force one desktop
  grid across all aspect ratios.
- Use a synthetic MediaStream to validate deterministic layout/model handoff,
  status visibility, and responsive geometry. Keep physical camera alignment,
  gesture timing, and room-distance readability as attended claims.

## 31. A Visible Ready Screen Must Own Focus And Cancellation

An `aria-modal` Ready surface is not a navigation boundary by itself. If the
launcher action becomes hidden or disabled while focus stays behind it, the
screen can look correct and still strand keyboard and assistive users.

### Move focus with shell ownership

- Make the dialog programmatically focusable and move focus after it becomes
  visible. Treat the accessibility snapshot's active node as runtime evidence,
  not an inference from `role="dialog"`.
- Trap forward and reverse Tab across elements that are currently rendered.
  Controls inside collapsed details or disabled recovery actions are not valid
  stops merely because they exist in the DOM.
- Keep the dialog itself as the stable initial focus target so no extra visible
  close button or touch action is needed for no-touch gameplay.

### Escape must cancel the device request

- Returning to the launcher must invalidate the whole camera-start attempt,
  stop evaluation/resources, restore the primary action, and move focus there.
  Hiding Ready without cancelling its async work creates split ownership.
- `getUserMedia()` has no portable AbortSignal. Race it against the UI session;
  if permission resolves late, stop all returned tracks before they can be
  attached or publish tracking state.
- Test cancellation both while permission is pending and after tracking starts.
  A unit test for late-stream disposal plus an HTTP-served browser snapshot
  covers different failure surfaces; neither substitutes for the other.

## 32. The AR Stage Must Not Invent A Player

Camera status, HUD, and stage avatars are three views of one participant model.
If any one of them renders an unevaluated body, the experience stops feeling
camera-backed even when tracking and scoring remain technically correct. The
ambiguity is especially severe in portrait, where two center-lane silhouettes
can overlap and look like unstable body recognition.

### Derive presence from mode and evaluation truth

- Keep P1 visible as the primary setup/player anchor. In single-player modes,
  reveal every additional slot only when its authoritative runner evaluation is
  true.
- Duo Groove may reserve P1 and P2 visibly because the selected mode explicitly
  requires two local people. Additional local or remote slots still require
  evaluation truth.
- Centralize the rule as one pure visibility policy consumed by rendering.
  Avatar type, pose projection, lane physics, HUD, and scoring must not each
  invent their own participant count.

### Prove agreement on the real camera path

- Reach gameplay through the production camera/model handoff and compare the
  live status, HUD identities, and stage bodies in the same frame.
- Validate at both desktop and 390x844 portrait. A false body that looks merely
  decorative on desktop can directly cover the tracked player on a phone.
- A synthetic camera image can prove browser wiring and rendered presence. Real
  two-person entry order, crossing, departure, and identity stability remain
  attended QA and must not be inferred from the synthetic case.

### Keep presence truth separate from layout polish

- Removing a false participant does not authorize moving the remaining avatar,
  HUD, or camera preview in the same milestone. Record any overlap that remains
  as a separate spatial-hierarchy problem.
- Prefer removing unsupported procedural shapes over adding explanatory copy,
  another legend, or a touch control. In an AR surface, fewer truthful marks are
  stronger than more instructions.

## 33. The Body Lane And Persistent HUD Need Separate Floors

A bottom-centered runner and a bottom-centered status card cannot both own the
same pixels. Making the card translucent or moving it to a corner only changes
which pose or lane it covers. Active body motion needs an uninterrupted stage;
persistent score state needs a passive control floor outside that stage.

### Partition the running surface once

- Derive the stage, HUD deck, and player-card layout together from viewport size
  and displayed-player count. Do not let renderer and HUD independently assume
  they own the full canvas.
- End the running stage above the first HUD row, then paint an opaque quiet deck
  after the world and before the cards. Painter order makes the deck a truthful
  boundary even if pose limbs or crash shake extend beyond nominal stage bounds.
- Keep non-running states full-screen. Ready, countdown, pause, tracking hold,
  and result already hide the player HUD and should not inherit unused deck space.

### Preserve motion area before decorating the deck

- Retain at least 60 percent of the short-landscape viewport for the stage with
  one through four cards. The deck must solve occlusion without turning gameplay
  into a letterboxed thumbnail.
- Use one low-emphasis neon boundary and the existing identity/score/life/combo
  cards. Do not add labels, instructions, preferences, buttons, or new art to
  explain a spatial separation that geometry can make obvious.
- Keep the HUD fixed. A card that follows the player or jumps between corners is
  another moving target during whole-body play.

### Validate pixels, not only rectangles

- Prove containment for 390x844, 667x375, and desktop layouts with one through
  four cards, then traverse the HTTP-served production camera/model path.
- Compare the avatar, foot identity, nearest lane line, boundary, and HUD in the
  same frame at portrait and desktop sizes. Pure layout tests cannot reveal a
  pose limb drawn beyond its nominal bounds.
- Record other overlays that still compete with action guidance separately. A
  clean lower deck does not prove the camera PIP or top action beacon hierarchy.
