use brainbreak_core::{GameMode, PLAYER_CAPACITY, RunnerGame, RunnerOutcome, RunnerPhase};
use macroquad::prelude::*;

use crate::juice::hero_edge;
use crate::platform::{ModeArt, player_color};
use crate::visuals::draw_round_panel;

// --- Motion Prompt Types ---

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MotionPromptTone {
    Warning,
    Active,
    Ready,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MotionFigure {
    Hidden,
    Camera,
    Searching,
    Present,
    Ready,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MotionPromptPresentation {
    pub title: &'static str,
    pub instruction: &'static str,
    pub tone: MotionPromptTone,
    pub figures: [MotionFigure; 2],
}

#[derive(Clone, Copy, Debug)]
pub struct MotionPromptLayout {
    pub panel: Rect,
    pub art: Rect,
    pub content: Rect,
    pub figure_centers: [Vec2; 2],
    pub figure_scale: f32,
    pub title_font: u16,
    pub instruction_font: u16,
}

// --- Presentation Logic ---

pub fn ready_presentation(
    mode: GameMode,
    guide_only: bool,
    tracked_players: usize,
    p1_ready: bool,
    p2_ready: bool,
) -> MotionPromptPresentation {
    if guide_only {
        return MotionPromptPresentation {
            title: "CAMERA NEEDED",
            instruction: "TURN ON MOTION TO PLAY",
            tone: MotionPromptTone::Warning,
            figures: [MotionFigure::Camera, MotionFigure::Hidden],
        };
    }
    if tracked_players == 0 {
        return MotionPromptPresentation {
            title: "STEP INTO FRAME",
            instruction: "SHOW YOUR FULL BODY",
            tone: MotionPromptTone::Active,
            figures: [MotionFigure::Searching, MotionFigure::Hidden],
        };
    }
    if mode == GameMode::DuoGroove && tracked_players < 2 {
        return MotionPromptPresentation {
            title: if p1_ready {
                "PLAYER 1 READY"
            } else {
                "PLAYER 1 FOUND"
            },
            instruction: "PLAYER 2 STEP INTO FRAME",
            tone: MotionPromptTone::Warning,
            figures: [
                if p1_ready {
                    MotionFigure::Ready
                } else {
                    MotionFigure::Present
                },
                MotionFigure::Searching,
            ],
        };
    }
    if mode == GameMode::DuoGroove {
        return match (p1_ready, p2_ready) {
            (true, true) => MotionPromptPresentation {
                title: "BOTH PLAYERS READY",
                instruction: "GET READY",
                tone: MotionPromptTone::Ready,
                figures: [MotionFigure::Ready, MotionFigure::Ready],
            },
            (true, false) => MotionPromptPresentation {
                title: "PLAYER 1 READY",
                instruction: "PLAYER 2 RAISE A HAND",
                tone: MotionPromptTone::Active,
                figures: [MotionFigure::Ready, MotionFigure::Present],
            },
            (false, true) => MotionPromptPresentation {
                title: "PLAYER 2 READY",
                instruction: "PLAYER 1 RAISE A HAND",
                tone: MotionPromptTone::Active,
                figures: [MotionFigure::Present, MotionFigure::Ready],
            },
            (false, false) => MotionPromptPresentation {
                title: "DUO READY CHECK",
                instruction: "BOTH RAISE A HAND",
                tone: MotionPromptTone::Active,
                figures: [MotionFigure::Present, MotionFigure::Present],
            },
        };
    }

    if p1_ready || p2_ready {
        MotionPromptPresentation {
            title: "SIGNAL RECEIVED",
            instruction: "GET READY",
            tone: MotionPromptTone::Ready,
            figures: [MotionFigure::Ready, MotionFigure::Hidden],
        }
    } else {
        MotionPromptPresentation {
            title: "READY TO MOVE",
            instruction: "RAISE A HAND OR CLAP",
            tone: MotionPromptTone::Active,
            figures: [MotionFigure::Present, MotionFigure::Hidden],
        }
    }
}

pub fn tracking_hold_presentation(
    mode: GameMode,
    evaluated: [bool; PLAYER_CAPACITY],
    ready: [bool; PLAYER_CAPACITY],
) -> MotionPromptPresentation {
    if mode == GameMode::DuoGroove {
        return match (evaluated[0], evaluated[1]) {
            (false, false) => MotionPromptPresentation {
                title: "FIND BOTH PLAYERS",
                instruction: "STEP BACK INTO VIEW",
                tone: MotionPromptTone::Warning,
                figures: [MotionFigure::Searching, MotionFigure::Searching],
            },
            (true, false) => MotionPromptPresentation {
                title: if ready[0] {
                    "PLAYER 1 READY"
                } else {
                    "PLAYER 1 FOUND"
                },
                instruction: "PLAYER 2 STEP INTO VIEW",
                tone: MotionPromptTone::Warning,
                figures: [
                    if ready[0] {
                        MotionFigure::Ready
                    } else {
                        MotionFigure::Present
                    },
                    MotionFigure::Searching,
                ],
            },
            (false, true) => MotionPromptPresentation {
                title: if ready[1] {
                    "PLAYER 2 READY"
                } else {
                    "PLAYER 2 FOUND"
                },
                instruction: "PLAYER 1 STEP INTO VIEW",
                tone: MotionPromptTone::Warning,
                figures: [
                    MotionFigure::Searching,
                    if ready[1] {
                        MotionFigure::Ready
                    } else {
                        MotionFigure::Present
                    },
                ],
            },
            (true, true) => match (ready[0], ready[1]) {
                (true, true) => MotionPromptPresentation {
                    title: "BOTH PLAYERS READY",
                    instruction: "GET READY",
                    tone: MotionPromptTone::Ready,
                    figures: [MotionFigure::Ready, MotionFigure::Ready],
                },
                (true, false) => MotionPromptPresentation {
                    title: "PLAYER 1 READY",
                    instruction: "PLAYER 2 RAISE A HAND",
                    tone: MotionPromptTone::Active,
                    figures: [MotionFigure::Ready, MotionFigure::Present],
                },
                (false, true) => MotionPromptPresentation {
                    title: "PLAYER 2 READY",
                    instruction: "PLAYER 1 RAISE A HAND",
                    tone: MotionPromptTone::Active,
                    figures: [MotionFigure::Present, MotionFigure::Ready],
                },
                (false, false) => MotionPromptPresentation {
                    title: "TRACKING RESTORED",
                    instruction: "BOTH RAISE A HAND",
                    tone: MotionPromptTone::Active,
                    figures: [MotionFigure::Present, MotionFigure::Present],
                },
            },
        };
    }

    let has_evaluated_player = evaluated.into_iter().any(|value| value);
    let has_ready_player = evaluated
        .into_iter()
        .zip(ready)
        .any(|(is_evaluated, is_ready)| is_evaluated && is_ready);
    if !has_evaluated_player {
        MotionPromptPresentation {
            title: "FIND YOUR FRAME",
            instruction: "STEP BACK INTO VIEW",
            tone: MotionPromptTone::Warning,
            figures: [MotionFigure::Searching, MotionFigure::Hidden],
        }
    } else if has_ready_player {
        MotionPromptPresentation {
            title: "SIGNAL RECEIVED",
            instruction: "GET READY",
            tone: MotionPromptTone::Ready,
            figures: [MotionFigure::Ready, MotionFigure::Hidden],
        }
    } else {
        MotionPromptPresentation {
            title: "TRACKING RESTORED",
            instruction: "RAISE A HAND OR CLAP",
            tone: MotionPromptTone::Active,
            figures: [MotionFigure::Present, MotionFigure::Hidden],
        }
    }
}

pub fn pause_presentation(
    mode: GameMode,
    evaluated: [bool; PLAYER_CAPACITY],
    ready: [bool; PLAYER_CAPACITY],
) -> MotionPromptPresentation {
    if mode == GameMode::DuoGroove {
        return match (evaluated[0], evaluated[1]) {
            (false, false) => MotionPromptPresentation {
                title: "PAUSED",
                instruction: "BOTH STEP BACK INTO FRAME",
                tone: MotionPromptTone::Warning,
                figures: [MotionFigure::Searching, MotionFigure::Searching],
            },
            (true, false) => MotionPromptPresentation {
                title: "PAUSED",
                instruction: "PLAYER 2 BACK IN FRAME",
                tone: MotionPromptTone::Warning,
                figures: [
                    if ready[0] {
                        MotionFigure::Ready
                    } else {
                        MotionFigure::Present
                    },
                    MotionFigure::Searching,
                ],
            },
            (false, true) => MotionPromptPresentation {
                title: "PAUSED",
                instruction: "PLAYER 1 BACK IN FRAME",
                tone: MotionPromptTone::Warning,
                figures: [
                    MotionFigure::Searching,
                    if ready[1] {
                        MotionFigure::Ready
                    } else {
                        MotionFigure::Present
                    },
                ],
            },
            (true, true) => match (ready[0], ready[1]) {
                (true, true) => MotionPromptPresentation {
                    title: "PAUSED",
                    instruction: "GET READY",
                    tone: MotionPromptTone::Ready,
                    figures: [MotionFigure::Ready, MotionFigure::Ready],
                },
                (true, false) => MotionPromptPresentation {
                    title: "PAUSED",
                    instruction: "PLAYER 2 CLAP TO RESUME",
                    tone: MotionPromptTone::Active,
                    figures: [MotionFigure::Ready, MotionFigure::Present],
                },
                (false, true) => MotionPromptPresentation {
                    title: "PAUSED",
                    instruction: "PLAYER 1 CLAP TO RESUME",
                    tone: MotionPromptTone::Active,
                    figures: [MotionFigure::Present, MotionFigure::Ready],
                },
                (false, false) => MotionPromptPresentation {
                    title: "PAUSED",
                    instruction: "BOTH CLAP TO RESUME",
                    tone: MotionPromptTone::Active,
                    figures: [MotionFigure::Present, MotionFigure::Present],
                },
            },
        };
    }

    let has_evaluated_player = evaluated.into_iter().any(|value| value);
    let has_ready_player = evaluated
        .into_iter()
        .zip(ready)
        .any(|(is_evaluated, is_ready)| is_evaluated && is_ready);
    if !has_evaluated_player {
        MotionPromptPresentation {
            title: "PAUSED",
            instruction: "STEP BACK INTO FRAME",
            tone: MotionPromptTone::Warning,
            figures: [MotionFigure::Searching, MotionFigure::Hidden],
        }
    } else if has_ready_player {
        MotionPromptPresentation {
            title: "PAUSED",
            instruction: "GET READY",
            tone: MotionPromptTone::Ready,
            figures: [MotionFigure::Ready, MotionFigure::Hidden],
        }
    } else {
        MotionPromptPresentation {
            title: "PAUSED",
            instruction: "CLAP TO RESUME",
            tone: MotionPromptTone::Active,
            figures: [MotionFigure::Present, MotionFigure::Hidden],
        }
    }
}

// --- Motion Prompt Layout & Drawing ---

pub fn motion_prompt_layout(width: f32, height: f32) -> MotionPromptLayout {
    let compact = height < 500.0;
    let panel_width = (width - 24.0).min(600.0);
    let panel_height = if compact { 168.0 } else { 188.0 };
    let panel = Rect::new(
        (width - panel_width) * 0.5,
        height * 0.48 - panel_height * 0.5,
        panel_width,
        panel_height,
    );
    let art_width = (panel.w * 0.30).clamp(96.0, 146.0);
    let art = Rect::new(panel.x + 10.0, panel.y + 10.0, art_width, panel.h - 20.0);
    let content = Rect::new(
        art.x + art.w + 14.0,
        panel.y + 10.0,
        panel.x + panel.w - (art.x + art.w + 14.0) - 12.0,
        panel.h - 20.0,
    );
    let figure_scale = if compact { 23.0 } else { 27.0 };
    let figure_y = content.y + content.h - figure_scale * 0.95;
    MotionPromptLayout {
        panel,
        art,
        content,
        figure_centers: [
            vec2(content.x + figure_scale, figure_y),
            vec2(content.x + figure_scale * 3.3, figure_y),
        ],
        figure_scale,
        title_font: if content.w < 260.0 { 20 } else { 25 },
        instruction_font: if content.w < 260.0 { 13 } else { 15 },
    }
}

pub fn motion_figure_bounds(center: Vec2, scale: f32) -> Rect {
    Rect::new(
        center.x - scale * 0.9,
        center.y - scale * 1.05,
        scale * 1.8,
        scale * 1.95,
    )
}

pub fn motion_prompt_color(tone: MotionPromptTone) -> Color {
    match tone {
        MotionPromptTone::Warning => Color::from_rgba(251, 191, 36, 255),
        MotionPromptTone::Active => Color::from_rgba(103, 232, 249, 255),
        MotionPromptTone::Ready => Color::from_rgba(52, 211, 153, 255),
    }
}

fn mode_label(mode: GameMode) -> &'static str {
    match mode {
        GameMode::MirrorBeat => "MIRROR",
        GameMode::BeatStrike => "STRIKE",
        GameMode::DuoGroove => "DUO",
    }
}

fn draw_mode_prompt_art(bounds: Rect, mode: GameMode, mode_art: &ModeArt) {
    draw_round_panel(bounds, Color::new(0.04, 0.07, 0.17, 1.0));
    if let Some(texture) = mode_art.texture(mode) {
        draw_texture_ex(
            texture,
            bounds.x,
            bounds.y,
            Color::new(0.82, 0.88, 1.0, 0.92),
            DrawTextureParams {
                dest_size: Some(vec2(bounds.w, bounds.h)),
                source: Some(cover_source_rect(texture.width(), texture.height(), bounds)),
                ..Default::default()
            },
        );
        draw_rectangle(
            bounds.x,
            bounds.y,
            bounds.w,
            bounds.h,
            Color::new(0.02, 0.03, 0.12, 0.18),
        );
    } else {
        let accent = Color::new(0.40, 0.88, 1.0, 0.58);
        draw_circle(
            bounds.x + bounds.w * 0.5,
            bounds.y + bounds.h * 0.42,
            bounds.w.min(bounds.h) * 0.28,
            Color::new(0.45, 0.32, 0.95, 0.26),
        );
        for offset in [-0.18_f32, 0.0, 0.18] {
            draw_line(
                bounds.x + bounds.w * (0.18 + offset),
                bounds.y + bounds.h * 0.68,
                bounds.x + bounds.w * (0.64 + offset),
                bounds.y + bounds.h * 0.22,
                3.0,
                accent,
            );
        }
    }
    draw_rectangle(
        bounds.x,
        bounds.y + bounds.h - 28.0,
        bounds.w,
        28.0,
        Color::new(0.01, 0.02, 0.08, 0.78),
    );
    draw_rectangle_lines(
        bounds.x,
        bounds.y,
        bounds.w,
        bounds.h,
        2.0,
        Color::new(0.40, 0.88, 1.0, 0.58),
    );
    let _ = mode_label(mode);
}

fn draw_motion_camera(center: Vec2, scale: f32, color: Color) {
    let body = Rect::new(
        center.x - scale * 0.62,
        center.y - scale * 0.42,
        scale * 1.05,
        scale * 0.78,
    );
    draw_rectangle_lines(body.x, body.y, body.w, body.h, 2.5, color);
    draw_poly(
        body.x + body.w + scale * 0.22,
        body.y + body.h * 0.5,
        3,
        scale * 0.32,
        90.0,
        color,
    );
    draw_circle_lines(
        body.x + body.w * 0.52,
        body.y + body.h * 0.5,
        scale * 0.19,
        2.0,
        color,
    );
    draw_line(
        center.x - scale * 0.82,
        center.y - scale * 0.78,
        center.x + scale * 0.82,
        center.y + scale * 0.78,
        3.0,
        color,
    );
}

fn draw_motion_figure(center: Vec2, scale: f32, state: MotionFigure, accent: Color) {
    if state == MotionFigure::Hidden {
        return;
    }
    if state == MotionFigure::Camera {
        draw_motion_camera(center, scale, accent);
        return;
    }

    let muted = Color::from_rgba(148, 163, 184, 230);
    let color = if state == MotionFigure::Searching {
        muted
    } else {
        accent
    };
    let head = vec2(center.x, center.y - scale * 0.62);
    let shoulder = vec2(center.x, center.y - scale * 0.24);
    let hip = vec2(center.x, center.y + scale * 0.34);
    draw_circle_lines(head.x, head.y, scale * 0.18, 2.5, color);
    draw_line(shoulder.x, shoulder.y, hip.x, hip.y, 3.0, color);
    draw_line(
        hip.x,
        hip.y,
        center.x - scale * 0.32,
        center.y + scale * 0.82,
        3.0,
        color,
    );
    draw_line(
        hip.x,
        hip.y,
        center.x + scale * 0.32,
        center.y + scale * 0.82,
        3.0,
        color,
    );
    if state == MotionFigure::Ready {
        draw_line(
            shoulder.x,
            shoulder.y,
            center.x - scale * 0.42,
            center.y + scale * 0.08,
            3.0,
            color,
        );
        draw_line(
            shoulder.x,
            shoulder.y,
            center.x + scale * 0.42,
            center.y - scale * 0.82,
            3.0,
            color,
        );
        draw_poly(
            center.x + scale * 0.42,
            center.y - scale * 1.02,
            4,
            scale * 0.13,
            45.0,
            color,
        );
    } else {
        draw_line(
            shoulder.x,
            shoulder.y,
            center.x - scale * 0.46,
            center.y + scale * 0.12,
            3.0,
            color,
        );
        draw_line(
            shoulder.x,
            shoulder.y,
            center.x + scale * 0.46,
            center.y + scale * 0.12,
            3.0,
            color,
        );
    }
    if state == MotionFigure::Searching {
        let frame = motion_figure_bounds(center, scale);
        let corner = scale * 0.22;
        for (x, x_sign) in [(frame.x, 1.0_f32), (frame.x + frame.w, -1.0)] {
            draw_line(x, frame.y, x + corner * x_sign, frame.y, 2.0, color);
            draw_line(
                x,
                frame.y + frame.h,
                x + corner * x_sign,
                frame.y + frame.h,
                2.0,
                color,
            );
        }
        for (y, y_sign) in [(frame.y, 1.0_f32), (frame.y + frame.h, -1.0)] {
            draw_line(frame.x, y, frame.x, y + corner * y_sign, 2.0, color);
            draw_line(
                frame.x + frame.w,
                y,
                frame.x + frame.w,
                y + corner * y_sign,
                2.0,
                color,
            );
        }
    }
}

fn draw_motion_prompt_overlay(
    width: f32,
    height: f32,
    mode: GameMode,
    presentation: MotionPromptPresentation,
    mode_art: &ModeArt,
) {
    let layout = motion_prompt_layout(width, height);
    let accent = motion_prompt_color(presentation.tone);
    draw_round_panel(layout.panel, Color::new(0.015, 0.02, 0.09, 0.94));
    draw_rectangle_lines(
        layout.panel.x,
        layout.panel.y,
        layout.panel.w,
        layout.panel.h,
        3.0,
        accent,
    );
    draw_mode_prompt_art(layout.art, mode, mode_art);
    // Title/instruction words removed — VO + motion figures own guidance.
    let figure_scale = layout.figure_scale * 1.85;
    for (index, figure) in presentation.figures.into_iter().enumerate() {
        draw_motion_figure(
            layout.figure_centers[index],
            figure_scale,
            figure,
            accent,
        );
    }
}

fn draw_ready_overlay(
    width: f32,
    height: f32,
    runner: &RunnerGame,
    guide_only: bool,
    tracked_players: usize,
    mode_art: &ModeArt,
) {
    let presentation = ready_presentation(
        runner.mode,
        guide_only,
        tracked_players,
        runner.ready_players[0],
        runner.ready_players[1],
    );
    draw_motion_prompt_overlay(width, height, runner.mode, presentation, mode_art);
}

fn draw_tracking_hold_overlay(width: f32, height: f32, runner: &RunnerGame, mode_art: &ModeArt) {
    let presentation = tracking_hold_presentation(
        runner.mode,
        runner.players.map(|player| player.evaluated),
        runner.ready_players,
    );
    draw_motion_prompt_overlay(width, height, runner.mode, presentation, mode_art);
}

pub fn draw_phase_overlay(
    width: f32,
    height: f32,
    runner: &RunnerGame,
    guide_only: bool,
    tracked_players: usize,
    mode_art: &ModeArt,
) {
    if runner.phase == RunnerPhase::Running && !guide_only {
        return;
    }
    if runner.phase == RunnerPhase::GameOver {
        draw_result_overlay(width, height, runner, mode_art);
        return;
    }
    if runner.phase == RunnerPhase::Paused {
        draw_pause_overlay(width, height, runner);
        return;
    }
    if runner.phase == RunnerPhase::TrackingHold {
        draw_tracking_hold_overlay(width, height, runner, mode_art);
        return;
    }
    if matches!(
        runner.phase,
        RunnerPhase::Starting | RunnerPhase::PauseResuming | RunnerPhase::Resuming
    ) {
        draw_countdown_overlay(width, height, runner.countdown_remaining);
        return;
    }
    draw_ready_overlay(width, height, runner, guide_only, tracked_players, mode_art);
}

// --- Countdown ---

pub const COUNTDOWN_TICK_COUNT: usize = 12;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CountdownNumeral {
    Two,
    One,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CountdownPresentation {
    pub numeral: CountdownNumeral,
    pub second_fraction: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CountdownStroke {
    pub from: Vec2,
    pub to: Vec2,
}

#[derive(Clone, Copy, Debug)]
pub struct CountdownOverlayLayout {
    pub panel: Rect,
    pub title_y: f32,
    pub ring_center: Vec2,
    pub ring_radius: f32,
    pub stroke_scale: f32,
}

pub fn countdown_presentation(remaining_seconds: f32) -> CountdownPresentation {
    let bounded = if remaining_seconds.is_finite() {
        remaining_seconds.max(0.0)
    } else {
        0.0
    };
    if bounded > 1.0 {
        CountdownPresentation {
            numeral: CountdownNumeral::Two,
            second_fraction: (bounded - 1.0).clamp(0.0, 1.0),
        }
    } else {
        CountdownPresentation {
            numeral: CountdownNumeral::One,
            second_fraction: bounded.clamp(0.0, 1.0),
        }
    }
}

pub fn countdown_active_ticks(second_fraction: f32) -> usize {
    if !second_fraction.is_finite() {
        return 0;
    }
    (second_fraction.clamp(0.0, 1.0) * COUNTDOWN_TICK_COUNT as f32)
        .ceil()
        .min(COUNTDOWN_TICK_COUNT as f32) as usize
}

pub fn countdown_strokes(numeral: CountdownNumeral) -> ([CountdownStroke; 5], usize) {
    let empty = CountdownStroke {
        from: Vec2::ZERO,
        to: Vec2::ZERO,
    };
    match numeral {
        CountdownNumeral::Two => (
            [
                CountdownStroke {
                    from: vec2(-0.38, -0.45),
                    to: vec2(0.38, -0.45),
                },
                CountdownStroke {
                    from: vec2(0.38, -0.45),
                    to: vec2(0.38, -0.05),
                },
                CountdownStroke {
                    from: vec2(0.38, -0.05),
                    to: vec2(-0.38, -0.05),
                },
                CountdownStroke {
                    from: vec2(-0.38, -0.05),
                    to: vec2(-0.38, 0.42),
                },
                CountdownStroke {
                    from: vec2(-0.38, 0.42),
                    to: vec2(0.38, 0.42),
                },
            ],
            5,
        ),
        CountdownNumeral::One => (
            [
                CountdownStroke {
                    from: vec2(-0.16, -0.30),
                    to: vec2(0.0, -0.45),
                },
                CountdownStroke {
                    from: vec2(0.0, -0.45),
                    to: vec2(0.0, 0.42),
                },
                CountdownStroke {
                    from: vec2(-0.22, 0.42),
                    to: vec2(0.22, 0.42),
                },
                empty,
                empty,
            ],
            3,
        ),
    }
}

pub fn countdown_overlay_layout(width: f32, height: f32) -> CountdownOverlayLayout {
    let compact = height < 500.0;
    let panel_width = (width - 24.0).min(480.0);
    let panel_height = if compact { 210.0 } else { 230.0 };
    let panel = Rect::new(
        (width - panel_width) * 0.5,
        height * 0.48 - panel_height * 0.5,
        panel_width,
        panel_height,
    );
    let ring_radius = if compact { 50.0 } else { 56.0 };
    CountdownOverlayLayout {
        panel,
        title_y: panel.y + if compact { 42.0 } else { 46.0 },
        ring_center: vec2(
            panel.x + panel.w * 0.5,
            panel.y + if compact { 140.0 } else { 154.0 },
        ),
        ring_radius,
        stroke_scale: ring_radius * 1.24,
    }
}

fn draw_countdown_stroke(center: Vec2, scale: f32, stroke: CountdownStroke) {
    let from = center + stroke.from * scale;
    let to = center + stroke.to * scale;
    let glow = Color::new(0.40, 0.88, 1.0, 0.30);
    draw_line(from.x, from.y, to.x, to.y, 12.0, glow);
    draw_circle(from.x, from.y, 6.0, glow);
    draw_circle(to.x, to.y, 6.0, glow);
    draw_line(from.x, from.y, to.x, to.y, 6.0, WHITE);
    draw_circle(from.x, from.y, 3.0, WHITE);
    draw_circle(to.x, to.y, 3.0, WHITE);
}

fn draw_countdown_overlay(width: f32, height: f32, remaining_seconds: f32) {
    let presentation = countdown_presentation(remaining_seconds);
    let layout = countdown_overlay_layout(width, height);
    let accent = Color::from_rgba(103, 232, 249, 255);
    draw_round_panel(layout.panel, Color::new(0.015, 0.02, 0.09, 0.94));
    draw_rectangle_lines(
        layout.panel.x,
        layout.panel.y,
        layout.panel.w,
        layout.panel.h,
        3.0,
        accent,
    );
    let title = "GET READY";
    let _ = title;
    // Shape-only countdown — VO speaks Three/Two/One/Go.

    draw_circle_lines(
        layout.ring_center.x,
        layout.ring_center.y,
        layout.ring_radius - 7.0,
        2.0,
        Color::new(0.40, 0.88, 1.0, 0.22),
    );
    let active_ticks = countdown_active_ticks(presentation.second_fraction);
    for index in 0..COUNTDOWN_TICK_COUNT {
        let angle = -std::f32::consts::FRAC_PI_2
            + index as f32 * std::f32::consts::TAU / COUNTDOWN_TICK_COUNT as f32;
        let direction = vec2(angle.cos(), angle.sin());
        let from = layout.ring_center + direction * layout.ring_radius;
        let to = layout.ring_center + direction * (layout.ring_radius + 8.0);
        let active = index < active_ticks;
        draw_line(
            from.x,
            from.y,
            to.x,
            to.y,
            if active { 3.5 } else { 2.0 },
            if active {
                accent
            } else {
                Color::from_rgba(100, 116, 139, 120)
            },
        );
    }

    let (strokes, stroke_count) = countdown_strokes(presentation.numeral);
    for stroke in strokes.into_iter().take(stroke_count) {
        draw_countdown_stroke(layout.ring_center, layout.stroke_scale, stroke);
    }
}

// --- Pause Overlay ---

#[derive(Clone, Copy, Debug)]
pub struct PauseOverlayLayout {
    pub panel: Rect,
    pub left_bar: Rect,
    pub right_bar: Rect,
    pub title_y: f32,
    pub instruction_y: f32,
    pub single_figure_center: Vec2,
    pub duo_figure_centers: [Vec2; 2],
    pub figure_scale: f32,
}

pub fn pause_overlay_layout(width: f32, height: f32) -> PauseOverlayLayout {
    let compact = height < 500.0;
    let panel_width = (width - 24.0).min(520.0);
    let panel_height = if compact { 232.0 } else { 244.0 };
    let panel = Rect::new(
        (width - panel_width) * 0.5,
        height * 0.48 - panel_height * 0.5,
        panel_width,
        panel_height,
    );
    let bar_width = 19.0;
    let bar_height = 48.0;
    let bar_gap = 16.0;
    let bars_start_x = panel.x + (panel.w - bar_width * 2.0 - bar_gap) * 0.5;
    let bars_y = panel.y + 18.0;
    let figure_scale = if compact { 20.0 } else { 22.0 };
    let figure_y = panel.y + panel.h - 12.0 - figure_scale * 0.9;
    let center_x = panel.x + panel.w * 0.5;
    PauseOverlayLayout {
        panel,
        left_bar: Rect::new(bars_start_x, bars_y, bar_width, bar_height),
        right_bar: Rect::new(
            bars_start_x + bar_width + bar_gap,
            bars_y,
            bar_width,
            bar_height,
        ),
        title_y: panel.y + 106.0,
        instruction_y: panel.y + 140.0,
        single_figure_center: vec2(center_x, figure_y),
        duo_figure_centers: [
            vec2(center_x - figure_scale * 1.4, figure_y),
            vec2(center_x + figure_scale * 1.4, figure_y),
        ],
        figure_scale,
    }
}

fn draw_pause_overlay(width: f32, height: f32, runner: &RunnerGame) {
    let layout = pause_overlay_layout(width, height);
    let presentation = pause_presentation(
        runner.mode,
        runner.players.map(|player| player.evaluated),
        runner.ready_players,
    );
    let accent = motion_prompt_color(presentation.tone);
    draw_round_panel(layout.panel, Color::new(0.015, 0.02, 0.09, 0.94));
    draw_rectangle_lines(
        layout.panel.x,
        layout.panel.y,
        layout.panel.w,
        layout.panel.h,
        3.0,
        accent,
    );
    for (bar, color) in [
        (layout.left_bar, player_color(0)),
        (layout.right_bar, Color::from_rgba(196, 181, 253, 255)),
    ] {
        draw_rectangle(
            bar.x,
            bar.y,
            bar.w,
            bar.h,
            Color::new(0.03, 0.12, 0.24, 0.92),
        );
        draw_rectangle_lines(bar.x, bar.y, bar.w, bar.h, 4.0, color);
    }

    let title_font = if width < 520.0 { 31 } else { 36 };
    let _ = (presentation.title, title_font);
    // Pause bars + body figures only — clap/return guidance is spoken.

    if presentation.figures[1] == MotionFigure::Hidden {
        draw_motion_figure(
            layout.single_figure_center,
            layout.figure_scale * 2.2,
            presentation.figures[0],
            accent,
        );
    } else {
        for (index, figure) in presentation.figures.into_iter().enumerate() {
            draw_motion_figure(
                layout.duo_figure_centers[index],
                layout.figure_scale * 2.0,
                figure,
                player_color(index),
            );
        }
    }
}

// --- Result Overlay ---

#[derive(Clone, Copy, Debug)]
pub struct ResultLayout {
    pub panel: Rect,
    pub chip_width: f32,
    pub chip_height: f32,
    pub chip_y: f32,
    pub choose_y: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct ResultIndicatorLayout {
    pub center: Vec2,
    pub radius: f32,
    pub lock_body: Rect,
}

pub fn result_layout(width: f32, height: f32) -> ResultLayout {
    // Soft-select chips share the Zero-Touch contract: shorter edge ≥ 20% min viewport
    // (ship via hero_edge = 28%). Ít đồ: three huge chips dominate; panel grows to fit.
    let edge = hero_edge(width, height);
    let gap = (width * 0.018).clamp(8.0, 16.0);
    let chip_margin = (width * 0.028).clamp(12.0, 22.0);
    let panel_width = (width - 16.0).max(120.0);
    let chip_width = ((panel_width - chip_margin * 2.0 - gap * 2.0) / 3.0).max(1.0);
    let chip_height = edge;
    let bottom_pad = 24.0_f32.min(height * 0.08).max(12.0);
    let mut top_pad = (edge * 0.35).clamp(24.0, height * 0.2);
    let mut panel_height = (top_pad + chip_height + bottom_pad).min(height - 16.0);
    if panel_height < chip_height + bottom_pad + 8.0 {
        panel_height = (chip_height + bottom_pad + 8.0).min(height - 8.0);
    }
    top_pad = (panel_height - chip_height - bottom_pad).max(4.0);
    let panel = Rect::new(
        (width - panel_width) * 0.5,
        (height - panel_height) * 0.5,
        panel_width,
        panel_height,
    );
    ResultLayout {
        panel,
        chip_width,
        chip_height,
        chip_y: panel.y + top_pad,
        choose_y: panel.y + panel.h - bottom_pad * 0.5,
    }
}

pub fn result_title(outcome: Option<RunnerOutcome>) -> &'static str {
    match outcome {
        Some(RunnerOutcome::BreakComplete) => "GLOW COMPLETE",
        Some(RunnerOutcome::EnergySpent) | None => "AFTERGLOW!",
    }
}

pub fn result_share_hint() -> &'static str {
    ""
}

pub fn result_action_hint() -> &'static str {
    ""
}

pub fn result_chip_rect(layout: ResultLayout, index: usize) -> Rect {
    let gap = (layout.panel.w * 0.018).clamp(8.0, 16.0);
    let chip_margin = (layout.panel.w * 0.028).clamp(12.0, 22.0);
    Rect::new(
        layout.panel.x + chip_margin + index as f32 * (layout.chip_width + gap),
        layout.chip_y,
        layout.chip_width,
        layout.chip_height,
    )
}

pub fn result_indicator_layout(chip: Rect) -> ResultIndicatorLayout {
    let center = vec2(chip.x + chip.w - 16.0, chip.y + 14.0);
    ResultIndicatorLayout {
        center,
        radius: 7.0,
        lock_body: Rect::new(center.x - 8.0, center.y, 16.0, 12.0),
    }
}

pub fn result_choices(duo_available: bool) -> [(GameMode, bool); 3] {
    [
        (GameMode::MirrorBeat, true),
        (GameMode::BeatStrike, true),
        (GameMode::DuoGroove, duo_available),
    ]
}

pub fn result_duo_available(runner: &RunnerGame) -> bool {
    runner.players[0].evaluated && runner.players[1].evaluated
}

pub fn cover_source_rect(source_width: f32, source_height: f32, destination: Rect) -> Rect {
    if source_width <= 0.0 || source_height <= 0.0 || destination.w <= 0.0 || destination.h <= 0.0 {
        return Rect::new(0.0, 0.0, source_width.max(0.0), source_height.max(0.0));
    }
    let source_aspect = source_width / source_height;
    let destination_aspect = destination.w / destination.h;
    if source_aspect > destination_aspect {
        let crop_width = source_height * destination_aspect;
        Rect::new(
            (source_width - crop_width) * 0.5,
            0.0,
            crop_width,
            source_height,
        )
    } else {
        let crop_height = source_width / destination_aspect;
        Rect::new(
            0.0,
            (source_height - crop_height) * 0.5,
            source_width,
            crop_height,
        )
    }
}

fn draw_result_lock(chip: Rect, color: Color) {
    let indicator = result_indicator_layout(chip);
    draw_circle_lines(
        indicator.center.x,
        indicator.center.y,
        indicator.radius - 1.0,
        2.0,
        color,
    );
    draw_rectangle(
        indicator.lock_body.x,
        indicator.lock_body.y,
        indicator.lock_body.w,
        indicator.lock_body.h,
        Color::new(0.03, 0.04, 0.12, 0.94),
    );
    draw_rectangle_lines(
        indicator.lock_body.x,
        indicator.lock_body.y,
        indicator.lock_body.w,
        indicator.lock_body.h,
        2.0,
        color,
    );
}

fn draw_result_overlay(width: f32, height: f32, runner: &RunnerGame, mode_art: &ModeArt) {
    let layout = result_layout(width, height);
    let panel = layout.panel;
    let afterglow = ((get_time() as f32) * 1.6).sin().abs();
    // Soft full-screen afterglow wash — Garden DNA without a generative engine.
    draw_rectangle(
        0.0,
        0.0,
        width,
        height,
        Color::new(0.08, 0.55, 0.72, 0.045 + afterglow * 0.035),
    );
    draw_round_panel(panel, Color::new(0.015, 0.02, 0.09, 0.94));
    draw_rectangle_lines(
        panel.x,
        panel.y,
        panel.w,
        panel.h,
        3.0 + afterglow * 1.5,
        Color::new(0.40, 0.88, 1.0, 0.72 + afterglow * 0.22),
    );

    let leader = runner
        .players
        .iter()
        .filter(|player| player.evaluated)
        .max_by_key(|player| player.score)
        .unwrap_or(&runner.players[0]);
    // Pictogram celebration — VO says "You did it!" / clap replay (no word walls).
    let star_y = panel.y + if height < 500.0 { 42.0 } else { 52.0 };
    let star_r = (panel.w.min(panel.h) * 0.11).max(26.0);
    draw_poly(
        panel.x + panel.w * 0.5,
        star_y,
        5,
        star_r,
        -90.0,
        Color::from_rgba(255, 202, 58, 255),
    );
    let score = format!("{:05}", leader.score);
    let score_font = if width < 520.0 { 28 } else { 34 };
    let score_size = measure_text(&score, None, score_font, 1.0);
    draw_text(
        &score,
        panel.x + (panel.w - score_size.width) * 0.5,
        star_y + star_r + 30.0,
        score_font as f32,
        WHITE,
    );
    let _ = result_title(runner.result_outcome);

    let choices = result_choices(result_duo_available(runner));
    for (index, (mode, available)) in choices.into_iter().enumerate() {
        let selected = mode == runner.result_selection;
        let chip = result_chip_rect(layout, index);
        draw_round_panel(
            chip,
            if selected {
                Color::new(0.08, 0.32, 0.42, 0.94)
            } else {
                Color::new(0.04, 0.05, 0.15, 0.82)
            },
        );
        let art_bounds = Rect::new(chip.x + 4.0, chip.y + 4.0, chip.w - 8.0, chip.h - 8.0);
        if let Some(texture) = mode_art.texture(mode) {
            draw_texture_ex(
                texture,
                art_bounds.x,
                art_bounds.y,
                if available {
                    if selected {
                        WHITE
                    } else {
                        Color::new(0.72, 0.76, 0.90, 0.86)
                    }
                } else {
                    Color::new(0.36, 0.40, 0.52, 0.50)
                },
                DrawTextureParams {
                    dest_size: Some(vec2(art_bounds.w, art_bounds.h)),
                    source: Some(cover_source_rect(
                        texture.width(),
                        texture.height(),
                        art_bounds,
                    )),
                    ..Default::default()
                },
            );
        }
        if !available {
            draw_rectangle(
                art_bounds.x,
                art_bounds.y,
                art_bounds.w,
                art_bounds.h,
                Color::new(0.02, 0.03, 0.10, 0.48),
            );
        }
        draw_rectangle_lines(
            chip.x,
            chip.y,
            chip.w,
            chip.h,
            if selected { 5.0 } else { 1.5 },
            if selected {
                WHITE
            } else {
                Color::new(0.45, 0.32, 0.95, 0.42)
            },
        );
        if !available {
            draw_result_lock(chip, Color::from_rgba(226, 232, 240, 255));
        } else if selected {
            let indicator = result_indicator_layout(chip);
            draw_poly(
                indicator.center.x,
                indicator.center.y,
                4,
                indicator.radius * 1.35,
                45.0,
                Color::from_rgba(103, 232, 249, 255),
            );
        }
    }

    // Replay clap pictogram — spoken "Clap to play!" + big hands
    {
        let cx = panel.x + panel.w * 0.5;
        let cy = layout.choose_y + if height < 500.0 { 18.0 } else { 22.0 };
        let pulse = (macroquad::time::get_time() as f32 * 5.0).sin().abs();
        let hand_r = (width.min(height) * 0.045).max(22.0);
        let gap = hand_r * 0.2 + (1.0 - pulse) * hand_r * 0.9;
        draw_circle(cx - gap, cy, hand_r, Color::from_rgba(255, 202, 58, 255));
        draw_circle(cx - gap, cy, hand_r * 0.5, WHITE);
        draw_circle(cx + gap, cy, hand_r, Color::from_rgba(148, 103, 189, 255));
        draw_circle(cx + gap, cy, hand_r * 0.5, WHITE);
    }
}
