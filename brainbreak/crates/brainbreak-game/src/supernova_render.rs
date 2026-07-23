//! Supernova Freeze Party renderer — show-don't-tell kid visuals.
//! Pictograms, color signals (green move / red freeze), elemental VFX,
//! drum pulse rings. No on-canvas word walls — kids at 1.5–4m cannot read.

use brainbreak_core::{PoseFrame, SupernovaGame, SupernovaPhase};
use macroquad::prelude::*;

use crate::guide_coach::{
    draw_guide_coach, guide_coach_layout, user_overlay_bounds, GuidePose, GUIDE_DANCE_VERB_SECONDS,
};
use crate::juice::{
    draw_pink_neon_silhouette, draw_soft_glow, hero_edge, BurstKind, ParticlePool, ScreenShake,
    SpringScale,
};
use crate::visuals::AudioVisual;

const TAU: f32 = std::f32::consts::TAU;
const MAX_CONFETTI: usize = 72;

fn rand01() -> f32 {
    macroquad::rand::gen_range(0.0, 1.0)
}

/// Bouncy overshoot easing (cartoon squash-and-stretch).
fn bounce_scale(t: f32) -> f32 {
    let x = t.clamp(0.0, 1.0);
    1.0 + (x * TAU * 1.5).sin() * (1.0 - x) * 0.35
}

/// Bright kid-friendly palette (Danny Go primary neon).
const KID_COLORS: [Color; 6] = [
    Color::from_rgba(255, 89, 94, 255),   // red-coral
    Color::from_rgba(255, 202, 58, 255),  // sunny yellow
    Color::from_rgba(138, 201, 38, 255),  // lime green
    Color::from_rgba(25, 130, 196, 255),  // sky blue
    Color::from_rgba(148, 103, 189, 255), // grape purple
    Color::from_rgba(255, 140, 66, 255),  // orange
];

const ICE_BLUE: Color = Color::from_rgba(140, 220, 255, 255);

#[derive(Clone, Copy)]
struct Confetti {
    pos: Vec2,
    vel: Vec2,
    rot: f32,
    rot_speed: f32,
    life: f32,
    max_life: f32,
    w: f32,
    h: f32,
    color: Color,
    is_star: bool,
}

pub struct SupernovaStage {
    confetti: Vec<Confetti>,
    glow: ParticlePool,
    shake: ScreenShake,
    core_spring: SpringScale,
    cue_spring: SpringScale,
    result_spring: SpringScale,
    explosion_spawned: bool,
    flash_alpha: f32,
    time: f32,
    countdown_pop: f32,
    last_countdown_num: u8,
    freeze_pop: f32,
    dance_pop: f32,
    last_phase: SupernovaPhase,
    /// Hit-stop timer: briefly freezes stage animation for physical weight.
    hitstop: f32,
}

impl Default for SupernovaStage {
    fn default() -> Self {
        Self {
            confetti: Vec::new(),
            glow: ParticlePool::default(),
            shake: ScreenShake::with_decay(26.0),
            core_spring: SpringScale::new(1.0),
            cue_spring: SpringScale::new(1.0),
            result_spring: SpringScale::new(1.0),
            explosion_spawned: false,
            flash_alpha: 0.0,
            time: 0.0,
            countdown_pop: 0.0,
            last_countdown_num: 0,
            freeze_pop: 0.0,
            dance_pop: 0.0,
            last_phase: SupernovaPhase::Ready,
            hitstop: 0.0,
        }
    }
}

impl SupernovaStage {
    pub fn update(&mut self, dt: f32, game: &SupernovaGame, _audio: AudioVisual, reduce_motion: bool) {
        // Hit-stop: a beat of stillness on impact makes the drop feel heavy.
        if self.hitstop > 0.0 {
            self.hitstop = (self.hitstop - dt).max(0.0);
            return;
        }
        self.time += dt;
        self.glow.update(dt);
        self.shake.update(dt);
        self.core_spring.update(dt);
        self.cue_spring.update(dt);
        self.result_spring.update(dt);

        // Phase-change pop animations.
        if game.phase != self.last_phase {
            if game.phase == SupernovaPhase::Freeze {
                self.freeze_pop = 1.0;
                self.cue_spring.punch(5.0);
                if !reduce_motion {
                    self.glow.spawn_screen_burst(
                        vec2(screen_width() * 0.5, screen_height() * 0.42),
                        BurstKind::Freeze,
                        1.1,
                    );
                    self.shake.impulse(2.8);
                }
            }
            if game.phase == SupernovaPhase::Dance {
                self.dance_pop = 1.0;
                self.cue_spring.punch(4.2);
            }
            if game.phase == SupernovaPhase::Result {
                self.result_spring.punch(6.5);
            }
            self.last_phase = game.phase;
        }
        self.freeze_pop = (self.freeze_pop - dt * 2.5).max(0.0);
        self.dance_pop = (self.dance_pop - dt * 2.5).max(0.0);

        // Countdown pop animation.
        let num = game.countdown_remaining.ceil() as u8;
        if num != self.last_countdown_num {
            self.countdown_pop = 1.0;
            self.last_countdown_num = num;
            self.cue_spring.punch(3.5);
        }
        self.countdown_pop = (self.countdown_pop - dt * 3.0).max(0.0);

        // On-beat dance hits / drum smash juice.
        for feedback in game.feedback.iter() {
            if !feedback.hit {
                continue;
            }
            let kind = if feedback.on_beat {
                BurstKind::Clap
            } else {
                BurstKind::Hit
            };
            self.glow.spawn_screen_burst(
                vec2(screen_width() * 0.5, screen_height() * 0.42),
                kind,
                if feedback.on_beat { 1.15 } else { 0.9 },
            );
            self.core_spring.punch(if feedback.on_beat { 4.8 } else { 2.6 });
            if !reduce_motion {
                self.shake.impulse(if feedback.on_beat { 3.6 } else { 1.8 });
            }
        }

        // Spawn confetti explosion on drop + impact shake & hit-stop.
        if game.drop_triggered && !self.explosion_spawned {
            self.spawn_celebration(game.energy);
            self.glow.spawn_screen_burst(
                vec2(screen_width() * 0.5, screen_height() * 0.4),
                BurstKind::Drop,
                1.45,
            );
            self.explosion_spawned = true;
            self.flash_alpha = 1.0;
            self.core_spring.punch(8.0);
            if !reduce_motion {
                self.shake.impulse(14.0);
                self.hitstop = 0.08;
            }
        }

        // Near-full core: continuous micro-tremble before the Drop.
        if game.drop_imminent && !reduce_motion {
            let tremble = 1.8 + (game.energy - 0.82).max(0.0) * 18.0;
            self.shake.impulse(tremble);
        }

        // Perfect freeze: small satisfied shake (one-shot frame flag).
        if game.perfect_freeze {
            self.core_spring.punch(5.5);
            self.glow.spawn_screen_burst(
                vec2(screen_width() * 0.5, screen_height() * 0.42),
                BurstKind::Freeze,
                1.25,
            );
            if !reduce_motion {
                self.shake.impulse(3.5);
            }
        }

        // Reset when returning to countdown.
        if game.phase == SupernovaPhase::Countdown {
            self.explosion_spawned = false;
            self.confetti.clear();
            self.glow.clear();
        }

        // Decay flash (slower flash linger sells the Drop).
        let flash_decay = if game.phase == SupernovaPhase::Drop { 1.15 } else { 2.0 };
        self.flash_alpha = (self.flash_alpha - dt * flash_decay).max(0.0);

        // Update confetti physics.
        for c in self.confetti.iter_mut() {
            c.pos += c.vel * dt;
            c.vel.y += 200.0 * dt;
            c.vel.x *= 0.99;
            c.rot += c.rot_speed * dt;
            c.life -= dt;
        }
        self.confetti.retain(|c| c.life > 0.0);
    }

    fn spawn_celebration(&mut self, energy: f32) {
        let cx = screen_width() * 0.5;
        let cy = screen_height() * 0.4;
        let count = (MAX_CONFETTI as f32 * (0.5 + energy * 0.5)) as usize;
        for i in 0..count {
            let angle = (i as f32 / count as f32) * TAU + rand01() * 0.5;
            let speed = 180.0 + rand01() * 400.0;
            let life = 1.2 + rand01() * 2.0;
            self.confetti.push(Confetti {
                pos: vec2(cx, cy),
                vel: vec2(angle.cos() * speed, angle.sin() * speed - 150.0),
                rot: rand01() * TAU,
                rot_speed: (rand01() - 0.5) * 12.0,
                life,
                max_life: life,
                w: 8.0 + rand01() * 12.0,
                h: 5.0 + rand01() * 8.0,
                color: KID_COLORS[i % KID_COLORS.len()],
                is_star: i % 4 == 0,
            });
        }
    }

    pub fn draw(
        &self,
        game: &SupernovaGame,
        audio: AudioVisual,
        reduce_motion: bool,
        poses: &[PoseFrame],
    ) {
        let w = screen_width();
        let h = screen_height();

        // Beat-synced neon backdrop fills the screen (not shaken, so no edge gaps).
        self.draw_background(game, w, h, audio, reduce_motion);

        // Foreground impact shake offset.
        let shake = self.shake.offset(self.time, reduce_motion);
        let stage = guide_coach_layout(w, h);
        let stage_cx = stage.figure_center.x + shake.x;
        let stage_cy = stage.figure_center.y + shake.y;

        // Guide under user on the shared stage (both ~40%).
        if game.phase != SupernovaPhase::Result {
            draw_guide_coach(
                self.guide_pose_for(game),
                self.time,
                audio.pulse,
                reduce_motion,
            );
        }

        match game.phase {
            SupernovaPhase::Ready => {
                self.draw_pose_ribbons(poses, stage_cx, stage_cy, false);
                self.draw_mascot(
                    game,
                    stage_cx,
                    stage_cy - stage.figure_scale * 0.2,
                    0.0,
                    false,
                    true,
                    audio,
                );
            }
            SupernovaPhase::Countdown => {
                self.draw_pose_ribbons(poses, stage_cx, stage_cy, false);
                self.draw_mascot(
                    game,
                    stage_cx,
                    stage_cy - stage.figure_scale * 0.2,
                    0.0,
                    false,
                    true,
                    audio,
                );
                self.draw_countdown(game, w * 0.5, h * 0.14);
            }
            SupernovaPhase::Dance => {
                self.draw_pose_ribbons(poses, stage_cx, stage_cy, false);
                self.draw_mascot(
                    game,
                    stage_cx,
                    stage_cy - stage.figure_scale * 0.2,
                    game.energy,
                    false,
                    true,
                    audio,
                );
                if audio.pulse > 0.35 {
                    self.draw_drum_pulse_rings(stage_cx, stage_cy + hero_edge(w, h) * 0.2, audio);
                }
            }
            SupernovaPhase::LavaWarning => {
                self.draw_pose_ribbons(poses, stage_cx, stage_cy, false);
                self.draw_mascot(
                    game,
                    stage_cx,
                    stage_cy - stage.figure_scale * 0.2,
                    game.energy,
                    false,
                    true,
                    audio,
                );
                self.draw_lava_warning_cue(w * 0.5, h * 0.12);
                self.draw_countdown(game, w * 0.5, h * 0.18);
            }
            SupernovaPhase::Freeze => {
                let wobble = if game.freeze_wobble {
                    (self.time * 50.0).sin() * 5.0
                } else {
                    0.0
                };
                let cx = stage_cx + wobble;
                self.draw_pose_ribbons(poses, cx, stage_cy, true);
                self.draw_mascot(
                    game,
                    cx,
                    stage_cy - stage.figure_scale * 0.2,
                    game.energy,
                    true,
                    true,
                    audio,
                );
                self.draw_freeze_meter(game, cx, stage_cy);
                if game.perfect_freeze {
                    self.draw_perfect_star(w * 0.5, h * 0.22);
                }
            }
            SupernovaPhase::Drop => {
                self.draw_confetti(shake);
                self.draw_flash();
                self.draw_mascot(game, stage_cx, stage_cy, 1.0, false, true, audio);
            }
            SupernovaPhase::Result => {
                self.draw_confetti(Vec2::ZERO);
                self.draw_result(game, w, h, audio);
            }
        }

        self.glow.draw_screen(reduce_motion);
    }

    fn guide_pose_for(&self, game: &SupernovaGame) -> GuidePose {
        match game.phase {
            SupernovaPhase::Ready => GuidePose::RaiseHands,
            SupernovaPhase::Countdown => GuidePose::Go,
            SupernovaPhase::Dance if game.drop_imminent => GuidePose::Squat,
            SupernovaPhase::Dance => {
                let verb_i = ((self.time / GUIDE_DANCE_VERB_SECONDS).floor() as usize) % 8;
                GuidePose::from_dance_verb(verb_i)
            }
            SupernovaPhase::LavaWarning => GuidePose::from_voice_cue("lava"),
            SupernovaPhase::Freeze => GuidePose::from_voice_cue("freeze"),
            SupernovaPhase::Drop => GuidePose::from_voice_cue("drop"),
            SupernovaPhase::Result => GuidePose::Clap,
        }
    }

    fn draw_pose_ribbons(&self, poses: &[PoseFrame], cx: f32, cy: f32, frozen: bool) {
        let overlay = user_overlay_bounds(screen_width(), screen_height());
        let stage = guide_coach_layout(screen_width(), screen_height());
        // Pink neon “you” stacks on the same stage as the coach (~40% + aura).
        let flash = if frozen { 0.55 } else { 0.4 };
        for (index, pose) in poses.iter().take(2).enumerate() {
            let offset_x = if poses.len() > 1 {
                if index == 0 {
                    -overlay.w * 0.14
                } else {
                    overlay.w * 0.14
                }
            } else {
                0.0
            };
            let hip = average_pose_point(*pose, 11, 12);
            let shoulder = average_pose_point(*pose, 5, 6);
            let source_height = (hip.y - shoulder.y).abs().max(0.12) * 2.4;
            let body_height = stage.figure_scale * 2.35 + self.core_spring.value * 8.0;
            let scale = body_height / source_height;
            // Hip-match the guide figure on the shared mid-playfield stage.
            let anchor = vec2(cx + offset_x, cy + stage.figure_scale * 0.16);
            let project = |key: usize| {
                let point = pose.keypoints[key];
                vec2(
                    anchor.x + (point.x - hip.x) * scale,
                    anchor.y + (point.y - hip.y) * scale,
                )
            };
            draw_pink_neon_silhouette(*pose, &project, flash, body_height);
            if frozen {
                for chain_idx in [5usize, 6, 11, 12] {
                    if pose.keypoints[chain_idx].confidence < 0.22 {
                        continue;
                    }
                    let p = project(chain_idx);
                    draw_soft_glow(p, body_height * 0.08, Color::from_rgba(140, 220, 255, 90), 2);
                }
            }
        }
    }

    // --- Background: elemental party journey skins (wind/ocean → smoke/fire → ice → celebrate) ---
    fn draw_background(&self, game: &SupernovaGame, w: f32, h: f32, audio: AudioVisual, reduce_motion: bool) {
        let energy = game.energy;
        let phase = game.phase;
        let frozen = phase == SupernovaPhase::Freeze;
        let lava_warn = phase == SupernovaPhase::LavaWarning;
        let dropping = phase == SupernovaPhase::Drop;
        let cooling = phase == SupernovaPhase::Result;
        let opening = phase == SupernovaPhase::Countdown || phase == SupernovaPhase::Ready;
        let drive = if reduce_motion {
            audio.energy * 0.25
        } else {
            audio.pulse * 0.55 + audio.energy * 0.5
        };

        // Elemental sky: ice / fire+smoke / ocean cool / wind+ocean dance / smoke open
        let (top, bot) = if frozen {
            (
                Color::from_rgba(16, 42, 84, 255),
                Color::from_rgba(8, 24, 52, 255),
            )
        } else if lava_warn || dropping {
            (
                Color::from_rgba(78, 22, 10, 255),
                Color::from_rgba(36, 8, 6, 255),
            )
        } else if cooling {
            (
                Color::from_rgba(12, 48, 72, 255),
                Color::from_rgba(8, 28, 48, 255),
            )
        } else if opening {
            (
                Color::from_rgba(36, 32, 52, 255),
                Color::from_rgba(18, 16, 28, 255),
            )
        } else {
            // Dance: teal wind / ocean sway
            (
                Color::from_rgba(
                    (18.0 + energy * 28.0) as u8,
                    (42.0 + energy * 40.0) as u8,
                    (72.0 + energy * 50.0) as u8,
                    255,
                ),
                Color::from_rgba(
                    (10.0 + energy * 20.0) as u8,
                    (28.0 + energy * 36.0) as u8,
                    (58.0 + energy * 40.0) as u8,
                    255,
                ),
            )
        };
        for row in 0..14 {
            let t = row as f32 / 13.0;
            draw_rectangle(0.0, h * t, w, h / 13.0 + 1.0, lerp_color(top, bot, t));
        }

        // Soft smoke wisps on open / lava warning — sparse, not fog soup.
        if opening || lava_warn {
            for i in 0..3 {
                let fi = i as f32;
                let sx = (w * (0.2 + fi * 0.25) + self.time * (8.0 + fi)).rem_euclid(w);
                let sy = h * (0.18 + ((self.time * 0.15 + fi * 0.2).sin() * 0.06 + 0.06));
                let a = if lava_warn { 45 } else { 28 };
                draw_circle(sx, sy, 36.0 + fi * 8.0, Color::from_rgba(200, 200, 210, a));
            }
        }

        // A few big wind streaks during dance (readable motion, not glitter).
        if phase == SupernovaPhase::Dance && !reduce_motion {
            for i in 0..4 {
                let fi = i as f32;
                let y = h * (0.22 + fi * 0.12);
                let x = (self.time * (56.0 + fi * 18.0) + fi * 90.0).rem_euclid(w + 140.0) - 70.0;
                draw_rectangle(x, y, 90.0 + fi * 12.0, 4.0, Color::from_rgba(180, 240, 255, 75));
            }
        }

        // Ocean wave bands on dance + result cool-down
        if phase == SupernovaPhase::Dance || cooling {
            for i in 0..2 {
                let fi = i as f32;
                let y = h * 0.78 + (self.time * 1.2 + fi).sin() * 6.0 + fi * 14.0;
                draw_rectangle(
                    0.0,
                    y,
                    w,
                    10.0,
                    Color::from_rgba(60, 160, 210, if cooling { 70 } else { 40 }),
                );
            }
        }

        let star_alpha = 0.14 + audio.energy * 0.28;
        for star in 0..10 {
            let x = ((star * 71) % 997) as f32 / 997.0 * w;
            let y = ((star * 43 + 19) % 251) as f32 / 251.0 * h * 0.28;
            let twinkle = ((self.time * 2.0 + star as f32).sin() * 0.5 + 0.5) * star_alpha;
            let c = if frozen || cooling {
                Color::from_rgba(190, 230, 255, (twinkle * 255.0) as u8)
            } else if lava_warn || dropping {
                Color::from_rgba(255, 180, 120, (twinkle * 255.0) as u8)
            } else {
                Color::from_rgba(220, 255, 250, (twinkle * 255.0) as u8)
            };
            draw_circle(x, y, 2.0 + (star % 3) as f32, c);
        }

        self.draw_neon_sun(w * 0.5, h * 0.42, w.min(h), drive, frozen || cooling, energy);
        self.draw_horizon_grid(w, h, drive, frozen, reduce_motion);
        // Equalizer culled — competed with hero cues at camera distance.

        if frozen {
            for i in 0..6 {
                let fi = i as f32;
                let sx = (w * (0.12 + fi * 0.14) + self.time * (10.0 + fi * 3.0)).rem_euclid(w);
                let sy = (self.time * (30.0 + fi * 8.0) + fi * 90.0).rem_euclid(h);
                draw_snowflake(sx, sy, 10.0 + (fi % 3.0) * 3.0, Color::from_rgba(200, 235, 255, 150));
            }
        }

        // Fire ember sparks on lava warning / drop
        if (lava_warn || dropping) && !reduce_motion {
            for i in 0..18 {
                let fi = i as f32;
                let sx = (w * (0.03 + fi * 0.055) + (self.time * 20.0 + fi * 17.0).sin() * 12.0)
                    .rem_euclid(w);
                let sy = h - (self.time * (50.0 + fi * 9.0) + fi * 40.0).rem_euclid(h * 0.65);
                draw_circle(sx, sy, 3.0 + (fi % 3.0), Color::from_rgba(255, 140, 60, 180));
            }
        }

        // Virtual lava floor band — reads at distance without words
        if lava_warn || frozen || dropping {
            let lava_h = h * if frozen { 0.18 } else { 0.28 };
            let pulse = if reduce_motion {
                0.35
            } else {
                0.45 + (self.time * 4.0).sin().abs() * 0.35
            };
            draw_rectangle(
                0.0,
                h - lava_h,
                w,
                lava_h,
                Color::from_rgba(
                    255,
                    if frozen { 60 } else { 90 },
                    if frozen { 40 } else { 20 },
                    (90.0 + pulse * 80.0) as u8,
                ),
            );
            if !frozen {
                for i in 0..7 {
                    let fi = i as f32;
                    let x = (self.time * (30.0 + fi * 8.0) + fi * 70.0).rem_euclid(w + 40.0) - 20.0;
                    let y = h - lava_h * (0.35 + (self.time * 2.0 + fi).sin().abs() * 0.4);
                    draw_circle(x, y, 10.0 + fi * 2.0, Color::from_rgba(255, 200, 60, 140));
                }
            } else {
                // Ice crust over lava — freeze readable as color flip
                draw_rectangle(
                    0.0,
                    h - lava_h * 0.55,
                    w,
                    lava_h * 0.55,
                    Color::from_rgba(140, 220, 255, 70),
                );
            }
        }
    }

    // --- Neon sun: retro striped disc that breathes on every beat ---
    fn draw_neon_sun(&self, cx: f32, cy: f32, min_dim: f32, drive: f32, frozen: bool, energy: f32) {
        let base_r = min_dim * (0.16 + energy * 0.05);
        let r = base_r * (1.0 + drive * 0.06);
        let glow = if frozen {
            Color::from_rgba(120, 210, 255, 255)
        } else {
            Color::from_rgba(255, 92, 158, 255)
        };
        // Outer glow halos (breathe with the beat).
        draw_circle(cx, cy, r * 1.35, Color::new(glow.r, glow.g, glow.b, 0.06 + drive * 0.05));
        draw_circle(cx, cy, r * 1.16, Color::new(glow.r, glow.g, glow.b, 0.10 + drive * 0.06));
        // Core disc.
        draw_circle(cx, cy, r, Color::new(glow.r, glow.g, glow.b, 0.92));
        // Retro horizontal stripes cutting the sun.
        let stripe_color = Color::from_rgba(20, 10, 40, 160);
        for stripe in 0..6 {
            let y = cy - r + (stripe as f32 + 0.5) * r * 0.34;
            let dy = y - cy;
            let half = (r * r - dy * dy).max(0.0).sqrt();
            draw_rectangle(cx - half, y, half * 2.0, 3.0 + stripe as f32 * 1.1, stripe_color);
        }
    }

    // --- Horizon grid: perspective floor rushing toward the player on the beat ---
    fn draw_horizon_grid(&self, w: f32, h: f32, drive: f32, frozen: bool, reduce_motion: bool) {
        let horizon_y = h * 0.62;
        let near_y = h * 1.02;
        let cx = w * 0.5;
        let horizon_half = w * 0.06;
        let near_half = w * 0.75;
        let rail = if frozen {
            Color::from_rgba(110, 200, 255, 255)
        } else {
            Color::from_rgba(255, 60, 190, 255)
        };
        let rung = if frozen {
            Color::from_rgba(160, 230, 255, 255)
        } else {
            Color::from_rgba(40, 220, 255, 255)
        };

        // Dark floor grounds the scene.
        draw_rectangle(0.0, horizon_y, w, near_y - horizon_y, Color::from_rgba(8, 6, 24, 200));

        // Converging side rails.
        for side in [-1.0_f32, 1.0] {
            draw_line(
                cx + side * horizon_half, horizon_y,
                cx + side * near_half, near_y,
                2.5,
                Color::new(rail.r, rail.g, rail.b, 0.5 + drive * 0.3),
            );
        }
        // Horizontal rungs scroll toward the viewer (speed is beat-locked).
        let scroll = if reduce_motion { 0.0 } else { (self.time * 0.55) % 1.0 };
        for row in 0..10 {
            let t = ((row as f32 / 10.0) + scroll).fract();
            let eased = t * t; // accelerate toward the viewer
            let y = horizon_y + (near_y - horizon_y) * eased;
            let half = horizon_half + (near_half - horizon_half) * eased;
            let alpha = (0.10 + eased * 0.4) * (0.6 + drive * 0.4);
            draw_line(cx - half, y, cx + half, y, 1.0 + eased * 2.0, Color::new(rung.r, rung.g, rung.b, alpha));
        }
        // Glowing horizon line.
        draw_line(0.0, horizon_y, w, horizon_y, 2.0, Color::new(rail.r, rail.g, rail.b, 0.65 + drive * 0.3));
    }

    // --- Equalizer pylons: side bars that dance to the loudness ---
    // Equalizer / multi-icon rows culled — one hero cue owns attention.
    #[allow(dead_code)]
    fn draw_equalizer(&self, w: f32, h: f32, audio: AudioVisual, frozen: bool, reduce_motion: bool) {
        let base_y = h * 0.98;
        let bars = 5;
        let max_h = h * 0.16;
        for side in 0..2 {
            for i in 0..bars {
                let fi = i as f32;
                let phase_off = fi * 0.9 + side as f32 * 1.7;
                let dance = if reduce_motion {
                    0.5
                } else {
                    (self.time * 4.0 + phase_off).sin() * 0.5 + 0.5
                };
                let amp = 0.25 + audio.energy * 0.75;
                let bh = max_h * (0.2 + dance * amp);
                let x = if side == 0 {
                    w * 0.04 + fi * w * 0.035
                } else {
                    w * 0.96 - fi * w * 0.035
                };
                let bw = w * 0.018;
                let c = if frozen { ICE_BLUE } else { KID_COLORS[(i + side * 2) % KID_COLORS.len()] };
                draw_rectangle(x - bw * 0.5, base_y - bh, bw, bh, Color::new(c.r, c.g, c.b, 0.5 + audio.pulse * 0.3));
            }
        }
    }

    // --- Mascot: cute round blob with face; frozen = ice-blue & stiff ---
    #[allow(clippy::too_many_arguments)]
    fn draw_mascot(&self, game: &SupernovaGame, cx: f32, cy: f32, energy: f32, frozen: bool, show_face: bool, audio: AudioVisual) {
        let screen_min = screen_width().min(screen_height());
        let base_r = hero_edge(screen_width(), screen_height()) * 0.42;
        let radius = base_r + energy * screen_min * 0.06;

        // Bounce to the music: idle sway + a kick on every beat pulse.
        let beat_kick = if frozen { 0.0 } else { audio.pulse * 7.0 };
        let idle_bounce = if frozen { 0.0 } else { (self.time * 3.0).sin() * 4.0 };
        let squash = if frozen {
            1.0
        } else {
            self.core_spring.value * (1.0 + (self.time * 3.0).sin() * 0.03 + audio.pulse * 0.05)
        };
        let pos = vec2(cx, cy + idle_bounce - beat_kick);

        // Body color: frozen = ice blue; else blue → orange → gold with energy.
        let body_color = if frozen {
            ICE_BLUE
        } else if energy < 0.5 {
            lerp_color(KID_COLORS[3], KID_COLORS[5], energy * 2.0)
        } else {
            lerp_color(KID_COLORS[5], KID_COLORS[1], (energy - 0.5) * 2.0)
        };

        // AR aura: concentric glow breathing with the beat while dancing.
        if !frozen && (game.phase == SupernovaPhase::Dance || game.phase == SupernovaPhase::Drop) {
            let aura_color = if energy > 0.6 { KID_COLORS[1] } else { KID_COLORS[3] };
            draw_soft_glow(
                pos,
                radius * 1.35 + audio.pulse * 10.0,
                Color::new(aura_color.r, aura_color.g, aura_color.b, 0.28 + audio.pulse * 0.2),
                3,
            );
        }

        // Shadow.
        draw_circle(pos.x, pos.y + radius * 0.85, radius * 0.5, Color::from_rgba(0, 0, 0, 40));

        // Body.
        draw_circle(pos.x, pos.y, radius * squash, body_color);

        // Belly highlight.
        draw_circle(
            pos.x, pos.y + radius * 0.15,
            radius * 0.55 * squash,
            Color::from_rgba(255, 255, 255, 60),
        );

        // Ice crystals on top when frozen.
        if frozen {
            for i in 0..3 {
                let angle = -std::f32::consts::FRAC_PI_2 + (i as f32 - 1.0) * 0.5;
                let ix = pos.x + angle.cos() * radius;
                let iy = pos.y + angle.sin() * radius;
                draw_snowflake(ix, iy, 8.0, WHITE);
            }
        }

        if show_face {
            self.draw_face(game, pos, radius * squash, frozen);
        }

        // Arms: waving during dance / lava warning, held stiff down during freeze.
        if matches!(
            game.phase,
            SupernovaPhase::Dance | SupernovaPhase::LavaWarning | SupernovaPhase::Freeze
        ) {
            let arm_r = radius * 0.18;
            if frozen {
                // Stiff arms at sides.
                draw_circle(pos.x - radius * 0.85, pos.y + radius * 0.3, arm_r, body_color);
                draw_circle(pos.x + radius * 0.85, pos.y + radius * 0.3, arm_r, body_color);
            } else {
                // Waving arms.
                draw_circle(
                    pos.x - radius * 0.8,
                    pos.y - radius * 0.2 + (self.time * 5.0).sin() * 6.0,
                    arm_r, body_color,
                );
                draw_circle(
                    pos.x + radius * 0.8,
                    pos.y - radius * 0.2 + (self.time * 5.0 + 1.0).sin() * 6.0,
                    arm_r, body_color,
                );
            }
        }
    }

    fn draw_face(&self, game: &SupernovaGame, pos: Vec2, radius: f32, frozen: bool) {
        let eye_offset = radius * 0.3;
        let eye_r = radius * 0.14;
        let eye_y = pos.y - radius * 0.15;

        if frozen {
            // Closed happy eyes (curved lines) — frozen content.
            draw_circle_lines(pos.x - eye_offset, eye_y, eye_r * 0.8, 2.0, Color::from_rgba(30, 30, 50, 255));
            draw_circle_lines(pos.x + eye_offset, eye_y, eye_r * 0.8, 2.0, Color::from_rgba(30, 30, 50, 255));
        } else {
            // Big round eyes.
            draw_circle(pos.x - eye_offset, eye_y, eye_r, WHITE);
            draw_circle(pos.x + eye_offset, eye_y, eye_r, WHITE);
            let pupil_r = eye_r * 0.55;
            draw_circle(pos.x - eye_offset, eye_y + pupil_r * 0.3, pupil_r, Color::from_rgba(30, 30, 50, 255));
            draw_circle(pos.x + eye_offset, eye_y + pupil_r * 0.3, pupil_r, Color::from_rgba(30, 30, 50, 255));
            draw_circle(pos.x - eye_offset - pupil_r * 0.3, eye_y - pupil_r * 0.3, pupil_r * 0.35, WHITE);
            draw_circle(pos.x + eye_offset - pupil_r * 0.3, eye_y - pupil_r * 0.3, pupil_r * 0.35, WHITE);
        }

        // Mouth.
        let mouth_y = pos.y + radius * 0.25;
        match game.phase {
            SupernovaPhase::Drop | SupernovaPhase::Result => {
                draw_circle(pos.x, mouth_y, radius * 0.2, Color::from_rgba(60, 20, 20, 255));
                draw_circle(pos.x, mouth_y - radius * 0.04, radius * 0.12, Color::from_rgba(255, 120, 120, 255));
            }
            SupernovaPhase::Freeze => {
                // Tiny "o" mouth — holding breath, staying still.
                draw_circle(pos.x, mouth_y, radius * 0.08, Color::from_rgba(60, 20, 20, 255));
            }
            _ => {
                let smile_r = radius * 0.22;
                draw_circle_lines(pos.x, mouth_y - smile_r * 0.4, smile_r, 2.5, Color::from_rgba(60, 20, 20, 200));
            }
        }

        // Rosy cheeks (icy pink when frozen).
        let cheek = if frozen {
            Color::from_rgba(180, 220, 255, 90)
        } else {
            Color::from_rgba(255, 150, 150, 80)
        };
        draw_circle(pos.x - eye_offset * 1.4, mouth_y - radius * 0.05, radius * 0.08, cheek);
        draw_circle(pos.x + eye_offset * 1.4, mouth_y - radius * 0.05, radius * 0.08, cheek);
    }

    // --- Energy ring: circular progress around mascot (no text) ---
    #[allow(dead_code)]
    fn draw_energy_ring(&self, game: &SupernovaGame, cx: f32, cy: f32, audio: AudioVisual) {
        let radius = 105.0 + game.energy * 45.0;
        let segments = 24;
        let filled = (segments as f32 * game.energy) as usize;

        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * TAU - std::f32::consts::FRAC_PI_2;
            let next_angle = ((i + 1) as f32 / segments as f32) * TAU - std::f32::consts::FRAC_PI_2;
            let mid_angle = (angle + next_angle) * 0.5;
            let dx = cx + mid_angle.cos() * radius;
            let dy = cy + mid_angle.sin() * radius;

            if i < filled {
                // Filled segments swell on every beat.
                draw_circle(dx, dy, 5.0 + audio.pulse * 2.0, KID_COLORS[i % KID_COLORS.len()]);
            } else {
                draw_circle(dx, dy, 3.0, Color::from_rgba(255, 255, 255, 35));
            }
        }

        // Beat pulse ring driven by the real audio transient.
        let beat_pulse = audio.pulse;
        if beat_pulse > 0.1 {
            draw_circle_lines(
                cx, cy,
                radius + 12.0 + beat_pulse * 10.0,
                2.0 + beat_pulse * 2.5,
                Color::from_rgba(255, 255, 255, (beat_pulse * 110.0) as u8),
            );
        }
    }

    // --- Traffic-light color signals (flamingo / red-green format) ---
    #[allow(dead_code)]
    fn draw_signal_light(&self, cx: f32, cy: f32, kind: SignalKind, pulse: f32) {
        let scale = self.cue_spring.value
            * (1.0 + pulse * 0.12 + bounce_scale(self.dance_pop.max(self.freeze_pop)) * 0.08);
        let (fill, ring) = match kind {
            SignalKind::Go => (
                Color::from_rgba(80, 220, 90, 230),
                Color::from_rgba(180, 255, 180, 180),
            ),
            SignalKind::Warn => (
                Color::from_rgba(255, 180, 40, 240),
                Color::from_rgba(255, 220, 120, 200),
            ),
            SignalKind::Stop => (
                Color::from_rgba(255, 70, 70, 240),
                Color::from_rgba(255, 160, 160, 200),
            ),
        };
        let r = hero_edge(screen_width(), screen_height()) * 0.5 * scale;
        draw_circle(cx, cy, r * 1.35, Color::new(fill.r, fill.g, fill.b, 0.18));
        draw_circle(cx, cy, r, fill);
        draw_circle_lines(cx, cy, r * 1.15, 4.0, ring);
        // Inner glyph: play triangle / bang / lock bar
        match kind {
            SignalKind::Go => {
                draw_triangle(
                    vec2(cx - r * 0.25, cy - r * 0.35),
                    vec2(cx - r * 0.25, cy + r * 0.35),
                    vec2(cx + r * 0.4, cy),
                    WHITE,
                );
            }
            SignalKind::Warn => {
                draw_triangle(
                    vec2(cx, cy - r * 0.45),
                    vec2(cx - r * 0.38, cy + r * 0.32),
                    vec2(cx + r * 0.38, cy + r * 0.32),
                    Color::from_rgba(40, 20, 0, 255),
                );
                draw_circle(cx, cy + r * 0.12, 3.5, Color::from_rgba(40, 20, 0, 255));
            }
            SignalKind::Stop => {
                draw_rectangle(cx - r * 0.35, cy - r * 0.12, r * 0.7, r * 0.24, WHITE);
            }
        }
    }

    fn draw_drum_pulse_rings(&self, cx: f32, cy: f32, audio: AudioVisual) {
        let hit = audio.pulse;
        let base = hero_edge(screen_width(), screen_height()) * 0.22;
        for i in 0..2 {
            let fi = i as f32;
            let r = base + fi * base * 0.55 + hit * base * 0.35;
            let a = ((0.55 - fi * 0.18) * (0.35 + hit) * 255.0) as u8;
            draw_circle_lines(cx, cy, r, 4.0 + hit * 3.0, Color::from_rgba(255, 202, 58, a));
        }
    }

    /// Rotating gesture pictogram during dance — no English verbs.
    #[allow(dead_code)] // Bottom coach owns the verb; keep for sparse hero accents.
    fn draw_dance_cue(&self, cx: f32, cy: f32, audio: AudioVisual) {
        let verb_i = ((self.time / 3.2).floor() as usize) % 8;
        let scale = hero_edge(screen_width(), screen_height())
            * 0.5
            * bounce_scale(self.dance_pop)
            * (1.0 + audio.pulse * 0.08)
            * self.cue_spring.value;
        let color = KID_COLORS[verb_i % KID_COLORS.len()];
        draw_soft_glow(vec2(cx, cy), scale * 1.35, Color::new(color.r, color.g, color.b, 0.28), 3);
        draw_gesture_pictogram(GestureCue::from_index(verb_i), vec2(cx, cy), scale, color);
    }

    /// Pull-to-drop: big down squat silhouette + fire ring (no DROP! text).
    #[allow(dead_code)]
    fn draw_drop_cue(&self, cx: f32, cy: f32, audio: AudioVisual) {
        let pulse = 1.0 + audio.pulse * 0.12 + (self.time * 14.0).sin().abs() * 0.06;
        let scale = hero_edge(screen_width(), screen_height()) * 0.52 * pulse * self.cue_spring.value;
        draw_circle_lines(
            cx,
            cy,
            scale * 1.15,
            6.0,
            Color::from_rgba(255, 202, 58, (160.0 + audio.pulse * 80.0) as u8),
        );
        draw_gesture_pictogram(GestureCue::SquatDown, vec2(cx, cy), scale, KID_COLORS[1]);
        // Three mystery-box stars — sparse, large
        for i in 0..3 {
            let angle = (i as f32 / 3.0) * TAU + self.time * 3.0;
            let dist = scale * (0.95 + audio.pulse * 0.2);
            draw_star(
                cx + angle.cos() * dist,
                cy + angle.sin() * dist,
                12.0 + audio.pulse * 6.0,
                KID_COLORS[i % KID_COLORS.len()],
            );
        }
    }

    /// Freeze: ice stick-figure + snowflake crown (no FLOOR IS LAVA text).
    #[allow(dead_code)]
    fn draw_freeze_cue(&self, cx: f32, cy: f32) {
        let scale = hero_edge(screen_width(), screen_height())
            * 0.5
            * bounce_scale(self.freeze_pop.max(0.4))
            * self.cue_spring.value;
        draw_circle(cx, cy, scale * 1.25, Color::from_rgba(140, 220, 255, 48));
        draw_gesture_pictogram(GestureCue::FreezeStill, vec2(cx, cy), scale, ICE_BLUE);
        draw_snowflake(cx, cy - scale * 0.95, scale * 0.28, ICE_BLUE);
    }

    /// Lava warning: rising flame stack (no LOOK OUT text).
    fn draw_lava_warning_cue(&self, cx: f32, cy: f32) {
        let scale = bounce_scale(self.countdown_pop.max(0.4)) * self.cue_spring.value;
        let s = hero_edge(screen_width(), screen_height()) * 0.55 * scale;
        for i in 0..3 {
            let fi = i as f32;
            let wobble = (self.time * 8.0 + fi).sin() * 8.0;
            draw_triangle(
                vec2(cx + wobble, cy - s * (0.9 + fi * 0.28)),
                vec2(cx - s * (0.5 - fi * 0.08) + wobble, cy + s * 0.4),
                vec2(cx + s * (0.5 - fi * 0.08) + wobble, cy + s * 0.4),
                Color::from_rgba(255, (120 + i * 40) as u8, 40, 235),
            );
        }
        draw_circle(cx, cy + s * 0.22, s * 0.24, Color::from_rgba(255, 240, 120, 255));
    }

    // --- Freeze meter: quiet ring (dots, not snowflake glitter) ---
    fn draw_freeze_meter(&self, game: &SupernovaGame, cx: f32, cy: f32) {
        let radius = hero_edge(screen_width(), screen_height()) * 0.72 + game.energy * 20.0;
        let segments = 16;
        let filled = (segments as f32 * game.freeze_progress) as usize;

        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * TAU - std::f32::consts::FRAC_PI_2;
            let next_angle = ((i + 1) as f32 / segments as f32) * TAU - std::f32::consts::FRAC_PI_2;
            let mid_angle = (angle + next_angle) * 0.5;
            let dx = cx + mid_angle.cos() * radius;
            let dy = cy + mid_angle.sin() * radius;

            if i < filled {
                draw_circle(dx, dy, 7.0, ICE_BLUE);
            } else {
                draw_circle(dx, dy, 3.5, Color::from_rgba(255, 255, 255, 30));
            }
        }

        if game.freeze_wobble {
            let pulse = (self.time * 10.0).sin().abs();
            draw_circle_lines(
                cx, cy, radius + 16.0,
                4.0 + pulse * 2.0,
                Color::from_rgba(255, 89, 94, (120.0 + pulse * 100.0) as u8),
            );
        }
    }

    // --- Action icons: bigger bouncing pictograms during dance ---
    #[allow(dead_code)]
    fn draw_action_icons(&self, game: &SupernovaGame, w: f32, h: f32) {
        let icon_y = h * 0.86;
        let icon_r = 28.0;
        let spacing = w * 0.24;
        let start_x = w * 0.5 - spacing;

        let punch_bounce = bounce_scale((self.time * 2.0).rem_euclid(1.0));
        draw_circle(start_x, icon_y, icon_r * punch_bounce, KID_COLORS[0]);
        draw_circle(start_x, icon_y, icon_r * 0.5 * punch_bounce, WHITE);

        let squat_bounce = bounce_scale((self.time * 2.0 + 0.33).rem_euclid(1.0));
        let sx = start_x + spacing;
        draw_circle(sx, icon_y, icon_r * squat_bounce, KID_COLORS[3]);
        draw_triangle(
            vec2(sx, icon_y + icon_r * 0.45),
            vec2(sx - icon_r * 0.4, icon_y - icon_r * 0.25),
            vec2(sx + icon_r * 0.4, icon_y - icon_r * 0.25),
            WHITE,
        );

        let clap_phase = (self.time * 4.0).sin().abs();
        let clap_gap = icon_r * 0.35 * (1.0 - clap_phase);
        let cx_icon = start_x + spacing * 2.0;
        draw_circle(cx_icon - clap_gap, icon_y, icon_r * 0.6, KID_COLORS[1]);
        draw_circle(cx_icon + clap_gap, icon_y, icon_r * 0.6, KID_COLORS[4]);

        for feedback in game.feedback.iter() {
            if feedback.hit && feedback.on_beat {
                draw_circle_lines(w * 0.5, icon_y, icon_r * 2.8, 3.5, Color::from_rgba(138, 201, 38, 180));
            }
        }
    }

    // --- Combo stars during dance ---
    #[allow(dead_code)]
    fn draw_combo_stars(&self, game: &SupernovaGame, w: f32, h: f32) {
        if game.combo < 2 {
            return;
        }
        let star_count = (game.combo as usize).min(8);
        let star_r = 10.0;
        let total_w = star_count as f32 * (star_r * 2.5);
        let start_x = w * 0.5 - total_w * 0.5 + star_r;
        let y = h * 0.24;

        for i in 0..star_count {
            let pop = bounce_scale((self.time * 1.5 + i as f32 * 0.15).rem_euclid(1.0));
            draw_star(
                start_x + i as f32 * star_r * 2.5,
                y,
                star_r * pop,
                KID_COLORS[i % KID_COLORS.len()],
            );
        }
    }

    // --- Freeze stars: top-left row of earned ice stars ---
    #[allow(dead_code)]
    fn draw_freeze_stars(&self, game: &SupernovaGame, w: f32, h: f32) {
        if game.freeze_stars == 0 {
            return;
        }
        let count = (game.freeze_stars as usize).min(10);
        let star_r = 12.0;
        let y = h * 0.08;
        let start_x = w * 0.5 - (count as f32 * star_r * 2.6) * 0.5 + star_r;
        for i in 0..count {
            let pop = bounce_scale((self.time * 1.2 + i as f32 * 0.2).rem_euclid(1.0));
            draw_star(start_x + i as f32 * star_r * 2.6, y, star_r * pop, ICE_BLUE);
        }
    }

    // --- Perfect freeze star pop ---
    fn draw_perfect_star(&self, cx: f32, cy: f32) {
        let pop = bounce_scale((self.time * 2.0).rem_euclid(1.0)) * self.result_spring.value;
        let r = hero_edge(screen_width(), screen_height()) * 0.42 * pop;
        draw_star(cx, cy, r, KID_COLORS[1]);
        for i in 0..4 {
            let angle = (i as f32 / 4.0) * TAU + self.time * 3.0;
            draw_star(cx + angle.cos() * r * 1.45, cy + angle.sin() * r * 1.45, r * 0.28, ICE_BLUE);
        }
    }

    // --- Countdown: BIG bouncy digits only (universal — not English words) ---
    fn draw_countdown(&self, game: &SupernovaGame, cx: f32, cy: f32) {
        let num = game.countdown_remaining.ceil() as u8;
        let scale = bounce_scale(self.countdown_pop);
        let edge = hero_edge(screen_width(), screen_height());
        if num == 0 {
            let r = edge * 0.48 * scale;
            draw_circle(cx, cy, r * 1.4, Color::from_rgba(80, 220, 90, 80));
            draw_circle(cx, cy, r, Color::from_rgba(80, 220, 90, 230));
            draw_triangle(
                vec2(cx - r * 0.28, cy - r * 0.4),
                vec2(cx - r * 0.28, cy + r * 0.4),
                vec2(cx + r * 0.45, cy),
                WHITE,
            );
            return;
        }
        // Vector-ish big disc + numeral — still readable; VO says Three/Two/One.
        let font_size = edge * 0.85 * scale;
        let label = format!("{num}");
        let color = KID_COLORS[(num as usize).saturating_sub(1) % KID_COLORS.len()];
        draw_circle(cx, cy, edge * 0.42 * scale, Color::new(color.r, color.g, color.b, 0.22));
        draw_circle_lines(cx, cy, edge * 0.48 * scale, 5.0, Color::from_rgba(255, 255, 255, 120));
        let size = measure_text(&label, None, font_size as u16, 1.0);
        draw_text(&label, cx - size.width * 0.5, cy + font_size * 0.35, font_size, color);
    }

    fn draw_confetti(&self, shake: Vec2) {
        for c in &self.confetti {
            let alpha = ((c.life / c.max_life) * 255.0) as u8;
            let color = Color::from_rgba(
                (c.color.r * 255.0) as u8,
                (c.color.g * 255.0) as u8,
                (c.color.b * 255.0) as u8,
                alpha,
            );
            let px = c.pos.x + shake.x;
            let py = c.pos.y + shake.y;
            if c.is_star {
                draw_star(px, py, c.w * 0.5 * (c.life / c.max_life), color);
            } else {
                let hw = c.w * 0.5;
                let hh = c.h * 0.5;
                let cos_r = c.rot.cos();
                let sin_r = c.rot.sin();
                draw_triangle(
                    vec2(px + cos_r * hw, py + sin_r * hw),
                    vec2(px - sin_r * hh, py + cos_r * hh),
                    vec2(px - cos_r * hw, py - sin_r * hh),
                    color,
                );
            }
        }
    }

    fn draw_flash(&self) {
        if self.flash_alpha > 0.01 {
            let w = screen_width();
            let h = screen_height();
            draw_rectangle(
                0.0, 0.0, w, h,
                Color::from_rgba(255, 255, 255, (self.flash_alpha * 180.0) as u8),
            );
        }
    }

    // --- Result: mascot + stars + clap pictogram (no word walls) ---
    fn draw_result(&self, game: &SupernovaGame, w: f32, h: f32, audio: AudioVisual) {
        let cx = w * 0.5;
        let afterglow = (self.time * 1.5).sin().abs();
        draw_rectangle(
            0.0,
            0.0,
            w,
            h,
            Color::new(1.0, 0.72, 0.22, 0.04 + afterglow * 0.05 + audio.energy * 0.02),
        );

        self.draw_mascot(game, cx, h * 0.28, 1.0, false, true, audio);

        // Celebration burst instead of WOW/YAY text
        let pop = self.result_spring.value * bounce_scale((self.time * 0.8).rem_euclid(1.0));
        let burst_r = hero_edge(w, h) * 0.38 * pop;
        draw_soft_glow(
            vec2(cx, h * 0.52),
            burst_r * 1.6,
            Color::from_rgba(255, 202, 58, 120),
            3,
        );
        draw_circle(cx, h * 0.52, burst_r * 1.3, Color::from_rgba(255, 202, 58, 50));
        draw_star(cx, h * 0.52, burst_r, KID_COLORS[1]);
        for i in 0..4 {
            let angle = (i as f32 / 4.0) * TAU + self.time;
            draw_star(
                cx + angle.cos() * burst_r * 1.55,
                h * 0.52 + angle.sin() * burst_r * 1.55,
                burst_r * 0.22,
                KID_COLORS[i % KID_COLORS.len()],
            );
        }

        let star_count = (game.freeze_stars as usize).clamp(1, 3);
        let star_r = hero_edge(w, h) * 0.12;
        let total = star_count as f32 * star_r * 3.0;
        let sx = cx - total * 0.5 + star_r * 1.5;
        for i in 0..star_count {
            let s_pop = bounce_scale((self.time * 1.2 + i as f32 * 0.2).rem_euclid(1.0));
            draw_star(sx + i as f32 * star_r * 3.0, h * 0.68, star_r * s_pop, KID_COLORS[1]);
        }

        self.draw_clap_prompt(cx, h * 0.84);
    }

    // --- Clap prompt: two big hands clapping ---
    fn draw_clap_prompt(&self, cx: f32, cy: f32) {
        let pulse = (self.time * 5.0).sin().abs();
        let hand_r = hero_edge(screen_width(), screen_height()) * 0.22;
        let gap = hand_r * 0.15 + (1.0 - pulse) * hand_r * 0.85;

        draw_circle(cx - gap, cy, hand_r, KID_COLORS[1]);
        draw_circle(cx - gap, cy, hand_r * 0.5, WHITE);
        draw_circle(cx + gap, cy, hand_r, KID_COLORS[4]);
        draw_circle(cx + gap, cy, hand_r * 0.5, WHITE);

        if pulse > 0.85 {
            for i in 0..4 {
                let angle = (i as f32 / 4.0) * TAU + self.time * 2.0;
                let dist = hand_r * 1.4 + pulse * hand_r * 0.3;
                draw_star(
                    cx + angle.cos() * dist,
                    cy + angle.sin() * dist,
                    hand_r * 0.22,
                    KID_COLORS[i % KID_COLORS.len()],
                );
            }
        }
    }
}

// --- Helper: draw a 5-pointed star ---
fn draw_star(cx: f32, cy: f32, r: f32, color: Color) {
    let inner_r = r * 0.45;
    let mut points: Vec<Vec2> = Vec::with_capacity(10);
    for i in 0..10 {
        let angle = (i as f32 / 10.0) * TAU - std::f32::consts::FRAC_PI_2;
        let radius = if i % 2 == 0 { r } else { inner_r };
        points.push(vec2(cx + angle.cos() * radius, cy + angle.sin() * radius));
    }
    for i in 0..10 {
        let next = (i + 1) % 10;
        draw_triangle(vec2(cx, cy), points[i], points[next], color);
    }
}

// --- Helper: draw a 6-spoke snowflake ---
fn draw_snowflake(cx: f32, cy: f32, r: f32, color: Color) {
    for i in 0..6 {
        let angle = (i as f32 / 6.0) * TAU;
        let ex = cx + angle.cos() * r;
        let ey = cy + angle.sin() * r;
        draw_line(cx, cy, ex, ey, 1.5, color);
        // Small branches near the tip.
        let bx = cx + angle.cos() * r * 0.65;
        let by = cy + angle.sin() * r * 0.65;
        for side in &[-1.0_f32, 1.0] {
            let branch_angle = angle + side * 0.6;
            draw_line(
                bx, by,
                bx + branch_angle.cos() * r * 0.3,
                by + branch_angle.sin() * r * 0.3,
                1.2, color,
            );
        }
    }
}

// --- Helper: lerp between two colors ---
fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);
    Color::from_rgba(
        ((a.r + (b.r - a.r) * t) * 255.0) as u8,
        ((a.g + (b.g - a.g) * t) * 255.0) as u8,
        ((a.b + (b.b - a.b) * t) * 255.0) as u8,
        255,
    )
}

fn average_pose_point(pose: PoseFrame, a: usize, b: usize) -> Vec2 {
    vec2(
        (pose.keypoints[a].x + pose.keypoints[b].x) * 0.5,
        (pose.keypoints[a].y + pose.keypoints[b].y) * 0.5,
    )
}

#[derive(Clone, Copy)]
enum SignalKind {
    Go,
    Warn,
    Stop,
}

#[derive(Clone, Copy)]
enum GestureCue {
    Jump,
    Clap,
    Paint,
    Tiptoe,
    Wiggle,
    Reach,
    March,
    RoyalWave,
    SquatDown,
    FreezeStill,
}

impl GestureCue {
    fn from_index(i: usize) -> Self {
        match i % 8 {
            0 => Self::Jump,
            1 => Self::Clap,
            2 => Self::Paint,
            3 => Self::Tiptoe,
            4 => Self::Wiggle,
            5 => Self::Reach,
            6 => Self::March,
            _ => Self::RoyalWave,
        }
    }
}

/// Big silhouette gestures readable at 1.5–4m — no text.
fn draw_gesture_pictogram(cue: GestureCue, center: Vec2, scale: f32, color: Color) {
    let s = scale;
    let cx = center.x;
    let cy = center.y;
    match cue {
        GestureCue::Jump => {
            draw_circle(cx, cy - s * 0.42, s * 0.16, WHITE);
            draw_line(cx, cy - s * 0.26, cx, cy + s * 0.05, 5.0, color);
            draw_line(cx, cy - s * 0.1, cx - s * 0.35, cy - s * 0.28, 4.0, color);
            draw_line(cx, cy - s * 0.1, cx + s * 0.35, cy - s * 0.28, 4.0, color);
            draw_line(cx, cy + s * 0.05, cx - s * 0.28, cy + s * 0.42, 4.0, color);
            draw_line(cx, cy + s * 0.05, cx + s * 0.28, cy + s * 0.42, 4.0, color);
            draw_triangle(
                vec2(cx, cy - s * 0.72),
                vec2(cx - s * 0.14, cy - s * 0.52),
                vec2(cx + s * 0.14, cy - s * 0.52),
                WHITE,
            );
        }
        GestureCue::Clap => {
            let gap = s * 0.12;
            draw_circle(cx - s * 0.32 - gap, cy, s * 0.28, color);
            draw_circle(cx + s * 0.32 + gap, cy, s * 0.28, color);
            draw_circle(cx - s * 0.32 - gap, cy, s * 0.12, WHITE);
            draw_circle(cx + s * 0.32 + gap, cy, s * 0.12, WHITE);
        }
        GestureCue::Paint | GestureCue::Reach | GestureCue::RoyalWave => {
            draw_circle(cx, cy - s * 0.15, s * 0.16, WHITE);
            draw_line(cx, cy, cx, cy + s * 0.35, 5.0, color);
            draw_line(cx, cy + s * 0.05, cx - s * 0.22, cy + s * 0.45, 4.0, color);
            draw_line(cx, cy + s * 0.05, cx + s * 0.22, cy + s * 0.45, 4.0, color);
            draw_line(cx, cy - s * 0.05, cx - s * 0.4, cy - s * 0.55, 5.0, color);
            draw_line(cx, cy - s * 0.05, cx + s * 0.4, cy - s * 0.55, 5.0, color);
            draw_circle(cx - s * 0.42, cy - s * 0.58, s * 0.1, KID_COLORS[1]);
            draw_circle(cx + s * 0.42, cy - s * 0.58, s * 0.1, KID_COLORS[4]);
        }
        GestureCue::Tiptoe => {
            draw_circle(cx, cy - s * 0.35, s * 0.14, WHITE);
            draw_line(cx, cy - s * 0.2, cx, cy + s * 0.2, 4.0, color);
            draw_line(cx, cy + s * 0.2, cx - s * 0.12, cy + s * 0.5, 3.5, color);
            draw_line(cx, cy + s * 0.2, cx + s * 0.12, cy + s * 0.5, 3.5, color);
            draw_circle(cx - s * 0.12, cy + s * 0.55, 4.0, color);
            draw_circle(cx + s * 0.12, cy + s * 0.55, 4.0, color);
        }
        GestureCue::Wiggle => {
            draw_circle(cx, cy - s * 0.3, s * 0.15, WHITE);
            for i in 0..4 {
                let t = i as f32 / 3.0;
                let x = cx + ((t * TAU).sin()) * s * 0.25;
                let y = cy - s * 0.1 + t * s * 0.55;
                draw_circle(x, y, s * 0.08, color);
            }
        }
        GestureCue::March => {
            draw_circle(cx, cy - s * 0.38, s * 0.15, WHITE);
            draw_line(cx, cy - s * 0.22, cx, cy + s * 0.1, 5.0, color);
            draw_line(cx, cy + s * 0.1, cx - s * 0.25, cy + s * 0.45, 4.0, color);
            draw_line(cx, cy + s * 0.1, cx + s * 0.2, cy + s * 0.35, 4.0, color);
            draw_line(cx, cy - s * 0.05, cx + s * 0.4, cy - s * 0.25, 4.0, color);
        }
        GestureCue::SquatDown => {
            draw_circle(cx - s * 0.18, cy - s * 0.15, s * 0.14, WHITE);
            draw_line(cx - s * 0.18, cy, cx, cy + s * 0.15, 5.0, color);
            draw_line(cx, cy + s * 0.15, cx - s * 0.35, cy + s * 0.4, 4.0, color);
            draw_line(cx, cy + s * 0.15, cx + s * 0.25, cy + s * 0.4, 4.0, color);
            draw_triangle(
                vec2(cx, cy + s * 0.55),
                vec2(cx - s * 0.22, cy + s * 0.25),
                vec2(cx + s * 0.22, cy + s * 0.25),
                Color::from_rgba(255, 202, 58, 255),
            );
        }
        GestureCue::FreezeStill => {
            draw_circle(cx, cy - s * 0.4, s * 0.16, ICE_BLUE);
            draw_line(cx, cy - s * 0.22, cx, cy + s * 0.15, 6.0, ICE_BLUE);
            draw_line(cx - s * 0.35, cy - s * 0.05, cx + s * 0.35, cy - s * 0.05, 5.0, ICE_BLUE);
            draw_line(cx, cy + s * 0.15, cx - s * 0.2, cy + s * 0.5, 5.0, ICE_BLUE);
            draw_line(cx, cy + s * 0.15, cx + s * 0.2, cy + s * 0.5, 5.0, ICE_BLUE);
            draw_rectangle(cx - s * 0.28, cy - s * 0.08, s * 0.56, s * 0.1, WHITE);
        }
    }
}
