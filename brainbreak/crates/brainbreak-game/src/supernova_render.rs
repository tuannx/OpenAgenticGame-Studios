//! Supernova Freeze Party renderer — Danny Go–inspired kid-friendly visuals.
//! Big mascot blob, DANCE/FREEZE prompts, snowflake icon, confetti & stars.
//! Minimal text (only big single words), maximum visual joy for ages 4–7.

use brainbreak_core::{SupernovaGame, SupernovaOutcome, SupernovaPhase};
use macroquad::prelude::*;

use crate::visuals::AudioVisual;

const TAU: f32 = std::f32::consts::TAU;
const MAX_CONFETTI: usize = 150;

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

#[derive(Default)]
pub struct SupernovaStage {
    confetti: Vec<Confetti>,
    explosion_spawned: bool,
    flash_alpha: f32,
    time: f32,
    countdown_pop: f32,
    last_countdown_num: u8,
    freeze_pop: f32,
    dance_pop: f32,
    last_phase: SupernovaPhase,
    /// Screen-shake magnitude (decays; applied as a jitter offset).
    shake: f32,
    /// Hit-stop timer: briefly freezes stage animation for physical weight.
    hitstop: f32,
}

impl SupernovaStage {
    pub fn update(&mut self, dt: f32, game: &SupernovaGame, _audio: AudioVisual, reduce_motion: bool) {
        // Hit-stop: a beat of stillness on impact makes the drop feel heavy.
        if self.hitstop > 0.0 {
            self.hitstop = (self.hitstop - dt).max(0.0);
            return;
        }
        self.time += dt;

        // Phase-change pop animations.
        if game.phase != self.last_phase {
            if game.phase == SupernovaPhase::Freeze {
                self.freeze_pop = 1.0;
            }
            if game.phase == SupernovaPhase::Dance {
                self.dance_pop = 1.0;
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
        }
        self.countdown_pop = (self.countdown_pop - dt * 3.0).max(0.0);

        // Spawn confetti explosion on drop + impact shake & hit-stop.
        if game.drop_triggered && !self.explosion_spawned {
            self.spawn_celebration(game.energy);
            self.explosion_spawned = true;
            self.flash_alpha = 1.0;
            if !reduce_motion {
                self.shake = 9.0;
                self.hitstop = 0.05;
            }
        }

        // Perfect freeze: small satisfied shake (one-shot frame flag).
        if game.perfect_freeze && !reduce_motion {
            self.shake = self.shake.max(3.5);
        }

        // Reset when returning to countdown.
        if game.phase == SupernovaPhase::Countdown {
            self.explosion_spawned = false;
            self.confetti.clear();
        }

        // Decay flash + shake.
        self.flash_alpha = (self.flash_alpha - dt * 2.0).max(0.0);
        self.shake = (self.shake - dt * 26.0).max(0.0);

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

    pub fn draw(&self, game: &SupernovaGame, audio: AudioVisual, reduce_motion: bool) {
        let w = screen_width();
        let h = screen_height();

        // Beat-synced neon backdrop fills the screen (not shaken, so no edge gaps).
        self.draw_background(game, w, h, audio, reduce_motion);

        // Foreground impact shake offset.
        let shake = if self.shake > 0.05 && !reduce_motion {
            vec2(
                (self.time * 91.0).sin() * self.shake,
                (self.time * 73.0).cos() * self.shake * 0.6,
            )
        } else {
            Vec2::ZERO
        };

        match game.phase {
            SupernovaPhase::Ready => {
                self.draw_mascot(game, w * 0.5 + shake.x, h * 0.45 + shake.y, 0.0, false, true, audio);
            }
            SupernovaPhase::Countdown => {
                self.draw_mascot(game, w * 0.5 + shake.x, h * 0.5 + shake.y, 0.0, false, true, audio);
                self.draw_countdown(game, w * 0.5, h * 0.28);
            }
            SupernovaPhase::Dance => {
                let cx = w * 0.5 + shake.x;
                let cy = h * 0.42 + shake.y;
                self.draw_mascot(game, cx, cy, game.energy, false, true, audio);
                self.draw_energy_ring(game, cx, cy, audio);
                self.draw_dance_prompt(w * 0.5, h * 0.14, audio);
                self.draw_action_icons(game, w, h);
                self.draw_combo_stars(game, w, h);
                self.draw_freeze_stars(game, w, h);
            }
            SupernovaPhase::Freeze => {
                let wobble = if game.freeze_wobble {
                    (self.time * 50.0).sin() * 5.0
                } else {
                    0.0
                };
                let cx = w * 0.5 + wobble + shake.x;
                let cy = h * 0.42 + shake.y;
                self.draw_mascot(game, cx, cy, game.energy, true, true, audio);
                self.draw_freeze_prompt(w * 0.5, h * 0.14);
                self.draw_freeze_meter(game, cx, cy);
                self.draw_freeze_stars(game, w, h);
                if game.perfect_freeze {
                    self.draw_perfect_star(w * 0.5, h * 0.62);
                }
            }
            SupernovaPhase::Drop => {
                self.draw_confetti(shake);
                self.draw_flash();
                self.draw_mascot(game, w * 0.5 + shake.x, h * 0.42 + shake.y, 1.0, false, true, audio);
            }
            SupernovaPhase::Result => {
                self.draw_confetti(Vec2::ZERO);
                self.draw_result(game, w, h, audio);
            }
        }
    }

    // --- Background: neon cyber-party scene that breathes with the music ---
    fn draw_background(&self, game: &SupernovaGame, w: f32, h: f32, audio: AudioVisual, reduce_motion: bool) {
        let energy = game.energy;
        let frozen = game.phase == SupernovaPhase::Freeze;
        // Combined beat drive: sharp transient pulse + sustained loudness.
        let drive = if reduce_motion {
            audio.energy * 0.25
        } else {
            audio.pulse * 0.55 + audio.energy * 0.5
        };

        // 1) Gradient night sky (banded for smoothness).
        let (top, bot) = if frozen {
            (
                Color::from_rgba(16, 42, 84, 255),
                Color::from_rgba(8, 24, 52, 255),
            )
        } else {
            (
                Color::from_rgba(
                    (24.0 + energy * 46.0) as u8,
                    (12.0 + energy * 22.0) as u8,
                    (64.0 + energy * 66.0) as u8,
                    255,
                ),
                Color::from_rgba(
                    (44.0 + energy * 40.0) as u8,
                    (10.0 + energy * 18.0) as u8,
                    (72.0 + energy * 40.0) as u8,
                    255,
                ),
            )
        };
        for row in 0..14 {
            let t = row as f32 / 13.0;
            draw_rectangle(0.0, h * t, w, h / 13.0 + 1.0, lerp_color(top, bot, t));
        }

        // 2) Twinkling stars, brightening with musical energy.
        let star_alpha = 0.16 + audio.energy * 0.4;
        for star in 0..30 {
            let x = ((star * 71) % 997) as f32 / 997.0 * w;
            let y = ((star * 43 + 19) % 251) as f32 / 251.0 * h * 0.3;
            let twinkle = ((self.time * 2.0 + star as f32).sin() * 0.5 + 0.5) * star_alpha;
            let c = if frozen {
                Color::from_rgba(190, 230, 255, (twinkle * 255.0) as u8)
            } else {
                Color::from_rgba(255, 220, 250, (twinkle * 255.0) as u8)
            };
            draw_circle(x, y, 1.0 + (star % 3) as f32 * 0.5, c);
        }

        // 3) Beat-breathing neon sun behind the mascot.
        self.draw_neon_sun(w * 0.5, h * 0.40, w.min(h), drive, frozen, energy);

        // 4) Horizon grid floor scrolling toward the viewer on the beat.
        self.draw_horizon_grid(w, h, drive, frozen, reduce_motion);

        // 5) Side equalizer pylons pulsing with loudness.
        self.draw_equalizer(w, h, audio, frozen, reduce_motion);

        // 6) Falling snowflakes during freeze.
        if frozen {
            for i in 0..12 {
                let fi = i as f32;
                let sx = (w * (0.08 + fi * 0.08) + self.time * (10.0 + fi * 3.0)).rem_euclid(w);
                let sy = (self.time * (30.0 + fi * 8.0) + fi * 90.0).rem_euclid(h);
                draw_snowflake(sx, sy, 5.0 + (fi % 3.0) * 2.0, Color::from_rgba(200, 235, 255, 120));
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
        let base_r = 50.0;
        let radius = base_r + energy * 45.0;

        // Bounce to the music: idle sway + a kick on every beat pulse.
        let beat_kick = if frozen { 0.0 } else { audio.pulse * 7.0 };
        let idle_bounce = if frozen { 0.0 } else { (self.time * 3.0).sin() * 4.0 };
        let squash = if frozen { 1.0 } else { 1.0 + (self.time * 3.0).sin() * 0.03 + audio.pulse * 0.05 };
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
            draw_circle(pos.x, pos.y, radius * 1.5 + audio.pulse * 10.0, Color::new(aura_color.r, aura_color.g, aura_color.b, 0.05 + audio.pulse * 0.06));
            draw_circle(pos.x, pos.y, radius * 1.22 + audio.pulse * 6.0, Color::new(aura_color.r, aura_color.g, aura_color.b, 0.08 + audio.pulse * 0.08));
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

        // Arms: waving during dance, held stiff down during freeze.
        if game.phase == SupernovaPhase::Dance || game.phase == SupernovaPhase::Freeze {
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

    // --- DANCE! prompt: big bouncing word ---
    fn draw_dance_prompt(&self, cx: f32, cy: f32, audio: AudioVisual) {
        let scale = bounce_scale(self.dance_pop);
        let wiggle = (self.time * 6.0).sin() * 0.08;
        let font_size = 44.0 * scale * (1.0 + wiggle) * (1.0 + audio.pulse * 0.08);
        let color = KID_COLORS[0]; // red-coral, high energy
        let size = measure_text("DANCE!", None, font_size as u16, 1.0);
        draw_text("DANCE!", cx - size.width * 0.5, cy + font_size * 0.35, font_size, color);

        // Musical notes floating around the word.
        for i in 0..3 {
            let fi = i as f32;
            let nx = cx + (fi - 1.0) * 90.0 + (self.time * 20.0 + fi * 40.0).sin() * 8.0;
            let ny = cy - 20.0 - ((self.time * 25.0 + fi * 30.0).rem_euclid(40.0));
            draw_circle(nx, ny, 4.0, KID_COLORS[(i + 1) % KID_COLORS.len()]);
        }
    }

    // --- FREEZE! prompt: big snowflake + word ---
    fn draw_freeze_prompt(&self, cx: f32, cy: f32) {
        let scale = bounce_scale(self.freeze_pop);
        let font_size = 44.0 * scale;
        let size = measure_text("FREEZE!", None, font_size as u16, 1.0);
        draw_text("FREEZE!", cx - size.width * 0.5, cy + font_size * 0.35, font_size, ICE_BLUE);

        // Big spinning snowflakes on either side.
        let spin = self.time * 0.8;
        draw_snowflake(cx - size.width * 0.5 - 30.0, cy, 16.0 * scale, WHITE);
        draw_snowflake(cx + size.width * 0.5 + 30.0, cy, 16.0 * scale, WHITE);
        let _ = spin;
    }

    // --- Freeze meter: ring that fills as the child holds still ---
    fn draw_freeze_meter(&self, game: &SupernovaGame, cx: f32, cy: f32) {
        let radius = 110.0 + game.energy * 45.0;
        let segments = 20;
        let filled = (segments as f32 * game.freeze_progress) as usize;

        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * TAU - std::f32::consts::FRAC_PI_2;
            let next_angle = ((i + 1) as f32 / segments as f32) * TAU - std::f32::consts::FRAC_PI_2;
            let mid_angle = (angle + next_angle) * 0.5;
            let dx = cx + mid_angle.cos() * radius;
            let dy = cy + mid_angle.sin() * radius;

            if i < filled {
                draw_snowflake(dx, dy, 5.0, ICE_BLUE);
            } else {
                draw_circle(dx, dy, 2.5, Color::from_rgba(255, 255, 255, 30));
            }
        }

        // Wobble warning ring when moving during freeze.
        if game.freeze_wobble {
            let pulse = (self.time * 10.0).sin().abs();
            draw_circle_lines(
                cx, cy, radius + 14.0,
                3.0 + pulse * 2.0,
                Color::from_rgba(255, 89, 94, (120.0 + pulse * 100.0) as u8),
            );
        }
    }

    // --- Action icons: big bouncing icons during dance ---
    fn draw_action_icons(&self, game: &SupernovaGame, w: f32, h: f32) {
        let icon_y = h * 0.84;
        let icon_r = 22.0;
        let spacing = w * 0.22;
        let start_x = w * 0.5 - spacing;

        // Punch.
        let punch_bounce = bounce_scale((self.time * 2.0).rem_euclid(1.0));
        draw_circle(start_x, icon_y, icon_r * punch_bounce, KID_COLORS[0]);
        draw_circle(start_x, icon_y, icon_r * 0.5 * punch_bounce, WHITE);

        // Squat (down arrow).
        let squat_bounce = bounce_scale((self.time * 2.0 + 0.33).rem_euclid(1.0));
        let sx = start_x + spacing;
        draw_circle(sx, icon_y, icon_r * squat_bounce, KID_COLORS[3]);
        draw_triangle(
            vec2(sx, icon_y + icon_r * 0.4),
            vec2(sx - icon_r * 0.35, icon_y - icon_r * 0.2),
            vec2(sx + icon_r * 0.35, icon_y - icon_r * 0.2),
            WHITE,
        );

        // Clap (two circles meeting).
        let clap_phase = (self.time * 4.0).sin().abs();
        let clap_gap = icon_r * 0.3 * (1.0 - clap_phase);
        let cx_icon = start_x + spacing * 2.0;
        draw_circle(cx_icon - clap_gap, icon_y, icon_r * 0.55, KID_COLORS[1]);
        draw_circle(cx_icon + clap_gap, icon_y, icon_r * 0.55, KID_COLORS[4]);

        // On-beat glow.
        for feedback in game.feedback.iter() {
            if feedback.hit && feedback.on_beat {
                draw_circle_lines(w * 0.5, icon_y, icon_r * 2.5, 3.0, Color::from_rgba(138, 201, 38, 180));
            }
        }
    }

    // --- Combo stars during dance ---
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
        let pop = bounce_scale((self.time * 2.0).rem_euclid(1.0));
        draw_star(cx, cy, 30.0 * pop, KID_COLORS[1]);
        for i in 0..6 {
            let angle = (i as f32 / 6.0) * TAU + self.time * 3.0;
            draw_star(cx + angle.cos() * 45.0, cy + angle.sin() * 45.0, 8.0, ICE_BLUE);
        }
    }

    // --- Countdown: BIG bouncy number ---
    fn draw_countdown(&self, game: &SupernovaGame, cx: f32, cy: f32) {
        let num = game.countdown_remaining.ceil() as u8;
        let label = if num > 0 { format!("{num}") } else { "GO!".to_string() };
        let scale = bounce_scale(self.countdown_pop);
        let font_size = 80.0 * scale;
        let color = KID_COLORS[(3 - num as usize).clamp(0, 5) % KID_COLORS.len()];

        let size = measure_text(&label, None, font_size as u16, 1.0);
        draw_text(&label, cx - size.width * 0.5, cy + font_size * 0.35, font_size, color);
        draw_circle_lines(cx, cy, 60.0 * scale, 4.0, Color::from_rgba(255, 255, 255, 100));
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

    // --- Result: happy mascot + stars + one big word ---
    fn draw_result(&self, game: &SupernovaGame, w: f32, h: f32, audio: AudioVisual) {
        let cx = w * 0.5;

        self.draw_mascot(game, cx, h * 0.30, 1.0, false, true, audio);

        let word = match game.outcome {
            Some(SupernovaOutcome::FullSupernova) => "WOW!",
            _ => "YAY!",
        };
        let pop = bounce_scale((self.time * 0.8).rem_euclid(1.0));
        let font_size = 56.0 * pop;
        let size = measure_text(word, None, font_size as u16, 1.0);
        draw_text(word, cx - size.width * 0.5, h * 0.55 + font_size * 0.35, font_size, KID_COLORS[1]);

        // Stars = freeze stars earned (1–3 shown, scaled).
        let star_count = (game.freeze_stars as usize).clamp(1, 3);
        let star_r = 20.0;
        let total = star_count as f32 * star_r * 3.0;
        let sx = cx - total * 0.5 + star_r * 1.5;
        for i in 0..star_count {
            let s_pop = bounce_scale((self.time * 1.2 + i as f32 * 0.2).rem_euclid(1.0));
            draw_star(sx + i as f32 * star_r * 3.0, h * 0.65, star_r * s_pop, KID_COLORS[1]);
        }

        // Replay: animated clapping hands icon.
        self.draw_clap_prompt(cx, h * 0.82);
    }

    // --- Clap prompt: two big hands clapping ---
    fn draw_clap_prompt(&self, cx: f32, cy: f32) {
        let pulse = (self.time * 5.0).sin().abs();
        let hand_r = 28.0;
        let gap = 5.0 + (1.0 - pulse) * 25.0;

        draw_circle(cx - gap, cy, hand_r, KID_COLORS[1]);
        draw_circle(cx - gap, cy, hand_r * 0.5, WHITE);
        draw_circle(cx + gap, cy, hand_r, KID_COLORS[4]);
        draw_circle(cx + gap, cy, hand_r * 0.5, WHITE);

        if pulse > 0.85 {
            for i in 0..4 {
                let angle = (i as f32 / 4.0) * TAU + self.time * 2.0;
                let dist = 35.0 + pulse * 10.0;
                draw_star(
                    cx + angle.cos() * dist,
                    cy + angle.sin() * dist,
                    6.0,
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
