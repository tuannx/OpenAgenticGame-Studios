//! Generic renderer for config-driven games.
//!
//! Dispatches drawing to mechanic-specific visuals, themed by [`GameConfig`].

use brainbreak_core::config::{GameConfig, Mechanic, ThemeStyle};
use brainbreak_core::config_game::{ConfigGame, ConfigPhase};
use macroquad::prelude::*;

const TAU: f32 = std::f32::consts::TAU;

/// Particle for hit/miss feedback.
#[derive(Clone, Copy)]
struct FxParticle {
    pos: Vec2,
    vel: Vec2,
    life: f32,
    color: Color,
}

/// Renderer state for a config-driven game session.
#[derive(Default)]
pub struct ConfigStage {
    particles: Vec<FxParticle>,
    flash_alpha: f32,
    time: f32,
}

impl ConfigStage {
    pub fn update(&mut self, dt: f32, game: &ConfigGame) {
        self.time += dt;

        // Spawn particles on hit.
        if game.hit_flash {
            let center = vec2(screen_width() * 0.5, screen_height() * 0.4);
            for _ in 0..8 {
                let angle = macroquad::rand::gen_range(0.0, TAU);
                let speed = macroquad::rand::gen_range(60.0, 180.0);
                self.particles.push(FxParticle {
                    pos: center,
                    vel: vec2(angle.cos() * speed, angle.sin() * speed),
                    life: 0.6,
                    color: GOLD,
                });
            }
            self.flash_alpha = 0.3;
        }
        if game.miss_flash {
            self.flash_alpha = 0.15;
        }

        // Decay flash.
        self.flash_alpha = (self.flash_alpha - dt * 2.0).max(0.0);

        // Update particles.
        for p in self.particles.iter_mut() {
            p.pos += p.vel * dt;
            p.vel.y += 200.0 * dt;
            p.life -= dt;
        }
        self.particles.retain(|p| p.life > 0.0);
    }

    pub fn draw(&self, game: &ConfigGame, config: &GameConfig) {
        let w = screen_width();
        let h = screen_height();
        let primary = color_from_hex(config.theme.primary);
        let secondary = color_from_hex(config.theme.secondary);

        // Background gradient by style.
        match config.theme.style {
            ThemeStyle::Neon => {
                clear_background(Color::from_rgba(6, 8, 24, 255));
            }
            ThemeStyle::Cosmic => {
                clear_background(Color::from_rgba(10, 4, 30, 255));
            }
            ThemeStyle::Minimal => {
                clear_background(Color::from_rgba(18, 18, 22, 255));
            }
        }

        match game.phase {
            ConfigPhase::Ready => self.draw_ready(w, h, config, primary),
            ConfigPhase::Countdown => self.draw_countdown(w, h, game, primary),
            ConfigPhase::Active => {
                match config.mechanic {
                    Mechanic::Dodge => self.draw_dodge(w, h, game, primary, secondary),
                    Mechanic::Pump => self.draw_pump(w, h, game, primary, secondary),
                    Mechanic::Catch => self.draw_catch(w, h, game, primary, secondary),
                    Mechanic::Hold => self.draw_hold(w, h, game, primary),
                    Mechanic::Pattern => self.draw_pattern(w, h, game, config, primary, secondary),
                }
                self.draw_hud(w, game, config, primary);
            }
            ConfigPhase::Result => self.draw_result(w, h, game, config, primary),
        }

        // Flash overlay.
        if self.flash_alpha > 0.0 {
            let flash_color = if game.miss_flash {
                Color::new(1.0, 0.2, 0.2, self.flash_alpha)
            } else {
                Color::new(1.0, 1.0, 0.8, self.flash_alpha)
            };
            draw_rectangle(0.0, 0.0, w, h, flash_color);
        }

        // Particles.
        for p in &self.particles {
            let alpha = (p.life / 0.6).clamp(0.0, 1.0);
            draw_circle(p.pos.x, p.pos.y, 3.0 + alpha * 3.0, Color::new(
                p.color.r, p.color.g, p.color.b, alpha,
            ));
        }
    }

    // --- Phase renderers ---

    fn draw_ready(&self, w: f32, h: f32, config: &GameConfig, primary: Color) {
        draw_text_ex(&config.title, w * 0.5 - 100.0, h * 0.4, TextParams {
            font_size: 36, color: primary, ..Default::default()
        });
        draw_text_ex("STEP INTO FRAME", w * 0.5 - 80.0, h * 0.55, TextParams {
            font_size: 18, color: WHITE, ..Default::default()
        });
    }

    fn draw_countdown(&self, w: f32, h: f32, game: &ConfigGame, primary: Color) {
        let num = (game.countdown_remaining.ceil() as u8).max(1);
        let text = format!("{num}");
        draw_text_ex(&text, w * 0.5 - 15.0, h * 0.5, TextParams {
            font_size: 72, color: primary, ..Default::default()
        });
    }

    fn draw_dodge(&self, w: f32, h: f32, game: &ConfigGame, primary: Color, secondary: Color) {
        let lane_w = w / 5.0;
        let base_y = h * 0.85;

        // Draw lanes.
        for lane in -1..=1i8 {
            let x = w * 0.5 + lane as f32 * lane_w;
            draw_line(x, h * 0.2, x, base_y, 1.0, Color::new(0.3, 0.3, 0.5, 0.3));
        }

        // Draw entities.
        for entity in &game.entities[..game.entity_count] {
            if entity.resolved { continue; }
            let x = w * 0.5 + entity.lane as f32 * lane_w;
            let y = h * 0.2 + (1.0 - entity.distance) * (base_y - h * 0.2);
            let size = 18.0;
            draw_rectangle(x - size, y - size, size * 2.0, size * 2.0, secondary);
        }

        // Draw player.
        let px = w * 0.5 + game.players[0].lane as f32 * lane_w;
        draw_circle(px, base_y, 14.0, primary);
    }

    fn draw_pump(&self, w: f32, h: f32, game: &ConfigGame, primary: Color, secondary: Color) {
        let cx = w * 0.5;
        let cy = h * 0.45;
        let radius = 40.0 + game.energy * 60.0;
        let pulse = 1.0 + (self.time * 6.0).sin() * 0.05 * game.energy;

        // Core glow.
        draw_circle(cx, cy, radius * pulse * 1.3, Color::new(
            primary.r, primary.g, primary.b, 0.15,
        ));
        draw_circle(cx, cy, radius * pulse, primary);

        // Energy ring.
        let segments = 32;
        for i in 0..segments {
            let frac = i as f32 / segments as f32;
            if frac > game.energy { break; }
            let angle = frac * TAU - std::f32::consts::FRAC_PI_2;
            let x = cx + angle.cos() * (radius + 20.0);
            let y = cy + angle.sin() * (radius + 20.0);
            draw_circle(x, y, 4.0, secondary);
        }

        // Percentage text.
        let pct = format!("{}%", (game.energy * 100.0) as u8);
        draw_text_ex(&pct, cx - 20.0, cy + 8.0, TextParams {
            font_size: 24, color: WHITE, ..Default::default()
        });
    }

    fn draw_catch(&self, w: f32, h: f32, game: &ConfigGame, primary: Color, secondary: Color) {
        let lane_w = w / 5.0;
        let base_y = h * 0.85;

        // Draw falling targets.
        for entity in &game.entities[..game.entity_count] {
            if entity.resolved { continue; }
            let x = w * 0.5 + entity.lane as f32 * lane_w;
            let y = h * 0.1 + (1.0 - entity.distance) * (base_y - h * 0.1);
            draw_circle(x, y, 12.0, secondary);
        }

        // Draw player basket.
        let px = w * 0.5 + game.players[0].lane as f32 * lane_w;
        draw_rectangle(px - 20.0, base_y - 5.0, 40.0, 10.0, primary);
    }

    fn draw_hold(&self, w: f32, h: f32, game: &ConfigGame, primary: Color) {
        let cx = w * 0.5;
        let cy = h * 0.45;
        let progress = game.players[0].hold_progress;

        // Progress ring.
        let segments = 24;
        for i in 0..segments {
            let frac = i as f32 / segments as f32;
            let angle = frac * TAU - std::f32::consts::FRAC_PI_2;
            let x = cx + angle.cos() * 60.0;
            let y = cy + angle.sin() * 60.0;
            let color = if frac <= progress { primary } else { Color::new(0.3, 0.3, 0.4, 0.5) };
            draw_circle(x, y, 5.0, color);
        }

        // Center text.
        let pct = format!("{}%", (progress * 100.0) as u8);
        draw_text_ex(&pct, cx - 18.0, cy + 8.0, TextParams {
            font_size: 22, color: WHITE, ..Default::default()
        });
        draw_text_ex("HOLD POSE", cx - 45.0, cy + 90.0, TextParams {
            font_size: 16, color: WHITE, ..Default::default()
        });
    }

    fn draw_pattern(&self, w: f32, h: f32, game: &ConfigGame, config: &GameConfig, primary: Color, secondary: Color) {
        let card_w = 60.0;
        let gap = 16.0;
        let total_w = 4.0 * card_w + 3.0 * gap;
        let start_x = (w - total_w) * 0.5;
        let y = h * 0.35;

        // Draw sequence cards.
        for (i, &action) in game.pattern.iter().enumerate() {
            let x = start_x + i as f32 * (card_w + gap);
            let color = if i < game.pattern_index {
                primary // completed
            } else if i == game.pattern_index {
                secondary // current
            } else {
                Color::new(0.25, 0.25, 0.35, 0.8) // upcoming
            };
            draw_rectangle(x, y, card_w, card_w, color);
            // Action label.
            let label = action_label(action, config);
            draw_text_ex(label, x + 10.0, y + 38.0, TextParams {
                font_size: 14, color: WHITE, ..Default::default()
            });
        }

        draw_text_ex("MIMIC THE SEQUENCE", w * 0.5 - 90.0, h * 0.6, TextParams {
            font_size: 16, color: WHITE, ..Default::default()
        });
    }

    fn draw_hud(&self, w: f32, game: &ConfigGame, config: &GameConfig, primary: Color) {
        // Score.
        let score_text = format!("SCORE {}", game.total_score());
        draw_text_ex(&score_text, 16.0, 30.0, TextParams {
            font_size: 18, color: WHITE, ..Default::default()
        });

        // Combo.
        let combo = game.players[0].combo;
        if combo >= 2 {
            let combo_text = format!("x{combo}");
            draw_text_ex(&combo_text, w - 60.0, 30.0, TextParams {
                font_size: 20, color: primary, ..Default::default()
            });
        }

        // Timer bar.
        let progress = game.progress(config);
        let bar_w = w - 32.0;
        draw_rectangle(16.0, 42.0, bar_w, 4.0, Color::new(0.2, 0.2, 0.3, 0.6));
        draw_rectangle(16.0, 42.0, bar_w * progress, 4.0, primary);

        // Lives (if life-based).
        if config.end.mode == brainbreak_core::config::EndMode::LifeBased {
            let lives = game.players[0].lives;
            for i in 0..lives {
                draw_circle(24.0 + i as f32 * 18.0, 60.0, 6.0, RED);
            }
        }
    }

    fn draw_result(&self, w: f32, h: f32, game: &ConfigGame, config: &GameConfig, primary: Color) {
        let title = match game.outcome {
            Some(brainbreak_core::config_game::ConfigOutcome::Complete) => "COMPLETE!",
            Some(brainbreak_core::config_game::ConfigOutcome::Depleted) => "GAME OVER",
            None => "RESULT",
        };
        draw_text_ex(title, w * 0.5 - 60.0, h * 0.3, TextParams {
            font_size: 32, color: primary, ..Default::default()
        });

        let score = format!("SCORE: {}", game.total_score());
        draw_text_ex(&score, w * 0.5 - 50.0, h * 0.42, TextParams {
            font_size: 22, color: WHITE, ..Default::default()
        });

        let best = format!("BEST: {}", game.best_score);
        draw_text_ex(&best, w * 0.5 - 40.0, h * 0.50, TextParams {
            font_size: 18, color: GOLD, ..Default::default()
        });

        if game.new_best {
            draw_text_ex("NEW BEST!", w * 0.5 - 45.0, h * 0.57, TextParams {
                font_size: 20, color: GOLD, ..Default::default()
            });
        }

        let combo = format!("BEST COMBO: x{}", game.players[0].best_combo);
        draw_text_ex(&combo, w * 0.5 - 60.0, h * 0.64, TextParams {
            font_size: 16, color: WHITE, ..Default::default()
        });

        draw_text_ex("CLAP TO REPLAY", w * 0.5 - 70.0, h * 0.78, TextParams {
            font_size: 16, color: Color::new(0.7, 0.7, 0.8, 0.8 + (self.time * 3.0).sin() * 0.2),
            ..Default::default()
        });

        let _ = config; // used for theme in future
    }
}

// --- Helpers ---

fn color_from_hex(hex: u32) -> Color {
    Color::from_rgba(
        ((hex >> 16) & 0xFF) as u8,
        ((hex >> 8) & 0xFF) as u8,
        (hex & 0xFF) as u8,
        255,
    )
}

fn action_label(action: u32, _config: &GameConfig) -> &'static str {
    match action {
        1 => "L",
        2 => "R",
        4 => "JMP",
        8 => "SQT",
        16 => "LH",
        32 => "RH",
        64 => "CLP",
        _ => "?",
    }
}
