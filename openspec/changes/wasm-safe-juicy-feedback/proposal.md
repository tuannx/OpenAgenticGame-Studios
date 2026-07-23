# WASM-safe Juicy Game feedback

**Status**: shipped (soft-glow path) · lightmap/bloom gated  
**Date**: 2026-07-23

## WHY

Kids at 1.5–4m need instant physical feedback (bounce, contact light, micro shake).
Juice was split across Runner particles and Supernova confetti; crash shake was
one-frame only. A full lightmap/bloom postprocess is high-risk on Macroquad WASM
(mid-frame `render_target` + `set_camera` re-enters miniquad's event-handler
RefCell → panic at `wasm.rs:34`).

## WHAT shipped

Shared module `brainbreak-game/src/juice.rs`, reused by Runner + Supernova:

| System | Behavior | Triggers |
|---|---|---|
| **ParticlePool** (256 fixed slots) | Soft concentric glow discs (fake additive) | Jump, dodge, crash, beat pickup, clap/hit, freeze, drop |
| **ScreenShake** | Intensity + decay; zero when reduce-motion | Crash, beat, jump, drop, perfect freeze, on-beat drum, imminent tremble |
| **SpringScale** | Damped squash/stretch | Runner body + feedback markers; Supernova core/cues/result burst |
| **Neon silhouette** | Wrist→elbow→shoulder→hip→knee→ankle ribbons | Runner pose avatar; Supernova pose overlay behind mascot |

Existing sensory juice kept: edge wash, confetti, downbeat kick, elemental VFX.

## Deferred / gated (WASM safety)

| Feature | Verdict | Why |
|---|---|---|
| Quarter-res lightmap RT + multiply composite | **GATED** | Mid-frame RT/`set_camera` panic history |
| Gaussian bloom via `_ScreenTexture` postprocess | **GATED** | Same stack risk; not proven WASM-safe here |
| `macroquad-particles` crate | Skipped | Not a dep; fixed pool is enough and smaller |

Fake glow path (A) is the product path for this slice. Path (B) only if a
frame-boundary-safe RT path is proven later (or `cfg(not(wasm32))` desktop-only).

## Files

- `brainbreak/crates/brainbreak-game/src/juice.rs` — shared juice
- `brainbreak/crates/brainbreak-game/src/visuals.rs` — Runner wiring
- `brainbreak/crates/brainbreak-game/src/supernova_render.rs` — Supernova wiring
- `brainbreak/crates/brainbreak-game/src/main.rs` — shake offset + pose pass-through

## Feel-test map

### Neon Beat Runner
- **Jump** → cyan glow burst + body squash punch + light shake
- **Dodge** → green hit burst + spring pop
- **Crash** → red burst + stronger decaying shake + squash inward
- **Beat clap pickup** → purple burst + shake
- **Pose** → neon limb ribbons (not joint dots)

### Supernova Freeze Party
- **On-beat dance hit / clap** → gold clap burst + core spring + shake
- **Enter Freeze / perfect freeze** → ice glow burst + cue spring
- **Drop** → confetti + orange drop glow + heavy shake + hit-stop
- **Result** → spring-punched star burst icons
- **Pose overlay** → neon silhouette behind mascot while dancing/freezing

Reduce-motion: no shake; smaller particles; springs still settle toward 1.0.

## Validation

- `cargo test -p brainbreak-game -p brainbreak-core`
- Release WASM rebuild + Cloudflare deploy (see tasks)
