use brainbreak_core::RunnerGame;
use macroquad::prelude::*;

use crate::platform::player_color;
use crate::visuals::{AudioVisual, draw_round_panel};

pub fn draw_header(width: f32, runner: &RunnerGame, guide_only: bool) {
    let margin = (width * 0.03).max(18.0);
    draw_text(
        runner.mode.title(),
        margin,
        34.0,
        if width < 560.0 { 21.0 } else { 25.0 },
        Color::from_rgba(103, 232, 249, 255),
    );
    let state = match runner.phase {
        brainbreak_core::RunnerPhase::Paused => "PAUSED",
        brainbreak_core::RunnerPhase::TrackingHold => "TRACKING HOLD",
        brainbreak_core::RunnerPhase::Starting
        | brainbreak_core::RunnerPhase::PauseResuming
        | brainbreak_core::RunnerPhase::Resuming => "GET READY",
        _ if guide_only => "GUIDANCE ONLY",
        _ => "90 SEC BREAK",
    };
    let size = measure_text(state, None, 14, 1.0);
    draw_text(
        state,
        width - margin - size.width,
        34.0,
        14.0,
        if guide_only {
            Color::from_rgba(253, 230, 138, 255)
        } else {
            Color::from_rgba(196, 181, 253, 255)
        },
    );
}

#[derive(Clone, Copy, Debug)]
pub struct PlayerHudLayout {
    pub displayed: usize,
    pub columns: usize,
    pub card_width: f32,
    pub card_height: f32,
    pub gap: f32,
    pub start_x: f32,
    pub start_y: f32,
}

pub const STAGE_HUD_GAP: f32 = 18.0;

#[derive(Clone, Copy, Debug)]
pub struct RunningSurfaceLayout {
    pub stage: Rect,
    pub deck: Rect,
    pub hud: PlayerHudLayout,
}

#[derive(Clone, Copy, Debug)]
pub struct LifePipLayout {
    pub centers: [Vec2; 3],
    pub radius: f32,
}

pub fn life_pip_layout(card: Rect) -> LifePipLayout {
    let radius = if card.h < 52.0 { 4.5 } else { 5.5 };
    let gap = if card.h < 52.0 { 4.0 } else { 5.0 };
    let start_x = card.x + 16.0 + radius;
    let y = card.y + card.h - radius - 6.0;
    LifePipLayout {
        centers: std::array::from_fn(|index| {
            vec2(start_x + index as f32 * (radius * 2.0 + gap), y)
        }),
        radius,
    }
}

pub fn hud_combo_visible(combo: u16) -> bool {
    combo >= 2
}

pub fn player_hud_layout(width: f32, height: f32, displayed: usize) -> PlayerHudLayout {
    let columns = if displayed > 2 && width < 900.0 {
        2
    } else {
        displayed
    };
    let rows = displayed.div_ceil(columns);
    let side_margin = (width * 0.035).max(14.0);
    let gap = 8.0;
    let card_width =
        ((width - side_margin * 2.0 - gap * (columns - 1) as f32) / columns as f32).min(176.0);
    let card_height = if height < 520.0 && rows > 1 {
        48.0
    } else {
        56.0
    };
    let total_width = columns as f32 * card_width + (columns - 1) as f32 * gap;
    let total_height = rows as f32 * card_height + (rows - 1) as f32 * gap;
    PlayerHudLayout {
        displayed,
        columns,
        card_width,
        card_height,
        gap,
        start_x: (width - total_width) * 0.5,
        start_y: height - side_margin - total_height,
    }
}

pub fn displayed_player_count(runner: &RunnerGame, network: u32) -> usize {
    if network >= 2 {
        4
    } else {
        runner
            .players
            .iter()
            .take(2)
            .filter(|player| player.evaluated)
            .count()
            .max(1)
    }
}

pub fn running_surface_layout(width: f32, height: f32, displayed: usize) -> RunningSurfaceLayout {
    let hud = player_hud_layout(width, height, displayed);
    let stage_bottom = (hud.start_y - STAGE_HUD_GAP).max(1.0);
    RunningSurfaceLayout {
        stage: Rect::new(0.0, 0.0, width, stage_bottom),
        deck: Rect::new(0.0, stage_bottom, width, height - stage_bottom),
        hud,
    }
}

pub fn draw_hud_deck(deck: Rect) {
    draw_rectangle(
        deck.x,
        deck.y,
        deck.w,
        deck.h,
        Color::from_rgba(4, 5, 20, 255),
    );
    draw_rectangle(
        deck.x,
        deck.y,
        deck.w,
        2.0,
        Color::new(0.12, 0.92, 1.0, 0.24),
    );
}

pub fn draw_player_hud(runner: &RunnerGame, layout: PlayerHudLayout) {
    for player in 0..layout.displayed {
        let state = runner.players[player];
        let column = player % layout.columns;
        let row = player / layout.columns;
        let x = layout.start_x + column as f32 * (layout.card_width + layout.gap);
        let y = layout.start_y + row as f32 * (layout.card_height + layout.gap);
        let card = Rect::new(x, y, layout.card_width, layout.card_height);
        draw_round_panel(card, Color::new(0.025, 0.035, 0.11, 0.88));
        draw_rectangle(x, y, 5.0, layout.card_height, player_color(player));
        let top_baseline = y + if layout.card_height < 52.0 {
            20.0
        } else {
            23.0
        };
        let top_font = if layout.card_height < 52.0 { 17 } else { 19 };
        draw_text(
            player_label(player),
            x + 14.0,
            top_baseline,
            top_font as f32,
            player_color(player),
        );
        let score = if state.evaluated {
            format!("{:05}", state.score)
        } else {
            "WAIT".to_owned()
        };
        let score_size = measure_text(&score, None, top_font, 1.0);
        draw_text(
            &score,
            x + layout.card_width - 14.0 - score_size.width,
            top_baseline,
            top_font as f32,
            if state.evaluated {
                WHITE
            } else {
                Color::from_rgba(148, 163, 184, 255)
            },
        );
        draw_life_pips(life_pip_layout(card), state.lives, player_color(player));
        if state.evaluated && hud_combo_visible(state.combo) {
            draw_combo_badge(card, state.combo, player_color(player));
        }
    }
}

fn draw_life_pips(layout: LifePipLayout, lives: u8, color: Color) {
    for (index, center) in layout.centers.into_iter().enumerate() {
        let active = index < usize::from(lives.min(3));
        if active {
            draw_circle(center.x, center.y, layout.radius, color);
            draw_circle(center.x, center.y, layout.radius * 0.38, WHITE);
        } else {
            draw_circle_lines(
                center.x,
                center.y,
                layout.radius,
                1.5,
                Color::from_rgba(148, 163, 184, 170),
            );
            let cross = layout.radius * 0.55;
            draw_line(
                center.x - cross,
                center.y - cross,
                center.x + cross,
                center.y + cross,
                1.5,
                Color::from_rgba(148, 163, 184, 210),
            );
            draw_line(
                center.x + cross,
                center.y - cross,
                center.x - cross,
                center.y + cross,
                1.5,
                Color::from_rgba(148, 163, 184, 210),
            );
        }
    }
}

fn draw_combo_badge(card: Rect, combo: u16, color: Color) {
    let font = if card.h < 52.0 { 13 } else { 15 };
    let label = format!("x{}", combo);
    let size = measure_text(&label, None, font, 1.0);
    let baseline = card.y + card.h - 7.0;
    let center = vec2(card.x + card.w - 18.0 - size.width, baseline - 5.0);
    let radius = if card.h < 52.0 { 5.0 } else { 6.0 };
    for (from, to) in [
        (vec2(0.0, -radius), vec2(radius, 0.0)),
        (vec2(radius, 0.0), vec2(0.0, radius)),
        (vec2(0.0, radius), vec2(-radius, 0.0)),
        (vec2(-radius, 0.0), vec2(0.0, -radius)),
    ] {
        draw_line(
            center.x + from.x,
            center.y + from.y,
            center.x + to.x,
            center.y + to.y,
            2.0,
            color,
        );
    }
    draw_text(
        &label,
        center.x + radius + 5.0,
        baseline,
        font as f32,
        color,
    );
}

pub fn player_label(player: usize) -> &'static str {
    match player {
        0 => "P1",
        1 => "P2",
        2 => "P3",
        _ => "P4",
    }
}

// --- Session Meter ---

#[derive(Clone, Copy, Debug)]
pub struct SessionMeterLayout {
    pub session_rail: Rect,
    pub beat_rail: Rect,
}

pub fn session_meter_layout(width: f32) -> SessionMeterLayout {
    let meter_width = width.min(420.0) * 0.7;
    let x = (width - meter_width) * 0.5;
    SessionMeterLayout {
        session_rail: Rect::new(x, 43.0, meter_width, 4.0),
        beat_rail: Rect::new(x, 50.0, meter_width, 3.0),
    }
}

pub fn draw_session_meter(width: f32, run_progress: f32, audio: AudioVisual) {
    let layout = session_meter_layout(width);
    let progress = run_progress.clamp(0.0, 1.0);
    draw_rectangle(
        layout.session_rail.x,
        layout.session_rail.y,
        layout.session_rail.w,
        layout.session_rail.h,
        Color::new(1.0, 1.0, 1.0, 0.12),
    );
    draw_rectangle(
        layout.session_rail.x,
        layout.session_rail.y,
        layout.session_rail.w * progress,
        layout.session_rail.h,
        Color::new(0.77, 0.52, 0.98, 0.88),
    );
    draw_rectangle(
        layout.beat_rail.x,
        layout.beat_rail.y,
        layout.beat_rail.w,
        layout.beat_rail.h,
        Color::new(1.0, 1.0, 1.0, 0.09),
    );
    draw_rectangle(
        layout.beat_rail.x,
        layout.beat_rail.y,
        layout.beat_rail.w * audio.phase,
        layout.beat_rail.h + audio.energy * 2.0,
        Color::new(0.25 + audio.energy * 0.4, 0.85, 1.0, 0.9),
    );
}
