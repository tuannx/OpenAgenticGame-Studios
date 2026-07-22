use brainbreak_core::{
    GameMode, MotionRuntime, PoseFrame, RunnerFeedback, RunnerGame, RunnerHazard, RunnerObstacle,
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
}

const ACTION_CUE_MAX_DISTANCE: f32 = 0.82;
const FEEDBACK_PULSE_SECONDS: f32 = 0.72;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ActionPictogram {
    Dodge,
    Jump,
    Squat,
    Clap,
}

type ScaledSegment = ((f32, f32), (f32, f32));

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ActionCuePresentation {
    verb: &'static str,
    pictogram: ActionPictogram,
}

#[derive(Clone, Copy, Debug)]
struct ActionCueLayout {
    panel: Rect,
    icon_center: Vec2,
    icon_scale: f32,
    text_x: f32,
    verb_baseline: f32,
    verb_font_size: f32,
    lane_baseline: f32,
    lane_font_size: f32,
    lane_slots: Rect,
    proximity_rail: Rect,
}

fn action_cue_presentation(kind: RunnerHazard) -> ActionCuePresentation {
    match kind {
        RunnerHazard::LaneBlock => ActionCuePresentation {
            verb: "DODGE",
            pictogram: ActionPictogram::Dodge,
        },
        RunnerHazard::Hurdle => ActionCuePresentation {
            verb: "JUMP",
            pictogram: ActionPictogram::Jump,
        },
        RunnerHazard::OverheadGate => ActionCuePresentation {
            verb: "SQUAT",
            pictogram: ActionPictogram::Squat,
        },
        RunnerHazard::BeatOrb => ActionCuePresentation {
            verb: "CLAP",
            pictogram: ActionPictogram::Clap,
        },
    }
}

fn action_cue_layout(bounds: Rect) -> ActionCueLayout {
    let compact = bounds.h < 500.0;
    let panel_width = (bounds.w - 24.0).min(if compact { 252.0 } else { 280.0 });
    let panel_height = if compact { 94.0 } else { 112.0 };
    let panel = Rect::new(
        bounds.x + (bounds.w - panel_width) * 0.5,
        bounds.y + if compact { 58.0 } else { 64.0 },
        panel_width,
        panel_height,
    );
    let icon_scale = if compact { 25.0 } else { 31.0 };
    let text_x = panel.x + if compact { 78.0 } else { 88.0 };
    ActionCueLayout {
        panel,
        icon_center: vec2(
            panel.x + if compact { 42.0 } else { 48.0 },
            panel.y + panel.h * 0.48,
        ),
        icon_scale,
        text_x,
        verb_baseline: panel.y + if compact { 35.0 } else { 42.0 },
        verb_font_size: if compact { 25.0 } else { 30.0 },
        lane_baseline: panel.y + if compact { 57.0 } else { 68.0 },
        lane_font_size: if compact { 12.0 } else { 14.0 },
        lane_slots: Rect::new(
            text_x,
            panel.y + if compact { 66.0 } else { 78.0 },
            (panel.x + panel.w - 14.0 - text_x).min(126.0),
            if compact { 13.0 } else { 16.0 },
        ),
        proximity_rail: Rect::new(panel.x + 14.0, panel.y + panel.h - 8.0, panel.w - 28.0, 4.0),
    }
}

fn action_cue_proximity(distance: f32) -> f32 {
    (1.0 - distance / ACTION_CUE_MAX_DISTANCE).clamp(0.0, 1.0)
}

fn action_cue_lane_label(lane: i8) -> &'static str {
    match lane {
        -1 => "LEFT LANE",
        1 => "RIGHT LANE",
        _ => "CENTER LANE",
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FeedbackGlyph {
    Check,
    Cross,
    BeatSpark,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FeedbackPresentation {
    label: &'static str,
    glyph: FeedbackGlyph,
}

fn feedback_presentation(feedback: RunnerFeedback) -> Option<FeedbackPresentation> {
    match feedback {
        RunnerFeedback::None => None,
        RunnerFeedback::Dodge => Some(FeedbackPresentation {
            label: "NICE",
            glyph: FeedbackGlyph::Check,
        }),
        RunnerFeedback::Crash => Some(FeedbackPresentation {
            label: "NEXT",
            glyph: FeedbackGlyph::Cross,
        }),
        RunnerFeedback::BeatPickup => Some(FeedbackPresentation {
            label: "BEAT",
            glyph: FeedbackGlyph::BeatSpark,
        }),
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct FeedbackPulse {
    kind: RunnerFeedback,
    remaining: f32,
}

impl FeedbackPulse {
    fn start(&mut self, kind: RunnerFeedback) {
        self.kind = kind;
        self.remaining = if kind == RunnerFeedback::None {
            0.0
        } else {
            FEEDBACK_PULSE_SECONDS
        };
    }

    fn update(&mut self, dt: f32) {
        self.remaining = (self.remaining - dt.max(0.0)).max(0.0);
        if self.remaining == 0.0 {
            self.kind = RunnerFeedback::None;
        }
    }

    fn intensity(self) -> f32 {
        (self.remaining / FEEDBACK_PULSE_SECONDS).clamp(0.0, 1.0)
    }

    fn is_visible(self) -> bool {
        self.kind != RunnerFeedback::None && self.remaining > 0.0
    }
}

#[derive(Clone, Copy, Debug)]
struct FeedbackMarkerLayout {
    center: Vec2,
    radius: f32,
    player_baseline: f32,
    label_baseline: f32,
    label_font_size: f32,
}

fn feedback_marker_layout(
    bounds: Rect,
    visible_index: usize,
    visible_count: usize,
) -> FeedbackMarkerLayout {
    let count = visible_count.clamp(1, 4);
    let compact = bounds.h < 500.0;
    let radius = if compact { 22.0 } else { 28.0 };
    let gap = if compact { 10.0 } else { 14.0 };
    let total_width = count as f32 * radius * 2.0 + (count - 1) as f32 * gap;
    let start_x = bounds.x + (bounds.w - total_width) * 0.5 + radius;
    let center = vec2(
        start_x + visible_index.min(count - 1) as f32 * (radius * 2.0 + gap),
        bounds.y + bounds.h * if compact { 0.56 } else { 0.66 },
    );
    FeedbackMarkerLayout {
        center,
        radius,
        player_baseline: center.y - radius - 6.0,
        label_baseline: center.y + radius + if compact { 16.0 } else { 19.0 },
        label_font_size: if compact { 14.0 } else { 17.0 },
    }
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
    feedback_pulses: [FeedbackPulse; 4],
    elapsed: f32,
}

impl Default for RunnerStage {
    fn default() -> Self {
        Self {
            particles: std::array::from_fn(|_| Particle::default()),
            particle_cursor: 0,
            feedback_pulses: [FeedbackPulse::default(); 4],
            elapsed: 0.0,
        }
    }
}

fn stage_player_visibility(game: &RunnerGame) -> [bool; 4] {
    let mut visible = game.players.map(|player| player.evaluated);
    visible[0] = true;
    if game.mode == GameMode::DuoGroove {
        visible[1] = true;
    }
    visible
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
            self.feedback_pulses[player].update(dt);
            if game.feedback[player] != RunnerFeedback::None {
                self.feedback_pulses[player].start(game.feedback[player]);
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
        let visible_players = stage_player_visibility(game);
        for player in (0..game.players.len()).rev() {
            if !visible_players[player] {
                continue;
            }
            draw_runner(
                bounds,
                player,
                game,
                motion.players[player].pose,
                self.feedback_pulses[player].intensity(),
                audio,
            );
        }
        self.draw_particles(bounds, audio, reduce_motion);
        if game.phase == brainbreak_core::RunnerPhase::Running {
            self.draw_feedback_markers(bounds, reduce_motion);
            draw_next_cue(bounds, game.next_cue(), audio);
        }
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

    fn draw_feedback_markers(&self, bounds: Rect, reduce_motion: bool) {
        let visible_count = self
            .feedback_pulses
            .iter()
            .filter(|pulse| pulse.is_visible())
            .count();
        let mut visible_index = 0;
        for (player, pulse) in self.feedback_pulses.iter().copied().enumerate() {
            if !pulse.is_visible() {
                continue;
            }
            draw_feedback_marker(
                feedback_marker_layout(bounds, visible_index, visible_count),
                pulse,
                player,
                reduce_motion,
            );
            visible_index += 1;
        }
    }
}

fn draw_feedback_marker(
    layout: FeedbackMarkerLayout,
    pulse: FeedbackPulse,
    player: usize,
    reduce_motion: bool,
) {
    let Some(presentation) = feedback_presentation(pulse.kind) else {
        return;
    };
    let intensity = pulse.intensity();
    let progress = 1.0 - intensity;
    let fade = (intensity / 0.22).min(1.0);
    let scale = if reduce_motion {
        1.0
    } else {
        1.0 + (progress * std::f32::consts::PI).sin() * 0.12
    };
    let radius = layout.radius * scale;
    let outcome_color = color_with_alpha(feedback_color(pulse.kind), fade);
    let identity_color = color_with_alpha(player_color(player), fade * 0.9);

    draw_circle(
        layout.center.x,
        layout.center.y,
        radius * 1.12,
        Color::new(0.015, 0.02, 0.09, fade * 0.86),
    );
    draw_circle_lines(
        layout.center.x,
        layout.center.y,
        radius,
        3.0,
        identity_color,
    );
    draw_feedback_glyph(
        presentation.glyph,
        layout.center,
        radius * 0.72,
        outcome_color,
    );

    let player_label = player_label(player);
    let player_size = measure_text(player_label, None, 12, 1.0);
    draw_text(
        player_label,
        layout.center.x - player_size.width * 0.5,
        layout.player_baseline,
        12.0,
        identity_color,
    );
    let label_size = measure_text(presentation.label, None, layout.label_font_size as u16, 1.0);
    draw_text(
        presentation.label,
        layout.center.x - label_size.width * 0.5,
        layout.label_baseline,
        layout.label_font_size,
        outcome_color,
    );
}

fn draw_feedback_glyph(glyph: FeedbackGlyph, center: Vec2, scale: f32, color: Color) {
    let stroke = (scale * 0.18).max(3.0);
    let segments: &[ScaledSegment] = match glyph {
        FeedbackGlyph::Check => &[
            ((-0.58, 0.02), (-0.16, 0.44)),
            ((-0.16, 0.44), (0.62, -0.46)),
        ],
        FeedbackGlyph::Cross => &[
            ((-0.52, -0.52), (0.52, 0.52)),
            ((0.52, -0.52), (-0.52, 0.52)),
        ],
        FeedbackGlyph::BeatSpark => &[
            ((0.0, -0.58), (0.42, 0.0)),
            ((0.42, 0.0), (0.0, 0.58)),
            ((0.0, 0.58), (-0.42, 0.0)),
            ((-0.42, 0.0), (0.0, -0.58)),
            ((0.0, -0.78), (0.0, -1.0)),
            ((0.0, 0.78), (0.0, 1.0)),
            ((-0.66, 0.0), (-0.92, 0.0)),
            ((0.66, 0.0), (0.92, 0.0)),
        ],
    };
    draw_scaled_segments(center, scale, stroke, color, segments);
}

fn player_label(player: usize) -> &'static str {
    match player {
        0 => "P1",
        1 => "P2",
        2 => "P3",
        _ => "P4",
    }
}

fn color_with_alpha(color: Color, alpha: f32) -> Color {
    Color::new(color.r, color.g, color.b, color.a * alpha.clamp(0.0, 1.0))
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

    // Hand Wrist AR Aura Glow (keypoints 9 & 10)
    for hand_idx in [9, 10] {
        if pose.keypoints[hand_idx].confidence >= 0.25 {
            let wrist = project(hand_idx);
            draw_circle(
                wrist.x,
                wrist.y,
                body_height * 0.08 + flash * 4.0,
                Color::new(color.r, color.g, color.b, 0.45),
            );
            draw_circle(
                wrist.x,
                wrist.y,
                body_height * 0.035,
                Color::from_rgba(255, 255, 255, 230),
            );
        }
    }
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
    if obstacle.distance > ACTION_CUE_MAX_DISTANCE {
        return;
    }

    let presentation = action_cue_presentation(obstacle.kind);
    let layout = action_cue_layout(bounds);
    let color = hazard_color(obstacle.kind);
    let proximity = action_cue_proximity(obstacle.distance);
    let panel_color = Color::new(
        0.02,
        0.025,
        0.11,
        0.88 + audio.pulse * 0.08 + proximity * 0.04,
    );

    draw_round_panel(layout.panel, panel_color);
    draw_rectangle_lines(
        layout.panel.x,
        layout.panel.y,
        layout.panel.w,
        layout.panel.h,
        2.0 + proximity * 2.0,
        Color::new(color.r, color.g, color.b, 0.58 + proximity * 0.36),
    );

    draw_circle(
        layout.icon_center.x,
        layout.icon_center.y,
        layout.icon_scale * 1.18,
        Color::new(color.r, color.g, color.b, 0.09 + audio.pulse * 0.04),
    );
    draw_circle_lines(
        layout.icon_center.x,
        layout.icon_center.y,
        layout.icon_scale * (0.95 + proximity * 0.08),
        2.0 + proximity,
        Color::new(color.r, color.g, color.b, 0.56),
    );
    draw_action_pictogram(
        presentation.pictogram,
        layout.icon_center,
        layout.icon_scale,
        color,
    );

    draw_text(
        presentation.verb,
        layout.text_x,
        layout.verb_baseline,
        layout.verb_font_size,
        color,
    );
    draw_text(
        action_cue_lane_label(obstacle.lane),
        layout.text_x,
        layout.lane_baseline,
        layout.lane_font_size,
        Color::from_rgba(226, 232, 240, 255),
    );
    draw_action_cue_lane_slots(layout.lane_slots, obstacle.lane, color);

    draw_rectangle(
        layout.proximity_rail.x,
        layout.proximity_rail.y,
        layout.proximity_rail.w,
        layout.proximity_rail.h,
        Color::new(1.0, 1.0, 1.0, 0.13),
    );
    draw_rectangle(
        layout.proximity_rail.x,
        layout.proximity_rail.y,
        layout.proximity_rail.w * proximity,
        layout.proximity_rail.h,
        color,
    );
    draw_circle(
        layout.proximity_rail.x + layout.proximity_rail.w * proximity,
        layout.proximity_rail.y + layout.proximity_rail.h * 0.5,
        3.0 + proximity * 2.0,
        WHITE,
    );
}

fn draw_action_cue_lane_slots(bounds: Rect, obstacle_lane: i8, color: Color) {
    let gap = 5.0;
    let slot_width = (bounds.w - gap * 2.0) / 3.0;
    for index in 0..3 {
        let selected = index as i8 - 1 == obstacle_lane;
        let extra_height = if selected { 4.0 } else { 0.0 };
        let slot = Rect::new(
            bounds.x + index as f32 * (slot_width + gap),
            bounds.y - extra_height,
            slot_width,
            bounds.h + extra_height,
        );
        if selected {
            draw_rectangle(
                slot.x,
                slot.y,
                slot.w,
                slot.h,
                Color::new(color.r, color.g, color.b, 0.72),
            );
        }
        draw_rectangle_lines(
            slot.x,
            slot.y,
            slot.w,
            slot.h,
            if selected { 3.0 } else { 1.5 },
            if selected {
                WHITE
            } else {
                Color::new(1.0, 1.0, 1.0, 0.28)
            },
        );
    }
}

fn draw_action_pictogram(pictogram: ActionPictogram, center: Vec2, scale: f32, color: Color) {
    let stroke = (scale * 0.12).max(3.0);
    match pictogram {
        ActionPictogram::Dodge => {
            let barrier = Rect::new(
                center.x - scale * 0.22,
                center.y - scale * 0.40,
                scale * 0.44,
                scale * 0.80,
            );
            draw_rectangle_lines(barrier.x, barrier.y, barrier.w, barrier.h, stroke, color);
            draw_scaled_segments(
                center,
                scale,
                stroke * 0.72,
                color,
                &[
                    ((-0.22, -0.40), (0.22, 0.40)),
                    ((0.22, -0.40), (-0.22, 0.40)),
                ],
            );
            draw_scaled_segments(
                center,
                scale,
                stroke * 0.72,
                WHITE,
                &[
                    ((-0.40, 0.0), (-0.72, 0.0)),
                    ((-0.72, 0.0), (-0.54, -0.16)),
                    ((-0.72, 0.0), (-0.54, 0.16)),
                    ((0.40, 0.0), (0.72, 0.0)),
                    ((0.72, 0.0), (0.54, -0.16)),
                    ((0.72, 0.0), (0.54, 0.16)),
                ],
            );
        }
        ActionPictogram::Jump => {
            draw_circle(center.x, center.y - scale * 0.34, scale * 0.13, WHITE);
            draw_scaled_segments(
                center,
                scale,
                stroke,
                color,
                &[
                    ((0.0, -0.20), (0.0, 0.12)),
                    ((0.0, -0.05), (-0.34, -0.24)),
                    ((0.0, -0.05), (0.34, -0.24)),
                    ((0.0, 0.10), (-0.28, 0.36)),
                    ((0.0, 0.10), (0.28, 0.36)),
                ],
            );
            draw_scaled_segments(
                center,
                scale,
                stroke * 0.72,
                WHITE,
                &[
                    ((0.0, -0.52), (0.0, -0.78)),
                    ((0.0, -0.78), (-0.14, -0.61)),
                    ((0.0, -0.78), (0.14, -0.61)),
                ],
            );
        }
        ActionPictogram::Squat => {
            draw_circle(
                center.x - scale * 0.16,
                center.y - scale * 0.18,
                scale * 0.13,
                WHITE,
            );
            draw_scaled_segments(
                center,
                scale,
                stroke,
                color,
                &[
                    ((-0.06, -0.10), (0.18, 0.08)),
                    ((0.18, 0.08), (-0.02, 0.34)),
                    ((-0.02, 0.34), (-0.34, 0.34)),
                    ((0.16, 0.08), (0.44, 0.30)),
                ],
            );
            draw_scaled_segments(
                center,
                scale,
                stroke * 0.72,
                WHITE,
                &[
                    ((0.0, -0.55), (0.0, -0.34)),
                    ((0.0, -0.34), (-0.14, -0.49)),
                    ((0.0, -0.34), (0.14, -0.49)),
                ],
            );
        }
        ActionPictogram::Clap => {
            let left_hand = vec2(center.x - scale * 0.12, center.y - scale * 0.05);
            let right_hand = vec2(center.x + scale * 0.12, center.y - scale * 0.05);
            draw_circle(left_hand.x, left_hand.y, scale * 0.14, color);
            draw_circle(right_hand.x, right_hand.y, scale * 0.14, color);
            draw_scaled_segments(
                center,
                scale,
                stroke,
                color,
                &[((-0.54, 0.38), (-0.12, 0.03)), ((0.54, 0.38), (0.12, 0.03))],
            );
            draw_scaled_segments(
                center,
                scale,
                stroke * 0.68,
                WHITE,
                &[
                    ((0.0, -0.28), (0.0, -0.58)),
                    ((-0.25, -0.20), (-0.45, -0.42)),
                    ((0.25, -0.20), (0.45, -0.42)),
                ],
            );
        }
    }
}

fn draw_scaled_segments(
    center: Vec2,
    scale: f32,
    stroke: f32,
    color: Color,
    segments: &[ScaledSegment],
) {
    for (from, to) in segments {
        draw_line(
            center.x + from.0 * scale,
            center.y + from.1 * scale,
            center.x + to.0 * scale,
            center.y + to.1 * scale,
            stroke,
            color,
        );
    }
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

#[cfg(test)]
mod action_cue_tests {
    use super::*;

    const VIEWPORTS: [(f32, f32); 3] = [(390.0, 844.0), (667.0, 375.0), (1440.0, 784.0)];

    #[test]
    fn every_hazard_has_a_distinct_shape_and_action_verb() {
        let presentations = [
            action_cue_presentation(RunnerHazard::LaneBlock),
            action_cue_presentation(RunnerHazard::Hurdle),
            action_cue_presentation(RunnerHazard::OverheadGate),
            action_cue_presentation(RunnerHazard::BeatOrb),
        ];

        assert_eq!(presentations[0].verb, "DODGE");
        assert_eq!(presentations[1].verb, "JUMP");
        assert_eq!(presentations[2].verb, "SQUAT");
        assert_eq!(presentations[3].verb, "CLAP");
        for left in 0..presentations.len() {
            for right in left + 1..presentations.len() {
                assert_ne!(
                    presentations[left].pictogram,
                    presentations[right].pictogram
                );
            }
        }
    }

    #[test]
    fn action_cue_stays_inside_camera_distance_viewports() {
        for (width, height) in VIEWPORTS {
            let bounds = Rect::new(0.0, 0.0, width, height);
            let layout = action_cue_layout(bounds);

            assert!(layout.panel.x >= bounds.x);
            assert!(layout.panel.y >= bounds.y + 54.0);
            assert!(layout.panel.x + layout.panel.w <= bounds.x + bounds.w);
            assert!(layout.panel.y + layout.panel.h <= bounds.y + bounds.h);
            assert!(layout.icon_scale >= 25.0);
            assert!(layout.verb_font_size >= 25.0);
            assert!(layout.lane_slots.x >= layout.panel.x);
            assert!(layout.lane_slots.x + layout.lane_slots.w <= layout.panel.x + layout.panel.w);
            assert!(
                layout.proximity_rail.y + layout.proximity_rail.h
                    <= layout.panel.y + layout.panel.h
            );
        }
    }

    #[test]
    fn proximity_is_clamped_and_increases_toward_collision() {
        assert_eq!(action_cue_proximity(ACTION_CUE_MAX_DISTANCE + 1.0), 0.0);
        assert_eq!(action_cue_proximity(ACTION_CUE_MAX_DISTANCE), 0.0);
        assert!(action_cue_proximity(0.4) > action_cue_proximity(0.7));
        assert_eq!(action_cue_proximity(0.0), 1.0);
        assert_eq!(action_cue_proximity(-1.0), 1.0);
    }

    #[test]
    fn lane_labels_describe_obstacle_location_not_move_direction() {
        assert_eq!(action_cue_lane_label(-1), "LEFT LANE");
        assert_eq!(action_cue_lane_label(0), "CENTER LANE");
        assert_eq!(action_cue_lane_label(1), "RIGHT LANE");
    }

    #[test]
    fn feedback_events_have_distinct_shape_and_positive_recovery_copy() {
        let dodge = feedback_presentation(RunnerFeedback::Dodge).expect("dodge feedback");
        let crash = feedback_presentation(RunnerFeedback::Crash).expect("crash feedback");
        let beat = feedback_presentation(RunnerFeedback::BeatPickup).expect("beat feedback");

        assert_eq!(dodge.label, "NICE");
        assert_eq!(crash.label, "NEXT");
        assert_eq!(beat.label, "BEAT");
        assert_ne!(dodge.glyph, crash.glyph);
        assert_ne!(dodge.glyph, beat.glyph);
        assert_ne!(crash.glyph, beat.glyph);
        assert_eq!(feedback_presentation(RunnerFeedback::None), None);
    }

    #[test]
    fn feedback_pulse_retains_one_frame_event_for_a_bounded_window() {
        let mut stage = RunnerStage::default();
        let mut game = RunnerGame::new();
        game.feedback[0] = RunnerFeedback::Dodge;

        stage.update(0.016, &game);
        assert_eq!(stage.feedback_pulses[0].kind, RunnerFeedback::Dodge);
        assert_eq!(stage.feedback_pulses[0].remaining, FEEDBACK_PULSE_SECONDS);

        game.feedback[0] = RunnerFeedback::None;
        stage.update(FEEDBACK_PULSE_SECONDS * 0.5, &game);
        assert!(stage.feedback_pulses[0].is_visible());
        assert!((stage.feedback_pulses[0].intensity() - 0.5).abs() < 0.001);

        stage.update(FEEDBACK_PULSE_SECONDS, &game);
        assert!(!stage.feedback_pulses[0].is_visible());
        assert_eq!(stage.feedback_pulses[0].kind, RunnerFeedback::None);
    }

    #[test]
    fn four_feedback_markers_fit_between_action_beacon_and_viewport_edges() {
        for (width, height) in VIEWPORTS {
            let bounds = Rect::new(0.0, 0.0, width, height);
            let cue = action_cue_layout(bounds);
            for count in 1..=4 {
                let mut previous_right = bounds.x;
                for index in 0..count {
                    let marker = feedback_marker_layout(bounds, index, count);
                    let left = marker.center.x - marker.radius;
                    let right = marker.center.x + marker.radius;
                    assert!(left >= bounds.x);
                    assert!(right <= bounds.x + bounds.w);
                    assert!(left >= previous_right);
                    assert!(marker.center.y - marker.radius - 18.0 >= cue.panel.y + cue.panel.h);
                    assert!(marker.label_baseline <= bounds.y + bounds.h);
                    previous_right = right;
                }
            }
        }
    }

    #[test]
    fn single_modes_show_only_truthful_player_slots() {
        let mut game = RunnerGame::new();
        assert_eq!(stage_player_visibility(&game), [true, false, false, false]);

        game.players[1].evaluated = true;
        game.players[3].evaluated = true;
        assert_eq!(stage_player_visibility(&game), [true, true, false, true]);

        game.mode = GameMode::BeatStrike;
        assert_eq!(stage_player_visibility(&game), [true, true, false, true]);
    }

    #[test]
    fn duo_keeps_two_required_slots_and_truthful_remote_presence() {
        let mut game = RunnerGame::with_mode(GameMode::DuoGroove);
        assert_eq!(stage_player_visibility(&game), [true, true, false, false]);

        game.players[2].evaluated = true;
        assert_eq!(stage_player_visibility(&game), [true, true, true, false]);
    }
}
