# Rule: Motion Game Taste & Zero-Touch UI Architecture

**Category:** Game Design & Experience Architecture  
**Scope:** `brainbreak/` & All Camera-Controlled Motion Games

---

## 1. Core Mindset & Principles

1. **Self-Improvement & Continuous Skillization**:
   - Every agent interaction MUST distill learned experience, feedback, and design patterns into persistent Skills (`refenrece/skills/`) and Rules (`refenrece/rules/`).
2. **LÀM ÍT - ĐƠN GIẢN - NGẮN GỌN - CHẤT LƯƠNG CAO (Do Less, Keep Simple, High Quality)**:
   - Avoid visual clutter, multi-level menus, or excessive text instructions.
   - Focus on ultra-refined, high-impact micro-interactions.
3. **LUÔN TRẢ LỜI ĐỦ LÝ DO MỚI BẮT ĐẦU LÀM (Always Explain Rationale First)**:
   - Present the strategic design rationale (WHY, WHAT, HOW) and confirm user alignment BEFORE performing any code mutations.

---

## 2. Zero-Touch UI Contract (Hợp Đồng Không Chạm)

- **Far-Distance Autonomy**: Users standing 1.5m – 2.5m away must NEVER be forced to walk to the device to tap the screen.
- **Far-Distance Gestures**:
  - 🖐️ **High Hand Hold (1.0s)**: Fills AR radial gauge to confirm menu selection.
  - 👏 **Double Clap**: Triggers instant hands-free Start / Retry / Combo Boost.
  - ↔️ **Body Lean**: Changes lanes / shifts posture target.
- **Multiplayer Synchronized Ready**: In 2-player mode, BOTH players must be detected in frame AND BOTH must perform ready gestures before starting.
- **Deterministic Run Entry**: First launch and hands-free replay enter one
  runtime-owned two-second countdown. The ready gesture must never also advance
  hazards or score on the same frame; interrupted first starts return to Ready,
  while interrupted tracking resumes return to the safety hold.
- **Intentional Hands-Free Pause**: Holding both hands high and separated for one
  second pauses before progression. Reserve the entire matching dwell for the
  control channel so its hand-up components cannot score. Pause is a
  camera-evaluated control action, never a scoring/miss action; clap resumes
  through the deterministic countdown, and Duo waits for both players.
- **Positive Time-Box**: A BrainBreak run completes after 90 active `Running`
  seconds. Setup, pause, and tracking recovery freeze the clock; completion wins
  the boundary before collision or score progression. The result remains
  hands-free and only bounded mode/outcome aggregates may shape future taste.
- **Canonical Mode Art**: Reuse one mode illustration from launcher through
  camera setup and result choice. Load from the shared URL once, crop without
  stretching, and keep a procedural fallback so missing art never blocks play.
- **Truthful Illustrated Choices**: If deterministic navigation skips a mode,
  dim its art and add a lock shape plus a short reason. Visual symmetry must not
  imply an action the runtime will reject.

---

## 3. Sensory Ritual & Audio-Lighting Synergy

- **Sensory Ritual**: Transform 60-90 second physical breaks into a multi-sensory Awe-inducing ritual combining EDM drops, AR particle aura, fluid dynamics, and generative art.
- **Quantized Audio-Visual Synergy**:
  - Actions landing on the downbeat (±100ms) trigger quantized punchy SFX, pitch scaling (+1 semitone per 5 combo tier), and neon road/sun lighting pulses synced to track BPM.
  - AR particle trails follow player wrists and feet in real time.
