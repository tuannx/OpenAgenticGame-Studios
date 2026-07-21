use brainbreak_core::{Action, MotionRuntime, PlayerMotionState, PoseFrame};
use macroquad::prelude::*;

const PARTICLE_CAPACITY: usize = 96;
const BONES: [(usize, usize); 12] = [
    (5, 6),
    (5, 7),
    (7, 9),
    (6, 8),
    (8, 10),
    (5, 11),
    (6, 12),
    (11, 12),
    (11, 13),
    (13, 15),
    (12, 14),
    (14, 16),
];

#[derive(Clone, Copy, Debug, Default)]
struct Particle {
    position: Vec2,
    velocity: Vec2,
    life: f32,
    color: Color,
}

pub struct MotionStage {
    particles: [Particle; PARTICLE_CAPACITY],
    particle_cursor: usize,
    hit_flash: [f32; 4],
    miss_flash: [f32; 4],
    elapsed: f32,
}

impl Default for MotionStage {
    fn default() -> Self {
        Self {
            particles: std::array::from_fn(|_| Particle::default()),
            particle_cursor: 0,
            hit_flash: [0.0; 4],
            miss_flash: [0.0; 4],
            elapsed: 0.0,
        }
    }
}

impl MotionStage {
    pub fn update(&mut self, dt: f32, runtime: &MotionRuntime) {
        self.elapsed += dt;
        for particle in &mut self.particles {
            if particle.life <= 0.0 {
                continue;
            }
            particle.life = (particle.life - dt * 1.7).max(0.0);
            particle.position += particle.velocity * dt;
            particle.velocity.y += dt * 0.18;
        }
        for player in 0..4 {
            self.hit_flash[player] = (self.hit_flash[player] - dt * 2.6).max(0.0);
            self.miss_flash[player] = (self.miss_flash[player] - dt * 3.4).max(0.0);
            if runtime.players[player].hit {
                self.hit_flash[player] = 1.0;
                let origin = action_point(runtime.players[player].pose, runtime.game.target);
                self.spawn_burst(origin, player_color(player), 18);
            } else if runtime.players[player].miss {
                self.miss_flash[player] = 1.0;
            }
        }
    }

    pub fn draw(&self, runtime: &MotionRuntime, bounds: Rect, accent: Color) {
        draw_round_panel(bounds, Color::from_rgba(12, 20, 42, 232));
        draw_stage_grid(bounds, accent);
        draw_target_zone(
            bounds,
            runtime.game.target,
            runtime.game.beat_progress,
            accent,
        );

        let local_players = runtime.players[..2]
            .iter()
            .filter(|player| player.pose.is_some())
            .count();
        if local_players == 0 {
            let message = "START CAMERA — YOUR BODY BECOMES THE CONTROLLER";
            let size = measure_text(message, None, 20, 1.0);
            draw_text(
                message,
                bounds.x + (bounds.w - size.width) * 0.5,
                bounds.y + bounds.h * 0.55,
                20.0,
                Color::from_rgba(148, 163, 184, 255),
            );
        }

        for (player, state) in runtime.players.iter().take(2).enumerate() {
            if let Some(pose) = state.pose {
                draw_pose(bounds, &pose, *state, player, self.hit_flash[player]);
            }
            draw_feedback_badge(
                bounds,
                player,
                self.hit_flash[player],
                self.miss_flash[player],
            );
        }

        for particle in &self.particles {
            if particle.life <= 0.0 {
                continue;
            }
            let position = stage_point(bounds, particle.position);
            let mut color = particle.color;
            color.a = particle.life.min(1.0);
            draw_circle(position.x, position.y, 3.0 + particle.life * 5.0, color);
        }

        let privacy = "ON-DEVICE MOTION • LANDMARKS STAY LOCAL";
        draw_text(
            privacy,
            bounds.x + 18.0,
            bounds.y + bounds.h - 16.0,
            13.0,
            Color::from_rgba(100, 116, 139, 255),
        );
    }

    fn spawn_burst(&mut self, origin: Vec2, color: Color, count: usize) {
        for index in 0..count {
            let angle = index as f32 / count as f32 * std::f32::consts::TAU;
            let speed = 0.10 + (index % 5) as f32 * 0.018;
            self.particles[self.particle_cursor] = Particle {
                position: origin,
                velocity: vec2(angle.cos() * speed, angle.sin() * speed - 0.035),
                life: 0.65 + (index % 3) as f32 * 0.12,
                color,
            };
            self.particle_cursor = (self.particle_cursor + 1) % PARTICLE_CAPACITY;
        }
    }
}

pub fn draw_round_panel(bounds: Rect, color: Color) {
    let radius = 18.0_f32.min(bounds.w * 0.25).min(bounds.h * 0.25);
    draw_rectangle(
        bounds.x + radius,
        bounds.y,
        bounds.w - radius * 2.0,
        bounds.h,
        color,
    );
    draw_rectangle(
        bounds.x,
        bounds.y + radius,
        bounds.w,
        bounds.h - radius * 2.0,
        color,
    );
    for (x, y) in [
        (bounds.x + radius, bounds.y + radius),
        (bounds.x + bounds.w - radius, bounds.y + radius),
        (bounds.x + radius, bounds.y + bounds.h - radius),
        (bounds.x + bounds.w - radius, bounds.y + bounds.h - radius),
    ] {
        draw_circle(x, y, radius, color);
    }
}

fn draw_stage_grid(bounds: Rect, accent: Color) {
    for column in 1..8 {
        let x = bounds.x + bounds.w * column as f32 / 8.0;
        draw_line(
            x,
            bounds.y + 12.0,
            x,
            bounds.y + bounds.h - 12.0,
            1.0,
            Color::new(accent.r, accent.g, accent.b, 0.06),
        );
    }
    for row in 1..5 {
        let y = bounds.y + bounds.h * row as f32 / 5.0;
        draw_line(
            bounds.x + 12.0,
            y,
            bounds.x + bounds.w - 12.0,
            y,
            1.0,
            Color::new(accent.r, accent.g, accent.b, 0.06),
        );
    }
}

fn draw_target_zone(bounds: Rect, target: Action, progress: f32, accent: Color) {
    let center = stage_point(bounds, target_point(target));
    let pulse = 1.0 - (progress - 0.72).abs().min(0.72) / 0.72;
    let radius = 34.0 + pulse * 17.0;
    draw_circle(
        center.x,
        center.y,
        radius + 12.0,
        Color::new(accent.r, accent.g, accent.b, 0.08),
    );
    draw_circle_lines(
        center.x,
        center.y,
        radius,
        4.0,
        Color::new(accent.r, accent.g, accent.b, 0.85),
    );
    draw_circle_lines(
        center.x,
        center.y,
        radius * 0.72,
        2.0,
        Color::new(1.0, 1.0, 1.0, 0.35),
    );
    let label = target.label();
    let size = measure_text(label, None, 17, 1.0);
    draw_text(
        label,
        center.x - size.width * 0.5,
        center.y + radius + 25.0,
        17.0,
        WHITE,
    );
}

fn draw_pose(
    bounds: Rect,
    pose: &PoseFrame,
    state: PlayerMotionState,
    player: usize,
    hit_flash: f32,
) {
    let color = player_color(player);
    let confidence = state.quality().clamp(0.0, 1.0);
    let glow = if state.active != 0 {
        1.0
    } else {
        0.35 + confidence * 0.45
    };

    for (from, to) in BONES {
        let a = pose.keypoints[from];
        let b = pose.keypoints[to];
        if a.confidence < 0.25 || b.confidence < 0.25 {
            continue;
        }
        let start = stage_point(bounds, vec2(a.x, a.y));
        let end = stage_point(bounds, vec2(b.x, b.y));
        draw_line(
            start.x,
            start.y,
            end.x,
            end.y,
            10.0 + hit_flash * 8.0,
            Color::new(color.r, color.g, color.b, 0.10 + hit_flash * 0.16),
        );
        draw_line(
            start.x,
            start.y,
            end.x,
            end.y,
            3.0 + glow * 2.0,
            Color::new(color.r, color.g, color.b, 0.48 + glow * 0.42),
        );
    }
    for keypoint in pose.keypoints {
        if keypoint.confidence < 0.25 {
            continue;
        }
        let point = stage_point(bounds, vec2(keypoint.x, keypoint.y));
        draw_circle(
            point.x,
            point.y,
            4.0 + keypoint.confidence * 3.0 + hit_flash * 4.0,
            Color::new(color.r, color.g, color.b, 0.65 + confidence * 0.35),
        );
    }

    let anchor = pose.keypoints[0];
    let label_position = stage_point(bounds, vec2(anchor.x, (anchor.y - 0.07).max(0.04)));
    let action = Action::ALL
        .into_iter()
        .find(|action| state.active & action.mask() != 0);
    let label = if state.calibration < 1.0 {
        format!(
            "P{} • CALIBRATING {:.0}%",
            player + 1,
            state.calibration * 100.0
        )
    } else {
        action.map_or_else(
            || format!("P{} • TRACKED {:.0}%", player + 1, confidence * 100.0),
            |action| format!("P{} • {}", player + 1, action.label()),
        )
    };
    let size = measure_text(&label, None, 15, 1.0);
    draw_rectangle(
        label_position.x - size.width * 0.5 - 8.0,
        label_position.y - 18.0,
        size.width + 16.0,
        24.0,
        Color::from_rgba(3, 7, 18, 205),
    );
    draw_text(
        &label,
        label_position.x - size.width * 0.5,
        label_position.y,
        15.0,
        WHITE,
    );
}

fn draw_feedback_badge(bounds: Rect, player: usize, hit: f32, miss: f32) {
    if hit <= 0.0 && miss <= 0.0 {
        return;
    }
    let text = if hit > miss {
        format!("P{} PERFECT!", player + 1)
    } else {
        format!("P{} TRY AGAIN", player + 1)
    };
    let color = if hit > miss {
        Color::from_rgba(52, 211, 153, 255)
    } else {
        Color::from_rgba(251, 113, 133, 255)
    };
    let alpha = hit.max(miss).min(1.0);
    let x = bounds.x + bounds.w * (0.30 + player as f32 * 0.40);
    let y = bounds.y + 44.0 - (1.0 - alpha) * 12.0;
    let size = measure_text(&text, None, 24, 1.0);
    draw_text(
        &text,
        x - size.width * 0.5,
        y,
        24.0,
        Color::new(color.r, color.g, color.b, alpha),
    );
}

fn action_point(pose: Option<PoseFrame>, action: Action) -> Vec2 {
    let Some(pose) = pose else {
        return target_point(action);
    };
    let point = match action {
        Action::LeftUp => pose.keypoints[9],
        Action::RightUp => pose.keypoints[10],
        Action::Clap => average(pose.keypoints[9], pose.keypoints[10]),
        _ => average(pose.keypoints[11], pose.keypoints[12]),
    };
    vec2(point.x, point.y)
}

fn average(
    a: brainbreak_core::Keypoint,
    b: brainbreak_core::Keypoint,
) -> brainbreak_core::Keypoint {
    brainbreak_core::Keypoint {
        x: (a.x + b.x) * 0.5,
        y: (a.y + b.y) * 0.5,
        confidence: (a.confidence + b.confidence) * 0.5,
    }
}

fn target_point(action: Action) -> Vec2 {
    match action {
        Action::MoveLeft => vec2(0.18, 0.50),
        Action::MoveRight => vec2(0.82, 0.50),
        Action::Jump => vec2(0.50, 0.18),
        Action::Squat => vec2(0.50, 0.78),
        Action::LeftUp => vec2(0.30, 0.25),
        Action::RightUp => vec2(0.70, 0.25),
        Action::Clap => vec2(0.50, 0.43),
    }
}

fn stage_point(bounds: Rect, normalized: Vec2) -> Vec2 {
    vec2(
        bounds.x + normalized.x.clamp(0.0, 1.0) * bounds.w,
        bounds.y + normalized.y.clamp(0.0, 1.0) * bounds.h,
    )
}

fn player_color(player: usize) -> Color {
    match player {
        0 => Color::from_rgba(34, 211, 238, 255),
        1 => Color::from_rgba(196, 181, 253, 255),
        2 => Color::from_rgba(251, 191, 36, 255),
        _ => Color::from_rgba(52, 211, 153, 255),
    }
}
