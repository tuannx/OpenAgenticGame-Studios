use crate::*;

fn standing_pose(timestamp_ms: f64) -> PoseFrame {
    let mut keypoints = [Keypoint {
        x: 0.5,
        y: 0.5,
        confidence: 1.0,
    }; KEYPOINT_COUNT];
    keypoints[5] = Keypoint {
        x: 0.4,
        y: 0.35,
        confidence: 1.0,
    };
    keypoints[6] = Keypoint {
        x: 0.6,
        y: 0.35,
        confidence: 1.0,
    };
    keypoints[9] = Keypoint {
        x: 0.4,
        y: 0.55,
        confidence: 1.0,
    };
    keypoints[10] = Keypoint {
        x: 0.6,
        y: 0.55,
        confidence: 1.0,
    };
    keypoints[11] = Keypoint {
        x: 0.44,
        y: 0.60,
        confidence: 1.0,
    };
    keypoints[12] = Keypoint {
        x: 0.56,
        y: 0.60,
        confidence: 1.0,
    };
    PoseFrame {
        tracked_id: 1,
        quality: 1.0,
        timestamp_ms,
        keypoints,
    }
}

fn pause_pose(timestamp_ms: f64) -> PoseFrame {
    let mut pose = standing_pose(timestamp_ms);
    pose.keypoints[9] = Keypoint {
        x: 0.30,
        y: 0.18,
        confidence: 1.0,
    };
    pose.keypoints[10] = Keypoint {
        x: 0.70,
        y: 0.18,
        confidence: 1.0,
    };
    pose
}

#[test]
fn raised_hand_only_triggers_on_rising_edge() {
    let mut recognizer = PoseRecognizer::default();
    let pose = standing_pose(0.0);
    recognizer.update(&pose);
    let mut raised = standing_pose(300.0);
    raised.keypoints[9].y = 0.20;
    assert_eq!(recognizer.update(&raised).triggered, Action::LeftUp.mask());
    raised.timestamp_ms = 600.0;
    assert_eq!(recognizer.update(&raised).triggered, 0);
}

#[test]
fn both_wrists_below_hips_trigger_hands_down() {
    let mut recognizer = PoseRecognizer::default();
    recognizer.update(&standing_pose(0.0));

    let mut reach_down = standing_pose(300.0);
    reach_down.keypoints[9].y = 0.72;
    reach_down.keypoints[10].y = 0.72;
    let sample = recognizer.update(&reach_down);
    assert_ne!(sample.active & Action::HandsDown.mask(), 0);
    assert_eq!(sample.triggered & Action::HandsDown.mask(), Action::HandsDown.mask());
    assert_eq!(sample.active & Action::GAMEPLAY_MASK, 0);

    reach_down.timestamp_ms = 600.0;
    assert_eq!(recognizer.update(&reach_down).triggered & Action::HandsDown.mask(), 0);
}

#[test]
fn pause_requires_a_one_second_separated_two_hand_hold() {
    let mut recognizer = PoseRecognizer::default();
    recognizer.update(&standing_pose(0.0));

    let started = recognizer.update(&pause_pose(300.0));
    assert_eq!(started.triggered & Action::Pause.mask(), 0);
    assert_eq!(started.active & Action::GAMEPLAY_MASK, 0);
    assert_eq!(started.triggered & Action::GAMEPLAY_MASK, 0);

    let almost = recognizer.update(&pause_pose(1_299.0));
    assert_eq!(almost.active & Action::Pause.mask(), 0);
    assert_eq!(almost.triggered & Action::Pause.mask(), 0);
    assert_eq!(almost.active & Action::GAMEPLAY_MASK, 0);
    assert_eq!(almost.triggered & Action::GAMEPLAY_MASK, 0);

    let held = recognizer.update(&pause_pose(1_300.0));
    assert_ne!(held.active & Action::Pause.mask(), 0);
    assert_eq!(held.triggered & Action::Pause.mask(), Action::Pause.mask());
    assert_eq!(recognizer.update(&pause_pose(1_600.0)).triggered, 0);
}

#[test]
fn transient_hands_and_overhead_clap_do_not_trigger_pause() {
    let mut recognizer = PoseRecognizer::default();
    recognizer.update(&standing_pose(0.0));
    recognizer.update(&pause_pose(300.0));
    assert_eq!(
        recognizer.update(&pause_pose(1_100.0)).triggered & Action::Pause.mask(),
        0
    );
    recognizer.update(&standing_pose(1_200.0));

    let mut overhead_clap = standing_pose(1_500.0);
    overhead_clap.keypoints[9].x = 0.49;
    overhead_clap.keypoints[9].y = 0.18;
    overhead_clap.keypoints[10].x = 0.51;
    overhead_clap.keypoints[10].y = 0.18;
    let clap = recognizer.update(&overhead_clap);
    assert_ne!(clap.triggered & Action::Clap.mask(), 0);
    overhead_clap.timestamp_ms = 3_000.0;
    let held_clap = recognizer.update(&overhead_clap);
    assert_eq!(held_clap.active & Action::Pause.mask(), 0);
    assert_eq!(held_clap.triggered & Action::Pause.mask(), 0);
}

#[test]
fn tracking_loss_resets_pause_dwell() {
    let mut recognizer = PoseRecognizer::default();
    recognizer.update(&pause_pose(0.0));
    recognizer.update(&pause_pose(800.0));
    recognizer.lose_tracking();

    assert_eq!(
        recognizer.update(&pause_pose(1_200.0)).triggered & Action::Pause.mask(),
        0
    );
    assert_eq!(
        recognizer.update(&pause_pose(2_200.0)).triggered & Action::Pause.mask(),
        Action::Pause.mask()
    );
}

#[test]
fn pause_control_action_is_not_scored_or_marked_as_a_miss() {
    let mut runtime = MotionRuntime::default();
    runtime.update(
        0.01,
        MotionInputFrame {
            fallback_actions: [Action::Pause.mask(), 0],
            evaluation_enabled: [true, false, false, false],
            ..MotionInputFrame::default()
        },
    );

    assert_eq!(runtime.game.scores[0], 0);
    assert_eq!(runtime.game.combos[0], 0);
    assert!(!runtime.players[0].hit);
    assert!(!runtime.players[0].miss);
    assert_eq!(runtime.local_evaluated_triggered()[0], Action::Pause.mask());
}

#[test]
fn pause_dwell_reserves_both_hands_from_gameplay_scoring() {
    let mut runtime = MotionRuntime::default();
    runtime.update(
        0.01,
        MotionInputFrame {
            local_poses: [Some(standing_pose(0.0)), None],
            evaluation_enabled: [true, false, false, false],
            custom_target: Some(Action::LeftUp.mask()),
            ..MotionInputFrame::default()
        },
    );

    for timestamp_ms in [300.0, 1_299.0, 1_300.0] {
        runtime.update(
            0.01,
            MotionInputFrame {
                local_poses: [Some(pause_pose(timestamp_ms)), None],
                evaluation_enabled: [true, false, false, false],
                custom_target: Some(Action::LeftUp.mask()),
                ..MotionInputFrame::default()
            },
        );
        assert_eq!(runtime.game.scores[0], 0);
        assert_eq!(runtime.game.combos[0], 0);
        assert!(!runtime.players[0].hit);
        assert!(!runtime.players[0].miss);
    }
    assert_eq!(runtime.local_evaluated_triggered()[0], Action::Pause.mask());
}

#[test]
fn scoring_is_bounded_and_deterministic() {
    let mut game = GameState::default();
    game.update(0.01, [Action::LeftUp.mask(), 0, 0, 0], None);
    assert_eq!(game.scores[0], 110);
    assert_eq!(game.combos[0], 1);
    game.update(0.01, [Action::MoveRight.mask(), 0, 0, 0], None);
    assert_eq!(game.combos[0], 0);
}

#[test]
fn mode_change_resets_match_state() {
    let mut game = GameState::default();
    game.scores[0] = 900;
    game.set_mode(GameMode::DuoGroove);
    assert_eq!(game.scores, [0; PLAYER_CAPACITY]);
    assert_eq!(game.mode, GameMode::DuoGroove);
}

#[test]
fn custom_target_is_used_for_scoring_and_display() {
    let mut game = GameState::default();
    game.update(
        0.01,
        [Action::Clap.mask(), 0, 0, 0],
        Some(Action::Clap.mask()),
    );
    assert_eq!(game.target, Action::Clap);
    assert_eq!(game.scores[0], 110);
}

#[test]
fn duo_groove_awards_one_sync_bonus_per_beat() {
    let mut game = GameState::new(GameMode::DuoGroove);
    game.update(
        0.01,
        [Action::LeftUp.mask(), Action::LeftUp.mask(), 0, 0],
        None,
    );
    assert_eq!(game.scores[0], 260);
    assert_eq!(game.scores[1], 260);
    game.update(
        0.01,
        [Action::LeftUp.mask(), Action::LeftUp.mask(), 0, 0],
        None,
    );
    assert_eq!(game.scores[0], 260);
    assert_eq!(game.scores[1], 260);
}

#[test]
fn motion_runtime_exposes_pose_action_and_hit_feedback() {
    let mut runtime = MotionRuntime::default();
    let pose = standing_pose(0.0);
    runtime.update(
        0.01,
        MotionInputFrame {
            local_poses: [Some(pose), None],
            fallback_actions: [Action::LeftUp.mask(), 0],
            ..MotionInputFrame::default()
        },
    );
    assert_eq!(runtime.players[0].pose, Some(pose));
    assert_eq!(runtime.local_triggered(), [Action::LeftUp.mask(), 0]);
    assert!(runtime.players[0].hit);
    assert!(!runtime.players[0].miss);
}

#[test]
fn motion_runtime_marks_wrong_actions_as_misses() {
    let mut runtime = MotionRuntime::default();
    runtime.update(
        0.01,
        MotionInputFrame {
            fallback_actions: [Action::MoveRight.mask(), 0],
            ..MotionInputFrame::default()
        },
    );
    assert!(runtime.players[0].miss);
    assert!(!runtime.players[0].hit);
}

#[test]
fn guide_only_input_never_scores_or_emits_feedback() {
    let mut runtime = MotionRuntime::default();
    runtime.update(
        0.01,
        MotionInputFrame {
            fallback_actions: [Action::LeftUp.mask(), 0],
            evaluation_enabled: [false; PLAYER_CAPACITY],
            ..MotionInputFrame::default()
        },
    );
    assert_eq!(runtime.game.scores, [0; PLAYER_CAPACITY]);
    assert_eq!(runtime.local_evaluated_triggered(), [0, 0]);
    assert!(!runtime.players[0].hit);
    assert!(!runtime.players[0].miss);
}

#[test]
fn recognizer_thresholds_are_configurable() {
    let mut recognizer = PoseRecognizer::new(RecognizerConfig {
        lateral_move_distance: 0.05,
        ..RecognizerConfig::default()
    });
    for sample in 0..60 {
        recognizer.update(&standing_pose(f64::from(sample) * 20.0));
    }
    let mut shifted = standing_pose(1_500.0);
    for keypoint in &mut shifted.keypoints {
        keypoint.x -= 0.08;
    }
    let sample = recognizer.update(&shifted);
    assert_ne!(sample.active & Action::MoveLeft.mask(), 0);
}

#[test]
fn two_players_calibrate_lateral_motion_independently() {
    let mut left_player = PoseRecognizer::default();
    let mut right_player = PoseRecognizer::default();
    let mut left_pose = standing_pose(0.0);
    let mut right_pose = standing_pose(0.0);
    for keypoint in &mut left_pose.keypoints {
        keypoint.x -= 0.22;
    }
    for keypoint in &mut right_pose.keypoints {
        keypoint.x += 0.22;
    }
    assert_eq!(
        left_player.update(&left_pose).active & Action::MoveLeft.mask(),
        0
    );
    assert_eq!(
        right_player.update(&right_pose).active & Action::MoveRight.mask(),
        0
    );
}

#[test]
fn reacquired_pose_can_trigger_an_action_again() {
    let mut runtime = MotionRuntime::default();
    let mut raised = standing_pose(300.0);
    raised.keypoints[9].y = 0.20;
    runtime.update(
        0.01,
        MotionInputFrame {
            local_poses: [Some(raised), None],
            ..MotionInputFrame::default()
        },
    );
    assert_ne!(runtime.players[0].triggered & Action::LeftUp.mask(), 0);
    runtime.update(0.01, MotionInputFrame::default());
    raised.timestamp_ms = 600.0;
    runtime.update(
        0.01,
        MotionInputFrame {
            local_poses: [Some(raised), None],
            ..MotionInputFrame::default()
        },
    );
    assert_ne!(runtime.players[0].triggered & Action::LeftUp.mask(), 0);
}

#[test]
fn runner_lane_changes_are_bounded() {
    let mut game = RunnerGame::new();
    game.phase = RunnerPhase::Running;
    for _ in 0..4 {
        game.update(
            0.01,
            [0; PLAYER_CAPACITY],
            [Action::MoveLeft.mask(), 0, 0, 0],
            [true, false, false, false],
        );
    }
    assert_eq!(game.players[0].lane, -1);
    for _ in 0..6 {
        game.update(
            0.01,
            [0; PLAYER_CAPACITY],
            [Action::MoveRight.mask(), 0, 0, 0],
            [true, false, false, false],
        );
    }
    assert_eq!(game.players[0].lane, 1);
}

#[test]
fn guide_only_runner_neither_scores_nor_loses_lives() {
    let mut game = RunnerGame::new();
    game.spawn(RunnerHazard::LaneBlock, 0);
    game.obstacles[0].as_mut().unwrap().distance = RUNNER_COLLISION_DISTANCE;
    game.update(
        0.02,
        [0; PLAYER_CAPACITY],
        [Action::Jump.mask(), 0, 0, 0],
        [false; PLAYER_CAPACITY],
    );
    assert_eq!(game.phase, RunnerPhase::Ready);
    assert_eq!(game.players[0].score, 0);
    assert_eq!(game.players[0].lives, 3);
}

#[test]
fn jump_avoids_hurdle_and_awards_combo() {
    let mut game = RunnerGame::new();
    game.phase = RunnerPhase::Running;
    game.spawn(RunnerHazard::Hurdle, 0);
    game.obstacles[0].as_mut().unwrap().distance = RUNNER_COLLISION_DISTANCE + 0.001;
    game.update(
        0.02,
        [Action::Jump.mask(), 0, 0, 0],
        [Action::Jump.mask(), 0, 0, 0],
        [true, false, false, false],
    );
    assert_eq!(game.players[0].lives, 3);
    assert_eq!(game.players[0].combo, 1);
    assert_eq!(game.players[0].best_combo, 1);
    assert_eq!(game.feedback[0], RunnerFeedback::Dodge);
}

#[test]
fn collision_costs_one_life_once() {
    let mut game = RunnerGame::new();
    game.phase = RunnerPhase::Running;
    game.spawn(RunnerHazard::LaneBlock, 0);
    game.obstacles[0].as_mut().unwrap().distance = RUNNER_COLLISION_DISTANCE + 0.001;
    let evaluation = [true, false, false, false];
    game.update(
        0.02,
        [0; PLAYER_CAPACITY],
        [Action::Clap.mask(), 0, 0, 0],
        evaluation,
    );
    assert_eq!(game.players[0].lives, 2);
    game.update(0.02, [0; PLAYER_CAPACITY], [0; PLAYER_CAPACITY], evaluation);
    assert_eq!(game.players[0].lives, 2);
}

#[test]
fn single_player_runner_counts_down_after_an_evaluated_motion() {
    let mut game = RunnerGame::with_mode(GameMode::MirrorBeat);
    let evaluation = [true, false, false, false];

    game.update(0.02, [0; PLAYER_CAPACITY], [0; PLAYER_CAPACITY], evaluation);
    assert_eq!(game.phase, RunnerPhase::Ready);

    game.update(
        0.02,
        [Action::LeftUp.mask(), 0, 0, 0],
        [Action::LeftUp.mask(), 0, 0, 0],
        evaluation,
    );
    assert_eq!(game.phase, RunnerPhase::Starting);
    assert_eq!(game.countdown_remaining, RUNNER_COUNTDOWN_SECONDS);
    assert_eq!(game.world_distance, 0.0);
    assert_eq!(game.beat_index, 0);

    for _ in 0..60 {
        game.update(
            0.05,
            [Action::MoveRight.mask(), 0, 0, 0],
            [Action::MoveRight.mask(), 0, 0, 0],
            evaluation,
        );
    }
    assert_eq!(game.phase, RunnerPhase::Running);
    assert_eq!(game.countdown_remaining, 0.0);
    assert_eq!(game.world_distance, 0.0);
    assert_eq!(game.players[0].lane, 0);

    game.update(0.05, [0; PLAYER_CAPACITY], [0; PLAYER_CAPACITY], evaluation);
    assert!(game.world_distance > 0.0);
}

#[test]
fn tracking_loss_during_initial_countdown_returns_to_ready() {
    let mut game = RunnerGame::with_mode(GameMode::MirrorBeat);
    game.update(
        0.02,
        [Action::LeftUp.mask(), 0, 0, 0],
        [Action::LeftUp.mask(), 0, 0, 0],
        [true, false, false, false],
    );
    assert_eq!(game.phase, RunnerPhase::Starting);

    game.update(
        0.05,
        [0; PLAYER_CAPACITY],
        [0; PLAYER_CAPACITY],
        [false; PLAYER_CAPACITY],
    );

    assert_eq!(game.phase, RunnerPhase::Ready);
    assert_eq!(game.ready_players, [false; PLAYER_CAPACITY]);
    assert_eq!(game.countdown_remaining, 0.0);
    assert_eq!(game.world_distance, 0.0);
    assert!(game.obstacles.iter().all(Option::is_none));
}

#[test]
fn duo_runner_requires_both_players_to_signal_readiness() {
    let mut game = RunnerGame::with_mode(GameMode::DuoGroove);
    let evaluation = [true, true, false, false];

    game.update(
        0.02,
        [Action::LeftUp.mask(), 0, 0, 0],
        [Action::LeftUp.mask(), 0, 0, 0],
        evaluation,
    );
    assert_eq!(game.phase, RunnerPhase::Ready);
    assert_eq!(game.ready_players, [true, false, false, false]);

    game.update(
        0.02,
        [0, Action::RightUp.mask(), 0, 0],
        [0, Action::RightUp.mask(), 0, 0],
        evaluation,
    );
    assert_eq!(game.phase, RunnerPhase::Starting);
    assert_eq!(game.countdown_remaining, RUNNER_COUNTDOWN_SECONDS);
}

#[test]
fn changing_runner_mode_resets_the_ready_session() {
    let mut game = RunnerGame::new();
    game.update(
        0.02,
        [Action::Clap.mask(), 0, 0, 0],
        [Action::Clap.mask(), 0, 0, 0],
        [true, false, false, false],
    );
    assert_eq!(game.phase, RunnerPhase::Starting);

    game.set_mode(GameMode::BeatStrike);
    assert_eq!(game.mode, GameMode::BeatStrike);
    assert_eq!(game.phase, RunnerPhase::Ready);
    assert_eq!(game.ready_players, [false; PLAYER_CAPACITY]);
}

#[test]
fn result_navigation_ignores_unevaluated_players_and_cycles_modes() {
    let mut game = RunnerGame::with_mode(GameMode::BeatStrike);
    game.phase = RunnerPhase::GameOver;
    game.players[0].evaluated = true;
    game.update(
        0.01,
        [0; PLAYER_CAPACITY],
        [0, Action::MoveRight.mask(), 0, 0],
        [true, false, false, false],
    );
    assert_eq!(game.result_selection, GameMode::BeatStrike);
    assert_eq!(game.take_next_run_request(), None);

    game.update(
        0.01,
        [0; PLAYER_CAPACITY],
        [Action::MoveLeft.mask(), Action::MoveRight.mask(), 0, 0],
        [true, true, false, false],
    );
    assert_eq!(game.result_selection, GameMode::BeatStrike);

    game.update(
        0.01,
        [0; PLAYER_CAPACITY],
        [Action::MoveRight.mask(), 0, 0, 0],
        [true, true, false, false],
    );
    assert_eq!(game.result_selection, GameMode::DuoGroove);
    game.update(
        0.01,
        [0; PLAYER_CAPACITY],
        [Action::MoveRight.mask(), 0, 0, 0],
        [true, true, false, false],
    );
    assert_eq!(game.result_selection, GameMode::MirrorBeat);
    game.update(
        0.01,
        [0; PLAYER_CAPACITY],
        [Action::MoveLeft.mask(), 0, 0, 0],
        [true, true, false, false],
    );
    assert_eq!(game.result_selection, GameMode::DuoGroove);
}

#[test]
fn result_navigation_skips_duo_without_two_local_evaluations() {
    let mut game = RunnerGame::with_mode(GameMode::BeatStrike);
    game.phase = RunnerPhase::GameOver;
    let one_player = [true, false, false, false];

    game.update(
        0.01,
        [0; PLAYER_CAPACITY],
        [Action::MoveRight.mask(), 0, 0, 0],
        one_player,
    );
    assert_eq!(game.result_selection, GameMode::MirrorBeat);

    game.update(
        0.01,
        [0; PLAYER_CAPACITY],
        [Action::MoveLeft.mask(), 0, 0, 0],
        one_player,
    );
    assert_eq!(game.result_selection, GameMode::BeatStrike);

    game.result_selection = GameMode::DuoGroove;
    game.update(
        0.01,
        [0; PLAYER_CAPACITY],
        [Action::Clap.mask(), 0, 0, 0],
        one_player,
    );
    assert_eq!(game.result_selection, GameMode::MirrorBeat);
    assert_eq!(game.take_next_run_request(), Some(GameMode::MirrorBeat));
}

#[test]
fn clap_requests_selected_run_and_preserves_best_score_when_started() {
    let mut game = RunnerGame::with_mode(GameMode::BeatStrike);
    game.phase = RunnerPhase::GameOver;
    game.result_selection = GameMode::BeatStrike;
    game.players[0].evaluated = true;
    game.players[0].score = 1_240;
    game.players[0].best_score = 1_240;
    game.players[0].lives = 0;
    game.update(
        0.01,
        [0; PLAYER_CAPACITY],
        [Action::Clap.mask(), 0, 0, 0],
        [true, false, false, false],
    );
    assert_eq!(game.phase, RunnerPhase::GameOver);
    assert_eq!(game.take_next_run_request(), Some(GameMode::BeatStrike));
    assert_eq!(game.take_next_run_request(), None);

    game.start_run(GameMode::BeatStrike, [true, false, false, false]);
    assert_eq!(game.mode, GameMode::BeatStrike);
    assert_eq!(game.phase, RunnerPhase::Starting);
    assert_eq!(game.countdown_remaining, RUNNER_COUNTDOWN_SECONDS);
    assert_eq!(game.players[0].lives, 3);
    assert_eq!(game.players[0].score, 0);
    assert_eq!(game.players[0].best_score, 1_240);
}

#[test]
fn duo_next_run_waits_for_two_evaluated_players() {
    let mut game = RunnerGame::with_mode(GameMode::MirrorBeat);
    game.players[0].best_score = 800;

    game.start_run(GameMode::DuoGroove, [true, false, false, false]);
    assert_eq!(game.mode, GameMode::DuoGroove);
    assert_eq!(game.phase, RunnerPhase::Ready);
    assert_eq!(game.players[0].best_score, 800);

    game.start_run(GameMode::DuoGroove, [true, true, false, false]);
    assert_eq!(game.phase, RunnerPhase::Starting);
    assert_eq!(game.countdown_remaining, RUNNER_COUNTDOWN_SECONDS);
}

#[test]
fn game_over_marks_a_new_personal_best() {
    let mut game = RunnerGame::new();
    game.phase = RunnerPhase::Running;
    game.players[0].score = 1_240;
    game.players[0].best_score = 900;
    game.players[0].lives = 0;
    game.update(
        0.01,
        [0; PLAYER_CAPACITY],
        [0; PLAYER_CAPACITY],
        [true, false, false, false],
    );

    assert_eq!(game.phase, RunnerPhase::GameOver);
    assert_eq!(game.result_outcome, Some(RunnerOutcome::EnergySpent));
    assert!(game.players[0].new_best);
    assert_eq!(game.players[0].best_score, 1_240);
}

#[test]
fn active_session_clock_freezes_outside_running() {
    let evaluation = [true, false, false, false];
    for phase in [
        RunnerPhase::Ready,
        RunnerPhase::Starting,
        RunnerPhase::Paused,
        RunnerPhase::PauseResuming,
        RunnerPhase::TrackingHold,
        RunnerPhase::Resuming,
        RunnerPhase::GameOver,
    ] {
        let mut game = RunnerGame::new();
        game.phase = phase;
        game.countdown_remaining = 1.0;
        game.run_elapsed_seconds = 12.0;
        game.update(0.05, [0; PLAYER_CAPACITY], [0; PLAYER_CAPACITY], evaluation);
        assert_eq!(game.run_elapsed_seconds, 12.0, "phase {phase:?}");
    }

    let mut running = RunnerGame::new();
    running.phase = RunnerPhase::Running;
    running.update(0.05, [0; PLAYER_CAPACITY], [0; PLAYER_CAPACITY], evaluation);
    assert_eq!(running.run_elapsed_seconds, 0.05);
    assert!(running.run_progress() > 0.0);
    assert!(running.run_remaining_seconds() < RUNNER_SESSION_SECONDS);
}

#[test]
fn completed_break_finishes_before_boundary_judgment_and_emits_once() {
    let mut game = RunnerGame::with_mode(GameMode::BeatStrike);
    game.phase = RunnerPhase::Running;
    game.run_elapsed_seconds = RUNNER_SESSION_SECONDS - 0.01;
    game.players[0].score = 420;
    game.players[0].combo = 4;
    game.spawn(RunnerHazard::LaneBlock, 0);
    game.obstacles[0].as_mut().unwrap().distance = RUNNER_COLLISION_DISTANCE;
    let distance = game.world_distance;
    let beat = game.beat_index;

    game.update(
        0.05,
        [Action::MoveRight.mask(), 0, 0, 0],
        [Action::MoveRight.mask(), 0, 0, 0],
        [true, false, false, false],
    );

    assert_eq!(game.phase, RunnerPhase::GameOver);
    assert_eq!(game.result_outcome, Some(RunnerOutcome::BreakComplete));
    assert_eq!(game.run_progress(), 1.0);
    assert_eq!(game.run_remaining_seconds(), 0.0);
    assert_eq!(game.world_distance, distance);
    assert_eq!(game.beat_index, beat);
    assert_eq!(game.players[0].score, 420);
    assert_eq!(game.players[0].combo, 4);
    assert_eq!(game.players[0].lives, 3);
    assert_eq!(game.players[0].lane, 0);
    assert!(game.obstacles.iter().all(Option::is_none));
    assert_eq!(
        game.take_result_event(),
        Some(RunnerResultEvent {
            mode: GameMode::BeatStrike,
            outcome: RunnerOutcome::BreakComplete,
        })
    );
    assert_eq!(game.take_result_event(), None);
}

#[test]
fn energy_spent_emits_a_distinct_positive_result_event() {
    let mut game = RunnerGame::with_mode(GameMode::MirrorBeat);
    game.phase = RunnerPhase::Running;
    game.players[0].lives = 0;

    game.update(
        0.01,
        [0; PLAYER_CAPACITY],
        [0; PLAYER_CAPACITY],
        [true, false, false, false],
    );

    assert_eq!(game.result_outcome, Some(RunnerOutcome::EnergySpent));
    assert_eq!(
        game.take_result_event(),
        Some(RunnerResultEvent {
            mode: GameMode::MirrorBeat,
            outcome: RunnerOutcome::EnergySpent,
        })
    );
}

#[test]
fn hands_free_replay_resets_session_progress_and_outcome() {
    let mut game = RunnerGame::with_mode(GameMode::MirrorBeat);
    game.phase = RunnerPhase::Running;
    game.run_elapsed_seconds = RUNNER_SESSION_SECONDS;
    game.finish_run(RunnerOutcome::BreakComplete);
    game.players[0].best_score = 900;

    game.start_run(GameMode::MirrorBeat, [true, false, false, false]);

    assert_eq!(game.phase, RunnerPhase::Starting);
    assert_eq!(game.run_progress(), 0.0);
    assert_eq!(game.run_remaining_seconds(), RUNNER_SESSION_SECONDS);
    assert_eq!(game.result_outcome, None);
    assert_eq!(game.take_result_event(), None);
    assert_eq!(game.players[0].best_score, 900);
}

#[test]
fn evaluated_pause_freezes_before_progression_and_clears_hazards() {
    let mut game = RunnerGame::new();
    game.phase = RunnerPhase::Running;
    game.players[0].score = 420;
    game.players[0].combo = 4;
    game.spawn(RunnerHazard::LaneBlock, 0);
    let distance = game.world_distance;
    let beat_index = game.beat_index;

    game.update(
        0.05,
        [Action::Jump.mask(), 0, 0, 0],
        [Action::Pause.mask() | Action::Jump.mask(), 0, 0, 0],
        [true, false, false, false],
    );

    assert_eq!(game.phase, RunnerPhase::Paused);
    assert_eq!(game.world_distance, distance);
    assert_eq!(game.beat_index, beat_index);
    assert_eq!(game.players[0].score, 420);
    assert_eq!(game.players[0].combo, 4);
    assert_eq!(game.players[0].lives, 3);
    assert_eq!(game.players[0].jump_time, 0.0);
    assert!(game.obstacles.iter().all(Option::is_none));
}

#[test]
fn intentional_pause_requires_evaluated_clap_then_counts_down_while_frozen() {
    let mut game = RunnerGame::new();
    game.phase = RunnerPhase::Paused;
    game.players[0].score = 420;
    let distance = game.world_distance;

    game.update(
        0.05,
        [0; PLAYER_CAPACITY],
        [Action::Clap.mask(), 0, 0, 0],
        [false; PLAYER_CAPACITY],
    );
    assert_eq!(game.phase, RunnerPhase::Paused);
    assert_eq!(game.ready_players, [false; PLAYER_CAPACITY]);

    game.update(
        0.05,
        [0; PLAYER_CAPACITY],
        [Action::MoveRight.mask(), 0, 0, 0],
        [true, false, false, false],
    );
    assert_eq!(game.phase, RunnerPhase::Paused);

    game.update(
        0.05,
        [0; PLAYER_CAPACITY],
        [Action::Clap.mask(), 0, 0, 0],
        [true, false, false, false],
    );
    assert_eq!(game.phase, RunnerPhase::PauseResuming);
    assert_eq!(game.countdown_remaining, RUNNER_COUNTDOWN_SECONDS);

    for _ in 0..60 {
        game.update(
            0.05,
            [Action::MoveRight.mask(), 0, 0, 0],
            [Action::MoveRight.mask(), 0, 0, 0],
            [true, false, false, false],
        );
    }
    assert_eq!(game.phase, RunnerPhase::Running);
    assert_eq!(game.world_distance, distance);
    assert_eq!(game.players[0].score, 420);
    assert_eq!(game.players[0].lane, 0);
}

#[test]
fn tracking_loss_during_intentional_resume_returns_to_paused() {
    let mut game = RunnerGame::new();
    game.phase = RunnerPhase::Paused;
    game.update(
        0.05,
        [0; PLAYER_CAPACITY],
        [Action::Clap.mask(), 0, 0, 0],
        [true, false, false, false],
    );
    assert_eq!(game.phase, RunnerPhase::PauseResuming);

    game.update(
        0.05,
        [0; PLAYER_CAPACITY],
        [0; PLAYER_CAPACITY],
        [false; PLAYER_CAPACITY],
    );

    assert_eq!(game.phase, RunnerPhase::Paused);
    assert_eq!(game.ready_players, [false; PLAYER_CAPACITY]);
    assert_eq!(game.countdown_remaining, 0.0);
}

#[test]
fn duo_intentional_pause_requires_both_players_to_clap() {
    let mut game = RunnerGame::with_mode(GameMode::DuoGroove);
    game.phase = RunnerPhase::Paused;
    let evaluation = [true, true, false, false];

    game.update(
        0.05,
        [0; PLAYER_CAPACITY],
        [Action::Clap.mask(), 0, 0, 0],
        evaluation,
    );
    assert_eq!(game.phase, RunnerPhase::Paused);
    assert_eq!(game.ready_players, [true, false, false, false]);

    game.update(
        0.05,
        [0; PLAYER_CAPACITY],
        [0, Action::MoveRight.mask(), 0, 0],
        evaluation,
    );
    assert_eq!(game.phase, RunnerPhase::Paused);

    game.update(
        0.05,
        [0; PLAYER_CAPACITY],
        [0, Action::Clap.mask(), 0, 0],
        evaluation,
    );
    assert_eq!(game.phase, RunnerPhase::PauseResuming);
    assert_eq!(game.ready_players, [true, true, false, false]);
}

#[test]
fn tracking_loss_freezes_running_state_and_clears_hazards() {
    let mut game = RunnerGame::new();
    game.phase = RunnerPhase::Running;
    game.players[0].score = 420;
    game.players[0].combo = 4;
    game.spawn(RunnerHazard::LaneBlock, 0);
    let distance = game.world_distance;
    let beat_index = game.beat_index;
    let beat_progress = game.beat_progress;

    game.update(
        0.05,
        [Action::Jump.mask(), 0, 0, 0],
        [Action::Jump.mask(), 0, 0, 0],
        [false; PLAYER_CAPACITY],
    );

    assert_eq!(game.phase, RunnerPhase::TrackingHold);
    assert_eq!(game.world_distance, distance);
    assert_eq!(game.beat_index, beat_index);
    assert_eq!(game.beat_progress, beat_progress);
    assert_eq!(game.players[0].score, 420);
    assert_eq!(game.players[0].combo, 4);
    assert_eq!(game.players[0].lives, 3);
    assert!(game.obstacles.iter().all(Option::is_none));
}

#[test]
fn tracking_hold_requires_evaluated_signal_then_counts_down_while_frozen() {
    let mut game = RunnerGame::new();
    game.phase = RunnerPhase::Running;
    game.update(
        0.05,
        [0; PLAYER_CAPACITY],
        [0; PLAYER_CAPACITY],
        [false; PLAYER_CAPACITY],
    );
    let distance = game.world_distance;

    game.update(
        0.05,
        [Action::Clap.mask(), 0, 0, 0],
        [Action::Clap.mask(), 0, 0, 0],
        [false; PLAYER_CAPACITY],
    );
    assert_eq!(game.phase, RunnerPhase::TrackingHold);
    assert_eq!(game.ready_players, [false; PLAYER_CAPACITY]);

    game.update(
        0.05,
        [Action::Clap.mask(), 0, 0, 0],
        [Action::Clap.mask(), 0, 0, 0],
        [true, false, false, false],
    );
    assert_eq!(game.phase, RunnerPhase::Resuming);
    assert_eq!(game.countdown_remaining, RUNNER_COUNTDOWN_SECONDS);
    assert_eq!(game.world_distance, distance);

    for _ in 0..60 {
        game.update(
            0.05,
            [0; PLAYER_CAPACITY],
            [0; PLAYER_CAPACITY],
            [true, false, false, false],
        );
    }
    assert_eq!(game.phase, RunnerPhase::Running);
    assert_eq!(game.countdown_remaining, 0.0);
    assert_eq!(game.world_distance, distance);
}

#[test]
fn tracking_loss_during_resume_restarts_the_hold() {
    let mut game = RunnerGame::new();
    game.phase = RunnerPhase::TrackingHold;
    game.update(
        0.05,
        [Action::LeftUp.mask(), 0, 0, 0],
        [Action::LeftUp.mask(), 0, 0, 0],
        [true, false, false, false],
    );
    assert_eq!(game.phase, RunnerPhase::Resuming);

    game.update(
        0.05,
        [0; PLAYER_CAPACITY],
        [0; PLAYER_CAPACITY],
        [false; PLAYER_CAPACITY],
    );

    assert_eq!(game.phase, RunnerPhase::TrackingHold);
    assert_eq!(game.ready_players, [false; PLAYER_CAPACITY]);
    assert_eq!(game.countdown_remaining, 0.0);
}

#[test]
fn duo_tracking_hold_requires_both_evaluated_players_to_signal() {
    let mut game = RunnerGame::with_mode(GameMode::DuoGroove);
    game.phase = RunnerPhase::TrackingHold;
    let evaluation = [true, true, false, false];

    game.update(
        0.05,
        [Action::LeftUp.mask(), 0, 0, 0],
        [Action::LeftUp.mask(), 0, 0, 0],
        evaluation,
    );
    assert_eq!(game.phase, RunnerPhase::TrackingHold);
    assert_eq!(game.ready_players, [true, false, false, false]);

    game.update(
        0.05,
        [0, Action::RightUp.mask(), 0, 0],
        [0, Action::RightUp.mask(), 0, 0],
        evaluation,
    );
    assert_eq!(game.phase, RunnerPhase::Resuming);
    assert_eq!(game.ready_players, [true, true, false, false]);
}

#[test]
fn breath_intensity_starts_at_zero_and_peaks_at_one() {
    let mut game = RunnerGame::new();
    game.phase = RunnerPhase::Running;
    assert_eq!(game.breath_intensity(), 0.0);

    // Mid-rise should be between 0 and 1
    game.run_elapsed_seconds = 27.5;
    let mid_rise = game.breath_intensity();
    assert!(mid_rise > 0.0 && mid_rise < 1.0, "mid-rise = {mid_rise}");

    // Peak holds at 1.0
    game.run_elapsed_seconds = 60.0;
    assert_eq!(game.breath_intensity(), 1.0);
    game.run_elapsed_seconds = 74.9;
    assert_eq!(game.breath_intensity(), 1.0);
}

#[test]
fn breath_intensity_release_eases_toward_floor() {
    let mut game = RunnerGame::new();
    game.phase = RunnerPhase::Running;

    // Start of release
    game.run_elapsed_seconds = 75.0;
    assert_eq!(game.breath_intensity(), 1.0);

    // End of release approaches 0.3
    game.run_elapsed_seconds = 89.9;
    let tail = game.breath_intensity();
    assert!((0.3..0.4).contains(&tail), "tail = {tail}");
}

#[test]
fn breath_intensity_is_monotonically_nondecreasing_during_rise() {
    let mut game = RunnerGame::new();
    game.phase = RunnerPhase::Running;
    let mut previous = 0.0;
    for step in 0..=55 {
        game.run_elapsed_seconds = step as f32;
        let current = game.breath_intensity();
        assert!(current >= previous, "step {step}: {current} < {previous}");
        previous = current;
    }
}
