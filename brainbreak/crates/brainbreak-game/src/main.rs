#[cfg(target_arch = "wasm32")]
mod config_render;
mod hud;
mod overlays;
mod platform;
mod supernova_render;
mod visuals;

use brainbreak_core::{
    MotionInputFrame, MotionRuntime, RunnerFeedback, RunnerGame, RunnerPhase,
    SupernovaGame, SupernovaOutcome, SupernovaPhase,
};
#[cfg(target_arch = "wasm32")]
use brainbreak_core::{ConfigGame, RunnerOutcome};
use macroquad::prelude::*;

#[cfg(target_arch = "wasm32")]
use config_render::ConfigStage;
use supernova_render::SupernovaStage;
use visuals::RunnerStage;

fn window_conf() -> Conf {
    Conf {
        window_title: "Neon Beat Runner".to_owned(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        fullscreen: false,
        sample_count: 4,
        ..Default::default()
    }
}

/// Returns true when the JS bridge (or native Tab key) selects Supernova mode (value 3).
fn is_supernova_mode() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        platform::raw_game_mode() == 3
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        is_key_down(KeyCode::Tab)
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut motion = MotionRuntime::default();
    let mut runner = RunnerGame::new();
    let mut stage = RunnerStage::default();
    let mode_art = platform::ModeArt::load().await;
    #[cfg(not(target_arch = "wasm32"))]
    let mut selected_mode = platform::current_game_mode();

    // Supernova Drop state
    let mut supernova = SupernovaGame::new();
    let mut supernova_stage = SupernovaStage::default();
    let mut supernova_started = false;
    // Party event tracking (music/voice cues to JS).
    let mut snova_phase_sent = SupernovaPhase::Ready;
    let mut snova_countdown_sent: u8 = u8::MAX;

    // Custom config-driven game state
    #[cfg(target_arch = "wasm32")]
    let mut custom_config: Option<brainbreak_core::GameConfig> = None;
    #[cfg(target_arch = "wasm32")]
    let mut custom_game: Option<ConfigGame> = None;
    #[cfg(target_arch = "wasm32")]
    let mut custom_stage = ConfigStage::default();
    #[cfg(target_arch = "wasm32")]
    let mut custom_started = false;

    loop {
        let dt = get_frame_time().min(0.05);
        #[cfg(target_arch = "wasm32")]
        let desired_mode = platform::current_game_mode();
        #[cfg(not(target_arch = "wasm32"))]
        let desired_mode = selected_mode;
        if motion.game.mode != desired_mode {
            motion.set_mode(desired_mode);
            runner.set_mode(desired_mode);
        }
        let poses = platform::browser_poses();
        let network = platform::network_status();
        let local_evaluation = [
            platform::evaluation_enabled(0),
            platform::evaluation_enabled(1),
        ];
        let guide_only = !local_evaluation.into_iter().any(|enabled| enabled);
        #[cfg(target_arch = "wasm32")]
        let (fallback_actions, remote_actions) = {
            let mut fallback = [
                platform::keyboard_actions(runner.phase == RunnerPhase::GameOver),
                0,
            ];
            fallback[0] |= platform::take_gamepad_actions(0);
            fallback[1] |= platform::take_gamepad_actions(1);
            (
                fallback,
                [
                    platform::take_remote_actions(0),
                    platform::take_remote_actions(1),
                ],
            )
        };
        #[cfg(not(target_arch = "wasm32"))]
        let (fallback_actions, remote_actions) = (
            [
                platform::keyboard_actions(runner.phase == RunnerPhase::GameOver),
                0,
            ],
            [0, 0],
        );

        motion.update(
            dt,
            MotionInputFrame {
                local_poses: [poses.first().copied(), poses.get(1).copied()],
                fallback_actions,
                remote_actions,
                evaluation_enabled: [
                    local_evaluation[0],
                    local_evaluation[1],
                    network >= 2,
                    network >= 2,
                ],
                custom_target: None,
            },
        );
        #[cfg(target_arch = "wasm32")]
        for (player, mask) in motion.local_evaluated_triggered().into_iter().enumerate() {
            if mask != 0 {
                platform::send_local_action(player as u32, mask);
            }
        }

        let active = motion.players.map(|player| player.active);
        let triggered = motion.players.map(|player| {
            if player.evaluation_enabled {
                player.triggered
            } else {
                0
            }
        });
        let evaluation = motion.players.map(|player| player.evaluation_enabled);

        // --- Supernova Drop mode (bypasses runner entirely) ---
        if is_supernova_mode() {
            // Auto-start when at least one player is detected.
            if !supernova_started && active.iter().any(|a| *a != 0) {
                supernova.start_run(evaluation);
                supernova_started = true;
            }
            // Handle replay from result screen.
            if supernova.phase == SupernovaPhase::Result {
                let any_action = triggered.iter().any(|t| *t != 0);
                if any_action && supernova.request_replay() {
                    supernova_started = true;
                }
            }
            supernova.update(dt, active, triggered);

            // Emit party events (music/voice cues) to the JS party director.
            // Phase-change block runs first so entering Countdown resets the
            // countdown tracker (avoids double-sending the first tick).
            if supernova.phase != snova_phase_sent {
                let event = match supernova.phase {
                    SupernovaPhase::Dance => Some(2),
                    SupernovaPhase::Freeze => Some(3),
                    SupernovaPhase::Drop => Some(5),
                    SupernovaPhase::Result => Some(6),
                    _ => None,
                };
                if let Some(kind) = event {
                    let value = if supernova.phase == SupernovaPhase::Result {
                        match supernova.outcome {
                            Some(SupernovaOutcome::FullSupernova) => 0,
                            _ => 1,
                        }
                    } else {
                        0
                    };
                    platform::supernova_event(kind, value);
                }
                if supernova.phase == SupernovaPhase::Countdown {
                    snova_countdown_sent = u8::MAX;
                }
                snova_phase_sent = supernova.phase;
            }
            if supernova.phase == SupernovaPhase::Countdown {
                let num = supernova.countdown_remaining.ceil() as u8;
                if num != snova_countdown_sent && num >= 1 {
                    snova_countdown_sent = num;
                    platform::supernova_event(1, u32::from(num));
                }
            }
            if supernova.perfect_freeze {
                platform::supernova_event(4, 0);
            }

            let audio = platform::audio_visual();
            let reduced = platform::reduce_motion();
            supernova_stage.update(dt, &supernova, audio, reduced);

            clear_background(Color::from_rgba(4, 5, 20, 255));
            supernova_stage.draw(&supernova, audio, reduced);
            next_frame().await;
            continue;
        }
        supernova_started = false;

        // --- Custom config-driven game (mode 4, bypasses runner) ---
        #[cfg(target_arch = "wasm32")]
        if platform::raw_game_mode() == 4 {
            // Lazy-load config on first entry.
            if custom_config.is_none() {
                custom_config = platform::load_game_config();
                custom_game = custom_config.as_ref().map(ConfigGame::new);
            }
            if let (Some(cfg), Some(game)) = (&custom_config, &mut custom_game) {
                // Auto-start when at least one player is detected.
                if !custom_started && active.iter().any(|a| *a != 0) {
                    game.start_run(cfg, evaluation);
                    custom_started = true;
                }
                // Handle replay from result screen.
                if game.phase == brainbreak_core::ConfigPhase::Result {
                    let any_action = triggered.iter().any(|t| *t != 0);
                    if any_action {
                        game.request_replay(cfg);
                    }
                }
                game.update(dt, cfg, triggered);
                custom_stage.update(dt, game);
                custom_stage.draw(game, cfg);
                next_frame().await;
                continue;
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            custom_config = None;
            custom_game = None;
            custom_started = false;
        }

        runner.update(dt, active, triggered, evaluation);
        if let Some(result) = runner.take_result_event() {
            #[cfg(target_arch = "wasm32")]
            {
                let outcome = match result.outcome {
                    RunnerOutcome::BreakComplete => 1,
                    RunnerOutcome::EnergySpent => 2,
                };
                platform::record_run_outcome(platform::game_mode_value(result.mode), outcome);
            }
            #[cfg(not(target_arch = "wasm32"))]
            let _ = result;
        }
        if let Some(next_mode) = runner.take_next_run_request() {
            #[cfg(not(target_arch = "wasm32"))]
            {
                selected_mode = next_mode;
            }
            motion.set_mode(next_mode);
            runner.start_run(next_mode, evaluation);
            #[cfg(target_arch = "wasm32")]
            platform::set_game_mode(platform::game_mode_value(next_mode));
        }
        #[cfg(target_arch = "wasm32")]
        for (player, feedback) in runner.feedback.into_iter().enumerate() {
            let kind = match feedback {
                RunnerFeedback::None => 0,
                RunnerFeedback::Dodge => 1,
                RunnerFeedback::Crash => 2,
                RunnerFeedback::BeatPickup => 3,
            };
            if kind != 0 {
                platform::play_feedback(kind, u32::from(runner.players[player].combo));
            }
        }
        stage.update(dt, &runner);

        let width = screen_width();
        let height = screen_height();
        let audio = platform::audio_visual();
        let reduced = platform::reduce_motion();
        let shake = if reduced {
            Vec2::ZERO
        } else {
            let crash = runner.feedback.contains(&RunnerFeedback::Crash);
            if crash {
                vec2(
                    (get_time() as f32 * 91.0).sin() * 5.0,
                    (get_time() as f32 * 73.0).cos() * 3.0,
                )
            } else {
                Vec2::ZERO
            }
        };
        clear_background(Color::from_rgba(4, 5, 20, 255));
        let running_surface = (runner.phase == RunnerPhase::Running).then(|| {
            hud::running_surface_layout(
                width,
                height,
                hud::displayed_player_count(&runner, network),
            )
        });
        let stage_bounds = running_surface
            .map(|layout| layout.stage)
            .unwrap_or(Rect::new(0.0, 0.0, width, height));
        let world_bounds = Rect::new(
            stage_bounds.x + shake.x,
            stage_bounds.y + shake.y,
            stage_bounds.w,
            stage_bounds.h,
        );
        stage.draw(&runner, &motion, world_bounds, audio, reduced);
        if let Some(layout) = running_surface {
            hud::draw_hud_deck(layout.deck);
        }
        hud::draw_header(width, &runner, guide_only);
        if let Some(layout) = running_surface {
            hud::draw_player_hud(&runner, layout.hud);
        }
        overlays::draw_phase_overlay(width, height, &runner, guide_only, poses.len(), &mode_art);
        if running_surface.is_some() {
            hud::draw_session_meter(width, runner.run_progress(), audio);
        }

        next_frame().await;
    }
}

#[cfg(test)]
mod layout_tests {
    use brainbreak_core::{GameMode, PLAYER_CAPACITY, RunnerGame, RunnerOutcome};
    use macroquad::prelude::*;

    use crate::hud::{
        STAGE_HUD_GAP, hud_combo_visible, life_pip_layout, player_hud_layout,
        running_surface_layout, session_meter_layout,
    };
    use crate::overlays::{
        COUNTDOWN_TICK_COUNT, CountdownNumeral, MotionFigure, countdown_active_ticks,
        countdown_overlay_layout, countdown_presentation, countdown_strokes, cover_source_rect,
        motion_figure_bounds, motion_prompt_layout, pause_overlay_layout, pause_presentation,
        ready_presentation, result_chip_rect, result_choices, result_duo_available,
        result_indicator_layout, result_layout, result_title, tracking_hold_presentation,
    };
    use crate::platform::{
        DUO_ART_PATH, MIRROR_ART_PATH, MODE_ART_MAX_ENCODED_BYTES, ModeArt, STRIKE_ART_PATH,
        decode_mode_art,
    };

    const VIEWPORTS: [(f32, f32); 3] = [(390.0, 844.0), (667.0, 375.0), (1440.0, 784.0)];

    fn player_flags(p1: bool, p2: bool) -> [bool; PLAYER_CAPACITY] {
        [p1, p2, false, false]
    }

    #[test]
    fn ready_presentation_preserves_runtime_truth_with_static_ascii_copy() {
        let cases = [
            (
                ready_presentation(GameMode::MirrorBeat, true, 0, false, false),
                "CAMERA NEEDED",
                [MotionFigure::Camera, MotionFigure::Hidden],
            ),
            (
                ready_presentation(GameMode::MirrorBeat, false, 0, false, false),
                "STEP INTO FRAME",
                [MotionFigure::Searching, MotionFigure::Hidden],
            ),
            (
                ready_presentation(GameMode::MirrorBeat, false, 1, false, false),
                "READY TO MOVE",
                [MotionFigure::Present, MotionFigure::Hidden],
            ),
            (
                ready_presentation(GameMode::MirrorBeat, false, 1, true, false),
                "SIGNAL RECEIVED",
                [MotionFigure::Ready, MotionFigure::Hidden],
            ),
            (
                ready_presentation(GameMode::DuoGroove, false, 1, false, false),
                "PLAYER 1 FOUND",
                [MotionFigure::Present, MotionFigure::Searching],
            ),
            (
                ready_presentation(GameMode::DuoGroove, false, 1, true, false),
                "PLAYER 1 READY",
                [MotionFigure::Ready, MotionFigure::Searching],
            ),
            (
                ready_presentation(GameMode::DuoGroove, false, 2, false, false),
                "DUO READY CHECK",
                [MotionFigure::Present, MotionFigure::Present],
            ),
            (
                ready_presentation(GameMode::DuoGroove, false, 2, true, false),
                "PLAYER 1 READY",
                [MotionFigure::Ready, MotionFigure::Present],
            ),
            (
                ready_presentation(GameMode::DuoGroove, false, 2, false, true),
                "PLAYER 2 READY",
                [MotionFigure::Present, MotionFigure::Ready],
            ),
            (
                ready_presentation(GameMode::DuoGroove, false, 2, true, true),
                "BOTH PLAYERS READY",
                [MotionFigure::Ready, MotionFigure::Ready],
            ),
        ];

        for (presentation, title, figures) in cases {
            assert_eq!(presentation.title, title);
            assert_eq!(presentation.figures, figures);
            assert!(presentation.title.is_ascii());
            assert!(presentation.instruction.is_ascii());
            assert!(presentation.title.len() <= 20);
            assert!(presentation.instruction.len() <= 25);
        }
    }

    #[test]
    fn tracking_hold_presentation_preserves_required_player_truth() {
        let remote_only = [false, false, true, true];
        let remote_ready = [false, false, true, false];
        let cases = [
            (
                tracking_hold_presentation(
                    GameMode::MirrorBeat,
                    player_flags(false, false),
                    player_flags(false, false),
                ),
                "FIND YOUR FRAME",
                [MotionFigure::Searching, MotionFigure::Hidden],
            ),
            (
                tracking_hold_presentation(
                    GameMode::MirrorBeat,
                    player_flags(true, false),
                    player_flags(false, false),
                ),
                "TRACKING RESTORED",
                [MotionFigure::Present, MotionFigure::Hidden],
            ),
            (
                tracking_hold_presentation(GameMode::MirrorBeat, remote_only, remote_ready),
                "SIGNAL RECEIVED",
                [MotionFigure::Ready, MotionFigure::Hidden],
            ),
            (
                tracking_hold_presentation(
                    GameMode::DuoGroove,
                    remote_only,
                    [false; PLAYER_CAPACITY],
                ),
                "FIND BOTH PLAYERS",
                [MotionFigure::Searching, MotionFigure::Searching],
            ),
            (
                tracking_hold_presentation(
                    GameMode::DuoGroove,
                    player_flags(true, false),
                    player_flags(false, false),
                ),
                "PLAYER 1 FOUND",
                [MotionFigure::Present, MotionFigure::Searching],
            ),
            (
                tracking_hold_presentation(
                    GameMode::DuoGroove,
                    player_flags(false, true),
                    player_flags(false, true),
                ),
                "PLAYER 2 READY",
                [MotionFigure::Searching, MotionFigure::Ready],
            ),
            (
                tracking_hold_presentation(
                    GameMode::DuoGroove,
                    player_flags(true, true),
                    player_flags(false, false),
                ),
                "TRACKING RESTORED",
                [MotionFigure::Present, MotionFigure::Present],
            ),
            (
                tracking_hold_presentation(
                    GameMode::DuoGroove,
                    player_flags(true, true),
                    player_flags(true, false),
                ),
                "PLAYER 1 READY",
                [MotionFigure::Ready, MotionFigure::Present],
            ),
            (
                tracking_hold_presentation(
                    GameMode::DuoGroove,
                    player_flags(true, true),
                    player_flags(false, true),
                ),
                "PLAYER 2 READY",
                [MotionFigure::Present, MotionFigure::Ready],
            ),
            (
                tracking_hold_presentation(
                    GameMode::DuoGroove,
                    player_flags(true, true),
                    player_flags(true, true),
                ),
                "BOTH PLAYERS READY",
                [MotionFigure::Ready, MotionFigure::Ready],
            ),
        ];

        for (presentation, title, figures) in cases {
            assert_eq!(presentation.title, title);
            assert_eq!(presentation.figures, figures);
            assert!(!presentation.title.contains("PAUSED"));
            assert!(presentation.title.is_ascii());
            assert!(presentation.instruction.is_ascii());
            assert!(presentation.title.len() <= 20);
            assert!(presentation.instruction.len() <= 25);
        }
    }

    #[test]
    fn pause_presentation_requires_frame_before_clap_and_preserves_player_truth() {
        let remote_only = [false, false, true, true];
        let remote_ready = [false, false, true, false];
        let cases = [
            (
                pause_presentation(
                    GameMode::MirrorBeat,
                    [false; PLAYER_CAPACITY],
                    [false; PLAYER_CAPACITY],
                ),
                "STEP BACK INTO FRAME",
                [MotionFigure::Searching, MotionFigure::Hidden],
            ),
            (
                pause_presentation(
                    GameMode::MirrorBeat,
                    player_flags(true, false),
                    [false; PLAYER_CAPACITY],
                ),
                "CLAP TO RESUME",
                [MotionFigure::Present, MotionFigure::Hidden],
            ),
            (
                pause_presentation(GameMode::MirrorBeat, remote_only, remote_ready),
                "GET READY",
                [MotionFigure::Ready, MotionFigure::Hidden],
            ),
            (
                pause_presentation(GameMode::DuoGroove, remote_only, [false; PLAYER_CAPACITY]),
                "BOTH STEP BACK INTO FRAME",
                [MotionFigure::Searching, MotionFigure::Searching],
            ),
            (
                pause_presentation(
                    GameMode::DuoGroove,
                    player_flags(true, false),
                    player_flags(false, false),
                ),
                "PLAYER 2 BACK IN FRAME",
                [MotionFigure::Present, MotionFigure::Searching],
            ),
            (
                pause_presentation(
                    GameMode::DuoGroove,
                    player_flags(false, true),
                    player_flags(false, true),
                ),
                "PLAYER 1 BACK IN FRAME",
                [MotionFigure::Searching, MotionFigure::Ready],
            ),
            (
                pause_presentation(
                    GameMode::DuoGroove,
                    player_flags(true, true),
                    player_flags(false, false),
                ),
                "BOTH CLAP TO RESUME",
                [MotionFigure::Present, MotionFigure::Present],
            ),
            (
                pause_presentation(
                    GameMode::DuoGroove,
                    player_flags(true, true),
                    player_flags(true, false),
                ),
                "PLAYER 2 CLAP TO RESUME",
                [MotionFigure::Ready, MotionFigure::Present],
            ),
            (
                pause_presentation(
                    GameMode::DuoGroove,
                    player_flags(true, true),
                    player_flags(false, true),
                ),
                "PLAYER 1 CLAP TO RESUME",
                [MotionFigure::Present, MotionFigure::Ready],
            ),
            (
                pause_presentation(
                    GameMode::DuoGroove,
                    player_flags(true, true),
                    player_flags(true, true),
                ),
                "GET READY",
                [MotionFigure::Ready, MotionFigure::Ready],
            ),
        ];

        for (presentation, instruction, figures) in cases {
            assert_eq!(presentation.title, "PAUSED");
            assert_eq!(presentation.instruction, instruction);
            assert_eq!(presentation.figures, figures);
            assert!(presentation.instruction.is_ascii());
            assert!(!presentation.instruction.contains('•'));
            assert!(presentation.instruction.len() <= 25);
        }
    }

    #[test]
    fn countdown_presentation_is_bounded_and_resets_once_between_two_and_one() {
        let cases = [
            (2.5, CountdownNumeral::Two, 1.0, 12),
            (2.0, CountdownNumeral::Two, 1.0, 12),
            (1.5, CountdownNumeral::Two, 0.5, 6),
            (1.01, CountdownNumeral::Two, 0.01, 1),
            (1.0, CountdownNumeral::One, 1.0, 12),
            (0.5, CountdownNumeral::One, 0.5, 6),
            (0.01, CountdownNumeral::One, 0.01, 1),
            (0.0, CountdownNumeral::One, 0.0, 0),
            (-1.0, CountdownNumeral::One, 0.0, 0),
            (f32::NAN, CountdownNumeral::One, 0.0, 0),
            (f32::INFINITY, CountdownNumeral::One, 0.0, 0),
        ];

        for (remaining, numeral, fraction, ticks) in cases {
            let presentation = countdown_presentation(remaining);
            assert_eq!(presentation.numeral, numeral);
            assert!((presentation.second_fraction - fraction).abs() < 0.001);
            assert_eq!(countdown_active_ticks(presentation.second_fraction), ticks);
        }
        assert_eq!(countdown_active_ticks(f32::NAN), 0);
        assert_eq!(countdown_active_ticks(-1.0), 0);
        assert_eq!(countdown_active_ticks(2.0), COUNTDOWN_TICK_COUNT);
    }

    #[test]
    fn countdown_numerals_use_distinct_bounded_vector_strokes() {
        let (two_strokes, two_count) = countdown_strokes(CountdownNumeral::Two);
        let (one_strokes, one_count) = countdown_strokes(CountdownNumeral::One);
        assert_eq!(two_count, 5);
        assert_eq!(one_count, 3);
        assert_ne!(two_strokes[..two_count], one_strokes[..one_count]);

        for stroke in two_strokes
            .into_iter()
            .take(two_count)
            .chain(one_strokes.into_iter().take(one_count))
        {
            for point in [stroke.from, stroke.to] {
                assert!(point.x.abs() <= 0.5);
                assert!(point.y.abs() <= 0.5);
            }
            assert_ne!(stroke.from, stroke.to);
        }
    }

    #[test]
    fn motion_prompt_layout_keeps_art_copy_and_figures_inside_target_viewports() {
        for (width, height) in VIEWPORTS {
            let layout = motion_prompt_layout(width, height);
            assert!(layout.panel.x >= 0.0);
            assert!(layout.panel.y >= 0.0);
            assert!(layout.panel.x + layout.panel.w <= width);
            assert!(layout.panel.y + layout.panel.h <= height);
            for region in [layout.art, layout.content] {
                assert!(region.x >= layout.panel.x);
                assert!(region.y >= layout.panel.y);
                assert!(region.x + region.w <= layout.panel.x + layout.panel.w);
                assert!(region.y + region.h <= layout.panel.y + layout.panel.h);
            }
            assert!(layout.art.x + layout.art.w < layout.content.x);
            for center in layout.figure_centers {
                let bounds = motion_figure_bounds(center, layout.figure_scale);
                assert!(bounds.x >= layout.content.x);
                assert!(bounds.y >= layout.content.y);
                assert!(bounds.x + bounds.w <= layout.content.x + layout.content.w);
                assert!(bounds.y + bounds.h <= layout.content.y + layout.content.h);
            }

            let source = cover_source_rect(640.0, 640.0, layout.art);
            assert!((source.w / source.h - layout.art.w / layout.art.h).abs() < 0.001);
        }
    }

    #[test]
    fn result_layout_stays_inside_target_viewports() {
        for (width, height) in VIEWPORTS {
            let layout = result_layout(width, height);
            assert!(layout.panel.x >= 0.0);
            assert!(layout.panel.y >= 0.0);
            assert!(layout.panel.x + layout.panel.w <= width);
            assert!(layout.panel.y + layout.panel.h <= height);
            assert!(layout.chip_width >= 90.0);
            assert!(layout.chip_y >= layout.panel.y);
            assert!(layout.chip_y + layout.chip_height < layout.choose_y);
            assert!(layout.choose_y + 24.0 <= layout.panel.y + layout.panel.h);
            for index in 0..3 {
                let chip = result_chip_rect(layout, index);
                assert!(chip.x >= layout.panel.x);
                assert!(chip.y >= layout.panel.y);
                assert!(chip.x + chip.w <= layout.panel.x + layout.panel.w);
                assert!(chip.y + chip.h <= layout.panel.y + layout.panel.h);
                let indicator = result_indicator_layout(chip);
                assert!(indicator.center.x - indicator.radius >= chip.x);
                assert!(indicator.center.x + indicator.radius <= chip.x + chip.w);
                assert!(indicator.center.y - indicator.radius >= chip.y);
                assert!(indicator.lock_body.y + indicator.lock_body.h <= chip.y + chip.h);
            }
        }
        assert_eq!(
            result_title(Some(RunnerOutcome::BreakComplete)),
            "BREAK COMPLETE"
        );
        assert_eq!(result_title(Some(RunnerOutcome::EnergySpent)), "NICE RUN!");
    }

    #[test]
    fn result_art_crop_covers_cards_without_stretching() {
        for (width, height) in VIEWPORTS {
            let layout = result_layout(width, height);
            for index in 0..3 {
                let destination = result_chip_rect(layout, index);
                let source = cover_source_rect(640.0, 640.0, destination);
                assert!(source.x >= 0.0);
                assert!(source.y >= 0.0);
                assert!(source.x + source.w <= 640.0);
                assert!(source.y + source.h <= 640.0);
                assert!((source.w / source.h - destination.w / destination.h).abs() < 0.001);
            }
        }

        let invalid = cover_source_rect(0.0, 640.0, Rect::new(0.0, 0.0, 100.0, 50.0));
        assert_eq!(invalid, Rect::new(0.0, 0.0, 0.0, 640.0));
    }

    #[test]
    fn result_duo_art_matches_deterministic_availability() {
        let mut runner = RunnerGame::new();
        assert!(!result_duo_available(&runner));
        assert_eq!(
            result_choices(false),
            [
                (GameMode::MirrorBeat, true),
                (GameMode::BeatStrike, true),
                (GameMode::DuoGroove, false),
            ]
        );

        runner.players[0].evaluated = true;
        runner.players[1].evaluated = true;
        assert!(result_duo_available(&runner));
        assert!(
            result_choices(true)
                .into_iter()
                .all(|(_, available)| available)
        );
    }

    #[test]
    fn canonical_mode_art_paths_exist_and_fallback_is_empty() {
        for path in [MIRROR_ART_PATH, STRIKE_ART_PATH, DUO_ART_PATH] {
            assert!(
                std::path::Path::new(path).is_file(),
                "missing canonical art: {path}"
            );
            let bytes = std::fs::read(path).expect("read canonical mode art");
            let decoded = decode_mode_art(&bytes).expect("decode canonical WebP mode art");
            assert_eq!((decoded.width, decoded.height), (640, 640));
            assert_eq!(decoded.rgba.len(), 640 * 640 * 4);
        }
        assert!(decode_mode_art(&[]).is_none());
        assert!(decode_mode_art(&vec![0; MODE_ART_MAX_ENCODED_BYTES + 1]).is_none());
        assert!(decode_mode_art(b"not a webp").is_none());
        let fallback = ModeArt::default();
        for mode in [
            GameMode::MirrorBeat,
            GameMode::BeatStrike,
            GameMode::DuoGroove,
        ] {
            assert!(fallback.texture(mode).is_none());
        }
    }

    #[test]
    fn session_and_beat_rails_stay_separate_inside_target_viewports() {
        for (width, height) in VIEWPORTS {
            let layout = session_meter_layout(width);
            for rail in [layout.session_rail, layout.beat_rail] {
                assert!(rail.x >= 0.0);
                assert!(rail.y >= 0.0);
                assert!(rail.x + rail.w <= width);
                assert!(rail.y + rail.h <= height);
            }
            assert!(layout.session_rail.y + layout.session_rail.h < layout.beat_rail.y);
            assert!(layout.beat_rail.y + layout.beat_rail.h < 58.0);
        }
    }

    #[test]
    fn four_player_hud_stays_inside_target_viewports() {
        for (width, height) in VIEWPORTS {
            let layout = player_hud_layout(width, height, 4);
            let rows = 4_usize.div_ceil(layout.columns);
            let right = layout.start_x
                + layout.columns as f32 * layout.card_width
                + (layout.columns - 1) as f32 * layout.gap;
            let bottom =
                layout.start_y + rows as f32 * layout.card_height + (rows - 1) as f32 * layout.gap;
            assert!(layout.start_x >= 0.0);
            assert!(layout.start_y >= 0.0);
            assert!(right <= width);
            assert!(bottom <= height);
            assert!(layout.card_width >= 150.0);
        }
    }

    #[test]
    fn running_surface_reserves_a_body_clear_hud_deck() {
        for (width, height) in VIEWPORTS {
            for displayed in 1..=4 {
                let layout = running_surface_layout(width, height, displayed);
                let rows = displayed.div_ceil(layout.hud.columns);
                let hud_bottom = layout.hud.start_y
                    + rows as f32 * layout.hud.card_height
                    + (rows - 1) as f32 * layout.hud.gap;

                assert_eq!(layout.stage.x, 0.0);
                assert_eq!(layout.stage.y, 0.0);
                assert_eq!(layout.stage.w, width);
                assert_eq!(layout.deck.x, 0.0);
                assert_eq!(layout.deck.y, layout.stage.h);
                assert_eq!(layout.deck.w, width);
                assert_eq!(layout.deck.y + layout.deck.h, height);
                assert!(layout.stage.h >= height * 0.60);
                assert!(layout.stage.h + STAGE_HUD_GAP <= layout.hud.start_y);
                assert!(layout.hud.start_y >= layout.deck.y);
                assert!(hud_bottom <= layout.deck.y + layout.deck.h);
            }
        }
    }

    #[test]
    fn hud_life_pips_stay_inside_cards_and_combo_starts_at_two() {
        assert!(!hud_combo_visible(0));
        assert!(!hud_combo_visible(1));
        assert!(hud_combo_visible(2));

        for (width, height) in VIEWPORTS {
            for displayed in 1..=4 {
                let layout = player_hud_layout(width, height, displayed);
                let card = Rect::new(
                    layout.start_x,
                    layout.start_y,
                    layout.card_width,
                    layout.card_height,
                );
                let pips = life_pip_layout(card);
                for center in pips.centers {
                    assert!(center.x - pips.radius >= card.x);
                    assert!(center.x + pips.radius <= card.x + card.w);
                    assert!(center.y - pips.radius >= card.y);
                    assert!(center.y + pips.radius <= card.y + card.h);
                }
            }
        }
    }

    #[test]
    fn start_and_resume_countdown_shapes_stay_inside_target_viewports() {
        for (width, height) in VIEWPORTS {
            let layout = countdown_overlay_layout(width, height);
            let panel = layout.panel;
            assert!(panel.x >= 0.0);
            assert!(panel.y >= 0.0);
            assert!(panel.x + panel.w <= width);
            assert!(panel.y + panel.h <= height);
            assert!(panel.w >= 320.0);
            let outer_radius = layout.ring_radius + 8.0;
            assert!(layout.ring_center.x - outer_radius >= panel.x);
            assert!(layout.ring_center.x + outer_radius <= panel.x + panel.w);
            assert!(layout.ring_center.y - outer_radius >= panel.y);
            assert!(layout.ring_center.y + outer_radius <= panel.y + panel.h);
            assert!(layout.title_y + 18.0 < layout.ring_center.y - outer_radius);
            assert!(layout.stroke_scale * 0.5 < layout.ring_radius);
        }
    }

    #[test]
    fn intentional_pause_panel_and_shape_stay_inside_target_viewports() {
        for (width, height) in VIEWPORTS {
            let layout = pause_overlay_layout(width, height);
            assert!(layout.panel.x >= 0.0);
            assert!(layout.panel.y >= 0.0);
            assert!(layout.panel.x + layout.panel.w <= width);
            assert!(layout.panel.y + layout.panel.h <= height);
            for bar in [layout.left_bar, layout.right_bar] {
                assert!(bar.x >= layout.panel.x);
                assert!(bar.y >= layout.panel.y);
                assert!(bar.x + bar.w <= layout.panel.x + layout.panel.w);
                assert!(bar.y + bar.h <= layout.title_y);
            }
            assert!(layout.title_y < layout.instruction_y);
            let single_bounds =
                motion_figure_bounds(layout.single_figure_center, layout.figure_scale);
            assert!(layout.instruction_y + 18.0 < single_bounds.y);
            assert!(single_bounds.x >= layout.panel.x);
            assert!(single_bounds.y >= layout.panel.y);
            assert!(single_bounds.x + single_bounds.w <= layout.panel.x + layout.panel.w);
            assert!(single_bounds.y + single_bounds.h <= layout.panel.y + layout.panel.h);

            let duo_bounds = layout
                .duo_figure_centers
                .map(|center| motion_figure_bounds(center, layout.figure_scale));
            for bounds in duo_bounds {
                assert!(bounds.x >= layout.panel.x);
                assert!(bounds.y >= layout.panel.y);
                assert!(bounds.x + bounds.w <= layout.panel.x + layout.panel.w);
                assert!(bounds.y + bounds.h <= layout.panel.y + layout.panel.h);
            }
            assert!(duo_bounds[0].x + duo_bounds[0].w < duo_bounds[1].x);
        }
    }
}
