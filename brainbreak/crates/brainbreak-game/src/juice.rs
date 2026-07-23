//! Shared WASM-safe juicy feedback for Runner + Supernova.
//!
//! Glow is fake-additive soft circles (layered translucent discs). Mid-frame
//! `render_target` / `set_camera` lightmap and screen-texture bloom are gated —
//! they re-enter miniquad's WASM event-handler RefCell and panic.

use brainbreak_core::PoseFrame;
use macroquad::prelude::*;

const PARTICLE_CAPACITY: usize = 128;
const TAU: f32 = std::f32::consts::TAU;

/// Hero pictogram / cue / soft-button edge length for Zero-Touch readability (1.5–4m).
///
/// Contract: shorter edge ≥ 20% of `min(screen_w, screen_h)`.
/// Applies to guidance pictograms AND player-facing buttons/cards (shell, Ready,
/// settings, WASM result chips). We ship 28% so one target dominates; area≥20%
/// of the full screen would force ~45% sides and crush Ready compass + framing
/// on short landscape.
pub const HERO_EDGE_MIN_FRACTION: f32 = 0.20;
pub const HERO_EDGE_SHIP_FRACTION: f32 = 0.28;

pub fn hero_edge(screen_w: f32, screen_h: f32) -> f32 {
    screen_w.min(screen_h) * HERO_EDGE_SHIP_FRACTION
}

pub fn hero_edge_meets_contract(edge: f32, screen_w: f32, screen_h: f32) -> bool {
    edge + 0.5 >= screen_w.min(screen_h) * HERO_EDGE_MIN_FRACTION
}

/// Limb chains readable at 1.5–4m: wrist→elbow→shoulder→hip→knee→ankle.
const SILHOUETTE_CHAINS: [&[usize]; 4] = [
    &[9, 7, 5, 11, 13, 15],
    &[10, 8, 6, 12, 14, 16],
    &[5, 6],
    &[11, 12],
];

#[derive(Clone, Copy, Debug, Default)]
struct GlowParticle {
    position: Vec2,
    velocity: Vec2,
    life: f32,
    max_life: f32,
    size: f32,
    color: Color,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BurstKind {
    Jump,
    Clap,
    Freeze,
    Drop,
    Hit,
    Crash,
    Beat,
}

impl BurstKind {
    fn palette(self) -> Color {
        match self {
            Self::Jump => Color::from_rgba(103, 232, 249, 255),
            Self::Clap => Color::from_rgba(255, 202, 58, 255),
            Self::Freeze => Color::from_rgba(140, 220, 255, 255),
            Self::Drop => Color::from_rgba(255, 140, 66, 255),
            Self::Hit => Color::from_rgba(138, 201, 38, 255),
            Self::Crash => Color::from_rgba(251, 113, 133, 255),
            Self::Beat => Color::from_rgba(196, 181, 253, 255),
        }
    }

    fn count(self) -> usize {
        // Fewer particles, bigger when they fire (Ít đồ / CHẤT).
        match self {
            Self::Drop => 18,
            Self::Crash => 14,
            Self::Hit | Self::Beat | Self::Clap => 10,
            Self::Jump | Self::Freeze => 8,
        }
    }

    fn speed(self) -> f32 {
        match self {
            Self::Drop => 0.52,
            Self::Crash => 0.34,
            Self::Hit | Self::Beat => 0.28,
            Self::Clap | Self::Jump => 0.24,
            Self::Freeze => 0.14,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ParticlePool {
    particles: [GlowParticle; PARTICLE_CAPACITY],
    cursor: usize,
}

impl Default for ParticlePool {
    fn default() -> Self {
        Self {
            particles: std::array::from_fn(|_| GlowParticle::default()),
            cursor: 0,
        }
    }
}

impl ParticlePool {
    pub fn update(&mut self, dt: f32) {
        let dt = dt.max(0.0);
        for particle in &mut self.particles {
            if particle.life <= 0.0 {
                continue;
            }
            particle.life = (particle.life - dt).max(0.0);
            particle.position += particle.velocity * dt;
            particle.velocity *= 1.0 - (2.4 * dt).min(0.85);
            particle.velocity.y += dt * 0.18;
        }
    }

    pub fn clear(&mut self) {
        for particle in &mut self.particles {
            particle.life = 0.0;
        }
        self.cursor = 0;
    }

    pub fn spawn_burst(&mut self, origin: Vec2, kind: BurstKind, scale: f32) {
        let color = kind.palette();
        let count = kind.count();
        let base_speed = kind.speed() * scale.max(0.35);
        for index in 0..count {
            let angle = index as f32 / count as f32 * TAU + (index % 5) as f32 * 0.17;
            let speed = base_speed * (0.55 + (index % 7) as f32 * 0.09);
            let life = 0.38 + (index % 5) as f32 * 0.08;
            self.particles[self.cursor] = GlowParticle {
                position: origin,
                velocity: vec2(angle.cos() * speed, angle.sin() * speed - base_speed * 0.35),
                life,
                max_life: life,
                size: (6.5 + (index % 4) as f32 * 3.2) * scale.clamp(0.8, 2.4),
                color,
            };
            self.cursor = (self.cursor + 1) % PARTICLE_CAPACITY;
        }
    }

    /// Screen-space burst (absolute pixels) used by Supernova.
    pub fn spawn_screen_burst(&mut self, origin: Vec2, kind: BurstKind, scale: f32) {
        let color = kind.palette();
        let count = kind.count();
        let base_speed = kind.speed() * 640.0 * scale.max(0.35);
        for index in 0..count {
            let angle = index as f32 / count as f32 * TAU + (index % 5) as f32 * 0.17;
            let speed = base_speed * (0.55 + (index % 7) as f32 * 0.09);
            let life = 0.42 + (index % 5) as f32 * 0.09;
            self.particles[self.cursor] = GlowParticle {
                position: origin,
                velocity: vec2(angle.cos() * speed, angle.sin() * speed - base_speed * 0.28),
                life,
                max_life: life,
                size: (10.0 + (index % 4) as f32 * 4.5) * scale.clamp(0.8, 2.4),
                color,
            };
            self.cursor = (self.cursor + 1) % PARTICLE_CAPACITY;
        }
    }

    pub fn draw_normalized(&self, bounds: Rect, reduce_motion: bool, energy: f32) {
        let size_mul = if reduce_motion {
            0.5
        } else {
            1.0 + energy * 0.55
        };
        for particle in &self.particles {
            if particle.life <= 0.0 {
                continue;
            }
            let t = (particle.life / particle.max_life.max(0.001)).clamp(0.0, 1.0);
            let position = vec2(
                bounds.x + particle.position.x * bounds.w,
                bounds.y + particle.position.y * bounds.h,
            );
            draw_soft_glow(
                position,
                particle.size * size_mul * (0.45 + t),
                with_alpha(particle.color, t * 0.9),
                if reduce_motion { 2 } else { 3 },
            );
        }
    }

    pub fn draw_screen(&self, reduce_motion: bool) {
        let size_mul = if reduce_motion { 0.55 } else { 1.0 };
        for particle in &self.particles {
            if particle.life <= 0.0 {
                continue;
            }
            let t = (particle.life / particle.max_life.max(0.001)).clamp(0.0, 1.0);
            draw_soft_glow(
                particle.position,
                particle.size * size_mul * (0.45 + t),
                with_alpha(particle.color, t * 0.92),
                if reduce_motion { 2 } else { 3 },
            );
        }
    }

    #[cfg(test)]
    pub fn live_count(&self) -> usize {
        self.particles.iter().filter(|p| p.life > 0.0).count()
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ScreenShake {
    magnitude: f32,
    decay_per_sec: f32,
}

impl ScreenShake {
    pub fn with_decay(decay_per_sec: f32) -> Self {
        Self {
            magnitude: 0.0,
            decay_per_sec: decay_per_sec.max(1.0),
        }
    }

    pub fn impulse(&mut self, amount: f32) {
        self.magnitude = self.magnitude.max(amount.max(0.0));
    }

    pub fn update(&mut self, dt: f32) {
        self.magnitude = (self.magnitude - self.decay_per_sec * dt.max(0.0)).max(0.0);
    }

    #[cfg(test)]
    pub fn magnitude(self) -> f32 {
        self.magnitude
    }

    pub fn offset(self, time: f32, reduce_motion: bool) -> Vec2 {
        if reduce_motion || self.magnitude < 0.05 {
            return Vec2::ZERO;
        }
        vec2(
            (time * 91.0).sin() * self.magnitude,
            (time * 73.0).cos() * self.magnitude * 0.62,
        )
    }
}

/// Damped spring scale for squash & stretch pops.
#[derive(Clone, Copy, Debug)]
pub struct SpringScale {
    pub value: f32,
    velocity: f32,
    target: f32,
    stiffness: f32,
    damping: f32,
}

impl Default for SpringScale {
    fn default() -> Self {
        Self::new(1.0)
    }
}

impl SpringScale {
    pub fn new(value: f32) -> Self {
        Self {
            value,
            velocity: 0.0,
            target: value,
            stiffness: 96.0,
            damping: 14.0,
        }
    }

    pub fn punch(&mut self, impulse: f32) {
        // Exaggerated cartoon squash — funny weight at camera distance.
        self.velocity += impulse * 1.35;
    }

    pub fn update(&mut self, dt: f32) {
        let dt = dt.clamp(0.0, 0.05);
        let force = (self.target - self.value) * self.stiffness - self.velocity * self.damping;
        self.velocity += force * dt;
        self.value += self.velocity * dt;
    }
}

/// Concentric soft discs — fake additive bloom without render targets.
pub fn draw_soft_glow(center: Vec2, radius: f32, color: Color, layers: u8) {
    let layers = layers.max(1);
    for layer in (0..layers).rev() {
        let t = (layer + 1) as f32 / layers as f32;
        let alpha = color.a * (0.18 + 0.28 * (1.0 - t));
        draw_circle(
            center.x,
            center.y,
            radius * (0.55 + t * 0.85),
            with_alpha(color, alpha),
        );
    }
    draw_circle(
        center.x,
        center.y,
        radius * 0.42,
        with_alpha(Color::new(1.0, 1.0, 1.0, 1.0), color.a * 0.55),
    );
}

/// Hot pink for the live user character overlay (“you”).
pub const USER_SKELETON_PINK: Color = Color::new(1.0, 0.28, 0.78, 1.0);
/// Base opacity for guide + user character overlays (~40% each).
pub const USER_SKELETON_ALPHA: f32 = 0.40;
/// Extra glow radius multiplier for pink neon aura around the user figure.
pub const USER_SKELETON_GLOW_MUL: f32 = 1.65;

/// Filled cartoon limb (capsule): thick stroke + round joints — reads as animation, not bones.
pub fn draw_filled_limb(from: Vec2, to: Vec2, thickness: f32, color: Color) {
    draw_line(from.x, from.y, to.x, to.y, thickness, color);
    let joint = thickness * 0.52;
    draw_circle(from.x, from.y, joint, color);
    draw_circle(to.x, to.y, joint, color);
}

/// Soft pink aura disc behind a character (WASM-safe concentric glows).
pub fn draw_character_aura(center: Vec2, radius: f32, color: Color, flash: f32) {
    draw_soft_glow(
        center,
        radius * (1.15 + flash * 0.2) * USER_SKELETON_GLOW_MUL,
        with_alpha(color, 0.22 + flash * 0.12),
        4,
    );
    draw_soft_glow(
        center,
        radius * 0.72,
        with_alpha(color, 0.16 + flash * 0.08),
        2,
    );
}

/// Shared stage pulse rings + sparse sparkles around the overlapped figures.
pub fn draw_shared_stage_fx(
    center: Vec2,
    radius: f32,
    time: f32,
    audio_pulse: f32,
    accent: Color,
    reduce_motion: bool,
) {
    if reduce_motion {
        draw_ellipse(
            center.x,
            center.y + radius * 0.72,
            radius * 0.55,
            radius * 0.12,
            0.0,
            with_alpha(accent, 0.12),
        );
        return;
    }
    let pulse = audio_pulse.clamp(0.0, 1.0);
    draw_ellipse(
        center.x,
        center.y + radius * 0.78,
        radius * (0.52 + pulse * 0.08),
        radius * (0.11 + pulse * 0.03),
        0.0,
        with_alpha(accent, 0.14 + pulse * 0.12),
    );
    for ring in 0..2 {
        let t = ((time * 0.55 + ring as f32 * 0.5) % 1.0).clamp(0.0, 1.0);
        let r = radius * (0.35 + t * 0.55 + pulse * 0.08);
        draw_circle_lines(
            center.x,
            center.y,
            r,
            3.0 + (1.0 - t) * 2.5,
            with_alpha(accent, (1.0 - t) * (0.22 + pulse * 0.18)),
        );
    }
    // Sparse beat sparks — bigger, fewer (Ít đồ).
    if pulse > 0.42 {
        for i in 0..4 {
            let a = time * 1.1 + i as f32 * (TAU * 0.25);
            let spark = vec2(
                center.x + a.cos() * radius * 0.62,
                center.y + a.sin() * radius * 0.42,
            );
            draw_soft_glow(spark, radius * 0.08, with_alpha(accent, 0.35), 2);
            draw_circle(spark.x, spark.y, radius * 0.028, with_alpha(WHITE, 0.45));
        }
    }
}

fn draw_character_torso(l_shoulder: Vec2, r_shoulder: Vec2, l_hip: Vec2, r_hip: Vec2, color: Color) {
    let mid_shoulder = (l_shoulder + r_shoulder) * 0.5;
    let mid_hip = (l_hip + r_hip) * 0.5;
    let width = (l_shoulder.distance(r_shoulder) * 0.55)
        .max(l_hip.distance(r_hip) * 0.55)
        .max(8.0);
    draw_filled_limb(mid_shoulder, mid_hip, width, color);
    draw_filled_limb(l_shoulder, r_shoulder, width * 0.55, color);
    draw_filled_limb(l_hip, r_hip, width * 0.5, color);
}

/// Neon lane avatar — filled limbs (not stick bones).
pub fn draw_neon_silhouette(
    pose: PoseFrame,
    project: &dyn Fn(usize) -> Vec2,
    color: Color,
    flash: f32,
    body_height: f32,
) {
    let fill = with_alpha(color, (0.72 + flash * 0.15).min(1.0));
    let limb_t = body_height * (0.085 + flash * 0.02);
    draw_pose_character(pose, project, fill, color, flash, body_height, limb_t, false);
}

/// Pink neon “you” character — ~40% fill + pink aura, no render targets.
pub fn draw_pink_neon_silhouette(
    pose: PoseFrame,
    project: &dyn Fn(usize) -> Vec2,
    flash: f32,
    body_height: f32,
) {
    let fill = with_alpha(USER_SKELETON_PINK, USER_SKELETON_ALPHA);
    let limb_t = body_height * (0.10 + flash * 0.025);
    let hip = if pose.keypoints[11].confidence >= 0.22 && pose.keypoints[12].confidence >= 0.22 {
        (project(11) + project(12)) * 0.5
    } else {
        project(0)
    };
    draw_character_aura(hip, body_height * 0.55, USER_SKELETON_PINK, flash);
    draw_pose_character(
        pose,
        project,
        fill,
        USER_SKELETON_PINK,
        flash,
        body_height,
        limb_t,
        true,
    );
}

fn draw_pose_character(
    pose: PoseFrame,
    project: &dyn Fn(usize) -> Vec2,
    fill: Color,
    aura: Color,
    flash: f32,
    body_height: f32,
    limb_t: f32,
    soft_hands: bool,
) {
    let conf = |i: usize| pose.keypoints[i].confidence >= 0.22;
    if conf(5) && conf(6) && conf(11) && conf(12) {
        draw_character_torso(project(5), project(6), project(11), project(12), fill);
    }
    for chain in SILHOUETTE_CHAINS {
        for window in chain.windows(2) {
            let from = window[0];
            let to = window[1];
            if !conf(from) || !conf(to) {
                continue;
            }
            let thick = if matches!(from, 5 | 6 | 11 | 12) && matches!(to, 5 | 6 | 11 | 12) {
                limb_t * 0.7
            } else {
                limb_t
            };
            draw_filled_limb(project(from), project(to), thick, fill);
        }
    }

    if conf(0) {
        let head = project(0);
        draw_soft_glow(
            head,
            body_height * 0.11 * USER_SKELETON_GLOW_MUL + flash * 4.0,
            with_alpha(aura, USER_SKELETON_ALPHA * 0.9 + flash * 0.15),
            3,
        );
        draw_circle(head.x, head.y, body_height * 0.095 + flash * 2.0, fill);
        draw_circle(
            head.x,
            head.y,
            body_height * 0.038,
            with_alpha(WHITE, USER_SKELETON_ALPHA + 0.2),
        );
    }

    for hand in [9usize, 10] {
        if !conf(hand) {
            continue;
        }
        let wrist = project(hand);
        draw_soft_glow(
            wrist,
            body_height * 0.08 * USER_SKELETON_GLOW_MUL + flash * 3.0,
            with_alpha(aura, USER_SKELETON_ALPHA + flash * 0.15),
            3,
        );
        let hand_a = if soft_hands {
            with_alpha(WHITE, USER_SKELETON_ALPHA + 0.25)
        } else {
            WHITE
        };
        draw_circle(wrist.x, wrist.y, body_height * 0.045, hand_a);
    }
}

pub fn with_alpha(color: Color, alpha: f32) -> Color {
    Color::new(color.r, color.g, color.b, alpha.clamp(0.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn particle_pool_reuses_slots_without_growing() {
        let mut pool = ParticlePool::default();
        for _ in 0..12 {
            pool.spawn_burst(vec2(0.5, 0.5), BurstKind::Hit, 1.0);
        }
        assert!(pool.live_count() <= PARTICLE_CAPACITY);
        assert!(pool.live_count() >= BurstKind::Hit.count());
        for _ in 0..40 {
            pool.update(0.05);
        }
        assert_eq!(pool.live_count(), 0);
    }

    #[test]
    fn screen_shake_decays_and_respects_reduce_motion() {
        let mut shake = ScreenShake::with_decay(20.0);
        shake.impulse(8.0);
        assert!(shake.magnitude() >= 8.0);
        shake.update(0.2);
        assert!(shake.magnitude() < 8.0);
        assert_eq!(shake.offset(1.0, true), Vec2::ZERO);
        assert_ne!(shake.offset(1.0, false), Vec2::ZERO);
    }

    #[test]
    fn spring_scale_returns_toward_target_after_punch() {
        let mut spring = SpringScale::new(1.0);
        spring.punch(6.0);
        for _ in 0..8 {
            spring.update(1.0 / 60.0);
        }
        assert!(spring.value > 1.02);
        for _ in 0..180 {
            spring.update(1.0 / 60.0);
        }
        assert!((spring.value - 1.0).abs() < 0.05);
    }

    #[test]
    fn silhouette_chains_cover_requested_limb_paths() {
        assert_eq!(SILHOUETTE_CHAINS[0], &[9, 7, 5, 11, 13, 15]);
        assert_eq!(SILHOUETTE_CHAINS[1], &[10, 8, 6, 12, 14, 16]);
    }

    #[test]
    fn hero_edge_meets_twenty_percent_min_contract() {
        for (w, h) in [(390.0, 844.0), (667.0, 375.0), (1440.0, 784.0)] {
            let edge = hero_edge(w, h);
            assert!(hero_edge_meets_contract(edge, w, h));
            assert!(edge >= w.min(h) * HERO_EDGE_MIN_FRACTION);
            assert!((edge - w.min(h) * HERO_EDGE_SHIP_FRACTION).abs() < 0.01);
        }
    }

    #[test]
    fn pink_user_character_opacity_is_forty_percent() {
        assert!((USER_SKELETON_ALPHA - 0.40).abs() < 0.001);
        assert!(USER_SKELETON_PINK.r > 0.9);
        assert!(USER_SKELETON_PINK.g < 0.4);
        assert!(USER_SKELETON_PINK.b > 0.7);
        assert!(USER_SKELETON_GLOW_MUL > 1.0);
    }
}
