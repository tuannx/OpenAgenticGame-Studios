use brainbreak_core::{
    MotionRuntime, PoseFrame, RunnerFeedback, RunnerGame, RunnerHazard, RunnerObstacle,
};
use macroquad::prelude::*;

const PARTICLE_CAPACITY: usize = 180;
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
pub struct AudioVisual {
    pub phase: f32,
    pub pulse: f32,
    pub energy: f32,
    pub playing: bool,
}

#[derive(Clone, Copy, Debug, Default)]
struct Particle {
    position: Vec2,
    velocity: Vec2,
    life: f32,
    size: f32,
    color: Color,
}

pub struct RunnerStage {
    particles: [Particle; PARTICLE_CAPACITY],
    particle_cursor: usize,
    feedback_flash: [f32; 4],
    elapsed: f32,
}

impl Default for RunnerStage {
    fn default() -> Self {
        Self {
            particles: std::array::from_fn(|_| Particle::default()),
            particle_cursor: 0,
            feedback_flash: [0.0; 4],
            elapsed: 0.0,
        }
    }
}

impl RunnerStage {
    pub fn update(&mut self, dt: f32, game: &RunnerGame) {
        self.elapsed += dt;
        for particle in &mut self.particles {
            if particle.life <= 0.0 {
                continue;
            }
            particle.life = (particle.life - dt * 1.35).max(0.0);
            particle.position += particle.velocity * dt;
            particle.velocity.y += dt * 0.24;
        }
        for player in 0..4 {
            self.feedback_flash[player] = (self.feedback_flash[player] - dt * 2.7).max(0.0);
            if game.feedback[player] != RunnerFeedback::None {
                self.feedback_flash[player] = 1.0;
                let color = feedback_color(game.feedback[player]);
                self.spawn_burst(
                    vec2(lane_normalized(game.players[player].lane), 0.82),
                    color,
                    22,
                );
            }
        }
    }

    pub fn draw(
        &self,
        game: &RunnerGame,
        motion: &MotionRuntime,
        bounds: Rect,
        audio: AudioVisual,
        reduce_motion: bool,
    ) {
        draw_backdrop(bounds, audio, self.elapsed, reduce_motion);
        draw_track(bounds, game, audio, self.elapsed);
        for obstacle in game
            .obstacles
            .iter()
            .flatten()
            .filter(|obstacle| obstacle.distance >= -0.13)
        {
            draw_obstacle(bounds, *obstacle, audio);
        }
        for (player, state) in game.players.iter().enumerate().rev() {
            if !state.evaluated && player >= 2 {
                continue;
            }
            draw_runner(
                bounds,
                player,
                game,
                motion.players[player].pose,
                self.feedback_flash[player],
                audio,
            );
        }
        self.draw_particles(bounds, audio, reduce_motion);
        draw_next_cue(bounds, game.next_cue(), audio);
    }

    fn draw_particles(&self, bounds: Rect, audio: AudioVisual, reduce_motion: bool) {
        let size_multiplier = if reduce_motion {
            0.55
        } else {
            1.0 + audio.energy * 0.7
        };
        for particle in &self.particles {
            if particle.life <= 0.0 {
                continue;
            }
            let position = vec2(
                bounds.x + particle.position.x * bounds.w,
                bounds.y + particle.position.y * bounds.h,
            );
            let mut color = particle.color;
            color.a = particle.life.min(1.0) * 0.85;
            draw_circle(
                position.x,
                position.y,
                particle.size * size_multiplier * (0.5 + particle.life),
                color,
            );
        }
    }

    fn spawn_burst(&mut self, origin: Vec2, color: Color, count: usize) {
        for index in 0..count {
            let angle = index as f32 / count as f32 * std::f32::consts::TAU;
            let speed = 0.075 + (index % 7) as f32 * 0.012;
            self.particles[self.particle_cursor] = Particle {
                position: origin,
                velocity: vec2(angle.cos() * speed, angle.sin() * speed - 0.055),
                life: 0.62 + (index % 4) as f32 * 0.09,
                size: 2.4 + (index % 3) as f32,
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

fn draw_backdrop(bounds: Rect, audio: AudioVisual, elapsed: f32, reduce_motion: bool) {
    let top = Color::from_rgba(5, 7, 28, 255);
    let bottom = Color::from_rgba(44, 10, 72, 255);
    for row in 0..18 {
        let t = row as f32 / 17.0;
        draw_rectangle(
            bounds.x,
            bounds.y + bounds.h * t,
            bounds.w,
            bounds.h / 16.0 + 1.0,
            mix_color(top, bottom, t),
        );
    }

    let pulse = if reduce_motion {
        audio.energy * 0.2
    } else {
        audio.pulse * 0.55 + audio.energy * 0.5
    };
    let sun = vec2(bounds.x + bounds.w * 0.5, bounds.y + bounds.h * 0.235);
    let sun_radius = bounds.w.min(bounds.h) * (0.115 + pulse * 0.012);
    draw_circle(
        sun.x,
        sun.y,
        sun_radius * 1.22,
        Color::new(0.96, 0.18, 0.62, 0.08),
    );
    draw_circle(sun.x, sun.y, sun_radius, Color::new(1.0, 0.36, 0.56, 0.92));
    for stripe in 0..7 {
        let y = sun.y - sun_radius + stripe as f32 * sun_radius * 0.31;
        draw_rectangle(
            sun.x - sun_radius,
            y,
            sun_radius * 2.0,
            4.0 + stripe as f32 * 1.2,
            Color::new(0.18, 0.04, 0.28, 0.62),
        );
    }

    let horizon = bounds.y + bounds.h * 0.31;
    for building in 0..24 {
        let x = bounds.x + bounds.w * building as f32 / 24.0;
        let seed = ((building * 37 + 11) % 13) as f32 / 13.0;
        let width = bounds.w / 22.0;
        let height = bounds.h * (0.045 + seed * 0.12);
        draw_rectangle(
            x,
            horizon - height,
            width,
            height,
            Color::from_rgba(8, 10, 29, 255),
        );
        let window_color = Color::new(0.12, 0.95, 0.96, 0.22 + audio.energy * 0.38);
        for floor in 0..3 {
            draw_rectangle(
                x + width * 0.22,
                horizon - height + 8.0 + floor as f32 * 13.0,
                3.0,
                5.0,
                window_color,
            );
        }
    }

    let star_alpha = 0.18 + audio.energy * 0.42;
    for star in 0..38 {
        let x = bounds.x + ((star * 71) % 997) as f32 / 997.0 * bounds.w;
        let y = bounds.y + ((star * 43 + 19) % 251) as f32 / 251.0 * bounds.h * 0.27;
        let twinkle = ((elapsed * 2.0 + star as f32).sin() * 0.5 + 0.5) * star_alpha;
        draw_circle(
            x,
            y,
            1.0 + (star % 3) as f32 * 0.45,
            Color::new(0.75, 0.95, 1.0, twinkle),
        );
    }
}

fn draw_track(bounds: Rect, game: &RunnerGame, audio: AudioVisual, elapsed: f32) {
    let horizon_y = bounds.y + bounds.h * 0.29;
    let near_y = bounds.y + bounds.h * 0.94;
    let center = bounds.x + bounds.w * 0.5;
    let horizon_half = bounds.w * 0.055;
    let near_half = bounds.w * 0.46;
    let road = Color::new(0.025, 0.035, 0.095, 0.96);
    draw_triangle(
        vec2(center - horizon_half, horizon_y),
        vec2(center + horizon_half, horizon_y),
        vec2(center + near_half, near_y),
        road,
    );
    draw_triangle(
        vec2(center - horizon_half, horizon_y),
        vec2(center - near_half, near_y),
        vec2(center + near_half, near_y),
        road,
    );

    let cyan = Color::new(0.12, 0.92, 1.0, 0.68 + audio.pulse * 0.26);
    let magenta = Color::new(1.0, 0.16, 0.72, 0.62 + audio.energy * 0.3);
    for edge in [-1.0, 1.0] {
        let far = road_point(bounds, edge, 1.0);
        let near = road_point(bounds, edge, 0.0);
        draw_line(
            far.x,
            far.y,
            near.x,
            near.y,
            8.0,
            Color::new(magenta.r, magenta.g, magenta.b, 0.10),
        );
        draw_line(far.x, far.y, near.x, near.y, 2.4, magenta);
    }
    for divider in [-0.333, 0.333] {
        let far = road_point(bounds, divider, 1.0);
        let near = road_point(bounds, divider, 0.0);
        draw_line(
            far.x,
            far.y,
            near.x,
            near.y,
            1.2,
            Color::new(cyan.r, cyan.g, cyan.b, 0.34),
        );
    }

    let scroll = (elapsed * game.speed * 0.72) % 0.1;
    for row in 0..12 {
        let distance = (row as f32 / 11.0 + scroll).fract();
        let left = road_point(bounds, -1.0, distance);
        let right = road_point(bounds, 1.0, distance);
        let alpha = 0.12 + (1.0 - distance) * 0.38;
        draw_line(
            left.x,
            left.y,
            right.x,
            right.y,
            1.0 + (1.0 - distance) * 1.8,
            Color::new(cyan.r, cyan.g, cyan.b, alpha),
        );
    }

    for light in 0..9 {
        let distance = light as f32 / 8.0;
        for side in [-1.16, 1.16] {
            let point = road_point(bounds, side, distance);
            let scale = perspective_scale(distance);
            let height = 8.0 + scale * 34.0;
            draw_line(
                point.x,
                point.y,
                point.x,
                point.y - height,
                2.0 + scale,
                Color::new(0.1, 0.9, 1.0, 0.42),
            );
            draw_circle(
                point.x,
                point.y - height,
                2.0 + scale * 4.0 + audio.pulse * 2.0,
                cyan,
            );
        }
    }
}

fn draw_obstacle(bounds: Rect, obstacle: RunnerObstacle, audio: AudioVisual) {
    let point = road_point(bounds, lane_to_road(obstacle.lane), obstacle.distance);
    let scale = perspective_scale(obstacle.distance);
    let size = 10.0 + scale * 88.0;
    let pulse = 1.0 + audio.pulse * 0.08 + audio.energy * 0.08;
    match obstacle.kind {
        RunnerHazard::LaneBlock => {
            let width = size * 0.72 * pulse;
            let height = size * 0.82 * pulse;
            glow_rect(
                Rect::new(point.x - width * 0.5, point.y - height, width, height),
                Color::from_rgba(251, 67, 127, 255),
            );
            draw_line(
                point.x - width * 0.34,
                point.y - height * 0.64,
                point.x + width * 0.34,
                point.y - height * 0.34,
                3.0,
                WHITE,
            );
            draw_line(
                point.x + width * 0.34,
                point.y - height * 0.64,
                point.x - width * 0.34,
                point.y - height * 0.34,
                3.0,
                WHITE,
            );
        }
        RunnerHazard::Hurdle => {
            let width = size * 0.9;
            let bar_y = point.y - size * 0.34;
            draw_line(
                point.x - width * 0.5,
                point.y,
                point.x - width * 0.5,
                bar_y,
                4.0 + scale * 3.0,
                Color::from_rgba(34, 211, 238, 255),
            );
            draw_line(
                point.x + width * 0.5,
                point.y,
                point.x + width * 0.5,
                bar_y,
                4.0 + scale * 3.0,
                Color::from_rgba(34, 211, 238, 255),
            );
            draw_line(
                point.x - width * 0.58,
                bar_y,
                point.x + width * 0.58,
                bar_y,
                10.0 + scale * 8.0,
                Color::new(0.13, 0.83, 0.93, 0.18),
            );
            draw_line(
                point.x - width * 0.58,
                bar_y,
                point.x + width * 0.58,
                bar_y,
                3.0 + scale * 4.0,
                Color::from_rgba(103, 232, 249, 255),
            );
        }
        RunnerHazard::OverheadGate => {
            let width = size;
            let height = size * 1.15;
            let color = Color::from_rgba(250, 204, 21, 255);
            for x in [point.x - width * 0.5, point.x + width * 0.5] {
                draw_line(
                    x,
                    point.y,
                    x,
                    point.y - height,
                    8.0 + scale * 6.0,
                    Color::new(color.r, color.g, color.b, 0.14),
                );
                draw_line(x, point.y, x, point.y - height, 3.0 + scale * 2.0, color);
            }
            draw_line(
                point.x - width * 0.55,
                point.y - height,
                point.x + width * 0.55,
                point.y - height,
                10.0 + scale * 8.0,
                Color::new(color.r, color.g, color.b, 0.16),
            );
            draw_line(
                point.x - width * 0.55,
                point.y - height,
                point.x + width * 0.55,
                point.y - height,
                4.0 + scale * 3.0,
                color,
            );
        }
        RunnerHazard::BeatOrb => {
            let radius = size * 0.28 * pulse;
            let color = Color::from_rgba(167, 139, 250, 255);
            draw_circle(
                point.x,
                point.y - size * 0.52,
                radius * 1.75,
                Color::new(color.r, color.g, color.b, 0.08),
            );
            draw_circle_lines(
                point.x,
                point.y - size * 0.52,
                radius * 1.3,
                3.0 + scale * 2.0,
                Color::new(color.r, color.g, color.b, 0.56),
            );
            draw_circle(
                point.x,
                point.y - size * 0.52,
                radius,
                Color::new(0.76, 0.44, 1.0, 0.78),
            );
            draw_circle(point.x, point.y - size * 0.52, radius * 0.34, WHITE);
        }
    }
}

fn draw_runner(
    bounds: Rect,
    player: usize,
    game: &RunnerGame,
    pose: Option<PoseFrame>,
    feedback_flash: f32,
    audio: AudioVisual,
) {
    let state = game.players[player];
    let mut anchor = road_point(bounds, lane_to_road(state.lane), 0.025);
    let player_spread = [-0.028, 0.028, -0.072, 0.072][player];
    anchor.x += bounds.w * player_spread;
    let jump_progress = if state.jump_time > 0.0 {
        1.0 - state.jump_time / 0.72
    } else {
        0.0
    };
    if state.jump_time > 0.0 {
        anchor.y -= (jump_progress.clamp(0.0, 1.0) * std::f32::consts::PI).sin() * bounds.h * 0.105;
    }
    let color = player_color(player);
    let body_height = bounds.h * if state.slide_time > 0.0 { 0.105 } else { 0.19 };
    let glow = 0.16 + audio.energy * 0.18 + feedback_flash * 0.25;
    draw_ellipse(
        anchor.x,
        anchor.y + 4.0,
        body_height * 0.34,
        body_height * 0.10,
        0.0,
        Color::new(color.r, color.g, color.b, glow),
    );
    if state.crash_time > 0.0 {
        anchor.x += (state.crash_time * 70.0).sin() * 8.0;
    }
    if let Some(pose) = pose {
        draw_pose_avatar(anchor, body_height, pose, color, feedback_flash);
    } else {
        draw_robot_avatar(
            anchor,
            body_height,
            state.slide_time > 0.0,
            color,
            feedback_flash,
        );
    }
    if state.pulse_time > 0.0 {
        let progress = 1.0 - state.pulse_time / 0.36;
        draw_circle_lines(
            anchor.x,
            anchor.y - body_height * 0.48,
            body_height * (0.32 + progress * 0.9),
            4.0 * (1.0 - progress).max(0.15),
            Color::new(0.9, 0.75, 1.0, 1.0 - progress),
        );
    }
    let label = format!("P{}", player + 1);
    let size = measure_text(&label, None, 14, 1.0);
    draw_round_panel(
        Rect::new(
            anchor.x - size.width * 0.5 - 7.0,
            anchor.y + 9.0,
            size.width + 14.0,
            20.0,
        ),
        Color::new(0.02, 0.03, 0.10, 0.82),
    );
    draw_text(
        &label,
        anchor.x - size.width * 0.5,
        anchor.y + 24.0,
        14.0,
        color,
    );
}

fn draw_pose_avatar(anchor: Vec2, body_height: f32, pose: PoseFrame, color: Color, flash: f32) {
    let hip = average_point(pose, 11, 12);
    let shoulder = average_point(pose, 5, 6);
    let source_height = (hip.y - shoulder.y).abs().max(0.12) * 2.4;
    let scale = body_height / source_height;
    let project = |index: usize| {
        let keypoint = pose.keypoints[index];
        vec2(
            anchor.x + (keypoint.x - hip.x) * scale,
            anchor.y + (keypoint.y - hip.y) * scale,
        )
    };
    for (from, to) in BONES {
        if pose.keypoints[from].confidence < 0.25 || pose.keypoints[to].confidence < 0.25 {
            continue;
        }
        let a = project(from);
        let b = project(to);
        draw_line(
            a.x,
            a.y,
            b.x,
            b.y,
            10.0 + flash * 7.0,
            Color::new(color.r, color.g, color.b, 0.12),
        );
        draw_line(
            a.x,
            a.y,
            b.x,
            b.y,
            3.0 + flash * 2.0,
            Color::new(color.r, color.g, color.b, 0.88),
        );
    }
    let head = project(0);
    draw_circle(
        head.x,
        head.y,
        body_height * 0.075 + flash * 3.0,
        Color::new(color.r, color.g, color.b, 0.92),
    );
    draw_circle(head.x, head.y, body_height * 0.031, WHITE);
}

fn draw_robot_avatar(anchor: Vec2, body_height: f32, sliding: bool, color: Color, flash: f32) {
    let head_y = anchor.y - body_height * if sliding { 0.62 } else { 0.88 };
    let shoulder_y = anchor.y - body_height * if sliding { 0.43 } else { 0.65 };
    let hip_y = anchor.y - body_height * 0.32;
    draw_circle(anchor.x, head_y, body_height * 0.095, color);
    draw_rectangle(
        anchor.x - body_height * 0.03,
        head_y - 2.0,
        body_height * 0.06,
        4.0,
        WHITE,
    );
    draw_line(
        anchor.x,
        shoulder_y,
        anchor.x,
        hip_y,
        7.0 + flash * 3.0,
        color,
    );
    draw_line(
        anchor.x - body_height * 0.22,
        shoulder_y + body_height * 0.10,
        anchor.x,
        shoulder_y,
        5.0,
        color,
    );
    draw_line(
        anchor.x,
        shoulder_y,
        anchor.x + body_height * 0.22,
        shoulder_y + body_height * 0.10,
        5.0,
        color,
    );
    draw_line(
        anchor.x,
        hip_y,
        anchor.x - body_height * 0.16,
        anchor.y,
        6.0,
        color,
    );
    draw_line(
        anchor.x,
        hip_y,
        anchor.x + body_height * 0.16,
        anchor.y,
        6.0,
        color,
    );
}

fn draw_next_cue(bounds: Rect, next: Option<RunnerObstacle>, audio: AudioVisual) {
    let Some(obstacle) = next else {
        return;
    };
    if obstacle.distance > 0.82 {
        return;
    }
    let lane = match obstacle.lane {
        -1 => "LEFT",
        1 => "RIGHT",
        _ => "CENTER",
    };
    let text = format!("{}  /  {}", obstacle.kind.cue(), lane);
    let font_size = 21;
    let size = measure_text(&text, None, font_size, 1.0);
    let width = size.width + 42.0;
    let x = bounds.x + (bounds.w - width) * 0.5;
    let y = bounds.y + 18.0;
    draw_round_panel(
        Rect::new(x, y, width, 42.0),
        Color::new(0.03, 0.04, 0.14, 0.78 + audio.pulse * 0.12),
    );
    draw_text(
        &text,
        x + 21.0,
        y + 28.0,
        font_size as f32,
        hazard_color(obstacle.kind),
    );
}

fn glow_rect(rect: Rect, color: Color) {
    draw_rectangle(
        rect.x - 10.0,
        rect.y - 10.0,
        rect.w + 20.0,
        rect.h + 20.0,
        Color::new(color.r, color.g, color.b, 0.08),
    );
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::new(color.r, color.g, color.b, 0.78),
    );
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 3.0, WHITE);
}

fn road_point(bounds: Rect, lane_position: f32, distance: f32) -> Vec2 {
    let distance = distance.clamp(0.0, 1.0);
    let t = (1.0 - distance).powf(1.42);
    let horizon_y = bounds.y + bounds.h * 0.29;
    let near_y = bounds.y + bounds.h * 0.94;
    let half_width = bounds.w * (0.055 + t * 0.405);
    vec2(
        bounds.x + bounds.w * 0.5 + lane_position * half_width,
        horizon_y + (near_y - horizon_y) * t,
    )
}

fn perspective_scale(distance: f32) -> f32 {
    (1.0 - distance.clamp(0.0, 1.0)).powf(1.35)
}

fn lane_to_road(lane: i8) -> f32 {
    lane as f32 * 0.61
}

fn lane_normalized(lane: i8) -> f32 {
    0.5 + lane as f32 * 0.22
}

fn average_point(pose: PoseFrame, a: usize, b: usize) -> Vec2 {
    vec2(
        (pose.keypoints[a].x + pose.keypoints[b].x) * 0.5,
        (pose.keypoints[a].y + pose.keypoints[b].y) * 0.5,
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

fn hazard_color(kind: RunnerHazard) -> Color {
    match kind {
        RunnerHazard::LaneBlock => Color::from_rgba(251, 113, 133, 255),
        RunnerHazard::Hurdle => Color::from_rgba(103, 232, 249, 255),
        RunnerHazard::OverheadGate => Color::from_rgba(250, 204, 21, 255),
        RunnerHazard::BeatOrb => Color::from_rgba(196, 181, 253, 255),
    }
}

fn feedback_color(feedback: RunnerFeedback) -> Color {
    match feedback {
        RunnerFeedback::Dodge => Color::from_rgba(52, 211, 153, 255),
        RunnerFeedback::Crash => Color::from_rgba(251, 113, 133, 255),
        RunnerFeedback::BeatPickup => Color::from_rgba(196, 181, 253, 255),
        RunnerFeedback::None => WHITE,
    }
}

fn mix_color(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t,
    )
}
