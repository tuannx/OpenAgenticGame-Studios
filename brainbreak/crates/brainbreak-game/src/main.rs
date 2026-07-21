#[cfg(target_arch = "wasm32")]
use brainbreak_core::Keypoint;
use brainbreak_core::{
    Action, MotionInputFrame, MotionRuntime, PoseFrame, RunnerFeedback, RunnerGame, RunnerPhase,
};
use macroquad::prelude::*;

mod visuals;
use visuals::{AudioVisual, RunnerStage, draw_round_panel};

#[cfg(target_arch = "wasm32")]
const POSE_BUFFER_BYTES: usize = 512;

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn bb_copy_pose(destination: *mut u8, capacity: u32) -> u32;
    fn bb_take_remote_actions(player: u32) -> u32;
    fn bb_send_local_action(player: u32, mask: u32);
    fn bb_network_status() -> u32;
    fn bb_take_gamepad_actions(player: u32) -> u32;
    fn bb_evaluation_enabled(player: u32) -> u32;
    fn bb_audio_beat_phase() -> f32;
    fn bb_audio_pulse() -> f32;
    fn bb_audio_energy() -> f32;
    fn bb_audio_playing() -> u32;
    fn bb_reduce_motion() -> u32;
    fn bb_play_feedback(kind: u32);
}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn brainbreak_bridge_crate_version() -> u32 {
    3
}

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

#[cfg(target_arch = "wasm32")]
fn read_u32(bytes: &[u8], cursor: &mut usize) -> Option<u32> {
    let value = u32::from_le_bytes(bytes.get(*cursor..*cursor + 4)?.try_into().ok()?);
    *cursor += 4;
    Some(value)
}

#[cfg(target_arch = "wasm32")]
fn read_f32(bytes: &[u8], cursor: &mut usize) -> Option<f32> {
    let value = f32::from_le_bytes(bytes.get(*cursor..*cursor + 4)?.try_into().ok()?);
    *cursor += 4;
    Some(value)
}

#[cfg(target_arch = "wasm32")]
fn read_f64(bytes: &[u8], cursor: &mut usize) -> Option<f64> {
    let value = f64::from_le_bytes(bytes.get(*cursor..*cursor + 8)?.try_into().ok()?);
    *cursor += 8;
    Some(value)
}

#[cfg(target_arch = "wasm32")]
fn browser_poses() -> Vec<PoseFrame> {
    let mut bytes = [0_u8; POSE_BUFFER_BYTES];
    let written = unsafe { bb_copy_pose(bytes.as_mut_ptr(), bytes.len() as u32) } as usize;
    if written < 16 || written > bytes.len() {
        return Vec::new();
    }
    let mut cursor = 0;
    if read_u32(&bytes[..written], &mut cursor) != Some(1) {
        return Vec::new();
    }
    let count = read_u32(&bytes[..written], &mut cursor).unwrap_or(0).min(2);
    let timestamp_ms = read_f64(&bytes[..written], &mut cursor).unwrap_or(0.0);
    let mut poses = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let Some(tracked_id) = read_u32(&bytes[..written], &mut cursor) else {
            break;
        };
        let Some(quality) = read_f32(&bytes[..written], &mut cursor) else {
            break;
        };
        let mut keypoints = [Keypoint::default(); 17];
        let mut complete = true;
        for keypoint in &mut keypoints {
            let Some(x) = read_f32(&bytes[..written], &mut cursor) else {
                complete = false;
                break;
            };
            let Some(y) = read_f32(&bytes[..written], &mut cursor) else {
                complete = false;
                break;
            };
            let Some(confidence) = read_f32(&bytes[..written], &mut cursor) else {
                complete = false;
                break;
            };
            *keypoint = Keypoint { x, y, confidence };
        }
        if complete {
            poses.push(PoseFrame {
                tracked_id,
                quality,
                timestamp_ms,
                keypoints,
            });
        }
    }
    poses
}

#[cfg(not(target_arch = "wasm32"))]
fn browser_poses() -> Vec<PoseFrame> {
    Vec::new()
}

fn keyboard_actions(restart: bool) -> u32 {
    let mut mask = 0;
    if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
        mask |= Action::MoveLeft.mask();
    }
    if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
        mask |= Action::MoveRight.mask();
    }
    if is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::W) {
        mask |= Action::Jump.mask();
    }
    if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
        mask |= Action::Squat.mask();
    }
    if is_key_pressed(KeyCode::C)
        || is_key_pressed(KeyCode::Enter)
        || (restart && is_key_pressed(KeyCode::R))
    {
        mask |= Action::Clap.mask();
    }
    mask
}

fn network_status() -> u32 {
    #[cfg(target_arch = "wasm32")]
    return unsafe { bb_network_status() };
    #[cfg(not(target_arch = "wasm32"))]
    return 0;
}

fn evaluation_enabled(_player: u32) -> bool {
    #[cfg(target_arch = "wasm32")]
    return unsafe { bb_evaluation_enabled(_player) != 0 };
    #[cfg(not(target_arch = "wasm32"))]
    false
}

fn reduce_motion() -> bool {
    #[cfg(target_arch = "wasm32")]
    return unsafe { bb_reduce_motion() != 0 };
    #[cfg(not(target_arch = "wasm32"))]
    false
}

fn audio_visual() -> AudioVisual {
    #[cfg(target_arch = "wasm32")]
    return unsafe {
        AudioVisual {
            phase: bb_audio_beat_phase().clamp(0.0, 1.0),
            pulse: bb_audio_pulse().clamp(0.0, 1.0),
            energy: bb_audio_energy().clamp(0.0, 1.0),
            playing: bb_audio_playing() != 0,
        }
    };
    #[cfg(not(target_arch = "wasm32"))]
    {
        let phase = ((get_time() as f32) * 126.0 / 60.0).fract();
        let distance = phase.min(1.0 - phase);
        AudioVisual {
            phase,
            pulse: (-distance * 13.0).exp(),
            energy: 0.18,
            playing: false,
        }
    }
}

fn player_color(player: usize) -> Color {
    match player {
        0 => Color::from_rgba(34, 211, 238, 255),
        1 => Color::from_rgba(196, 181, 253, 255),
        2 => Color::from_rgba(251, 191, 36, 255),
        _ => Color::from_rgba(52, 211, 153, 255),
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut motion = MotionRuntime::default();
    let mut runner = RunnerGame::new();
    let mut stage = RunnerStage::default();

    loop {
        let dt = get_frame_time().min(0.05);
        let poses = browser_poses();
        let network = network_status();
        let local_evaluation = [evaluation_enabled(0), evaluation_enabled(1)];
        let guide_only = !local_evaluation.into_iter().any(|enabled| enabled);
        #[cfg(target_arch = "wasm32")]
        let (fallback_actions, remote_actions) = unsafe {
            let mut fallback = [keyboard_actions(runner.phase == RunnerPhase::GameOver), 0];
            fallback[0] |= bb_take_gamepad_actions(0);
            fallback[1] |= bb_take_gamepad_actions(1);
            (
                fallback,
                [bb_take_remote_actions(0), bb_take_remote_actions(1)],
            )
        };
        #[cfg(not(target_arch = "wasm32"))]
        let (fallback_actions, remote_actions) = (
            [keyboard_actions(runner.phase == RunnerPhase::GameOver), 0],
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
        unsafe {
            for (player, mask) in motion.local_evaluated_triggered().into_iter().enumerate() {
                if mask != 0 {
                    bb_send_local_action(player as u32, mask);
                }
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
        runner.update(dt, active, triggered, evaluation);
        #[cfg(target_arch = "wasm32")]
        unsafe {
            for feedback in runner.feedback {
                let kind = match feedback {
                    RunnerFeedback::None => 0,
                    RunnerFeedback::Dodge => 1,
                    RunnerFeedback::Crash => 2,
                    RunnerFeedback::BeatPickup => 3,
                };
                if kind != 0 {
                    bb_play_feedback(kind);
                }
            }
        }
        stage.update(dt, &runner);

        let width = screen_width();
        let height = screen_height();
        let audio = audio_visual();
        let reduced = reduce_motion();
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
        let world_bounds = Rect::new(shake.x, shake.y, width, height);
        stage.draw(&runner, &motion, world_bounds, audio, reduced);
        draw_header(width, audio, guide_only);
        draw_player_hud(width, height, &runner, network);
        draw_phase_overlay(width, height, &runner, guide_only, poses.len());
        draw_beat_meter(width, height, audio);

        next_frame().await;
    }
}

fn draw_header(width: f32, audio: AudioVisual, guide_only: bool) {
    let margin = (width * 0.03).max(18.0);
    draw_text(
        "NEON",
        margin,
        36.0,
        18.0,
        Color::from_rgba(103, 232, 249, 255),
    );
    draw_text("BEAT RUNNER", margin, 65.0, 29.0, WHITE);
    let state = if guide_only {
        "GUIDANCE ONLY"
    } else if audio.playing {
        "126 BPM / LIVE"
    } else {
        "126 BPM / VISUAL CLOCK"
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

fn draw_player_hud(width: f32, height: f32, runner: &RunnerGame, network: u32) {
    let displayed = if network >= 2 {
        4
    } else {
        runner
            .players
            .iter()
            .take(2)
            .filter(|player| player.evaluated)
            .count()
            .max(1)
    };
    let card_width = if width < 700.0 { 142.0 } else { 174.0 };
    let gap = 10.0;
    let total = displayed as f32 * card_width + (displayed.saturating_sub(1)) as f32 * gap;
    let start_x = (width - total) * 0.5;
    let y = height - if height < 620.0 { 72.0 } else { 86.0 };
    for player in 0..displayed {
        let state = runner.players[player];
        let x = start_x + player as f32 * (card_width + gap);
        draw_round_panel(
            Rect::new(x, y, card_width, 58.0),
            Color::new(0.025, 0.035, 0.11, 0.88),
        );
        draw_rectangle(x, y, 4.0, 58.0, player_color(player));
        draw_text(
            format!("P{}  {:05}", player + 1, state.score),
            x + 14.0,
            y + 24.0,
            17.0,
            if state.evaluated {
                WHITE
            } else {
                Color::from_rgba(148, 163, 184, 255)
            },
        );
        let detail = if state.evaluated {
            format!("LIVES {}   COMBO x{}", state.lives, state.combo)
        } else {
            "NO CAMERA / NO SCORE".to_owned()
        };
        draw_text(&detail, x + 14.0, y + 45.0, 11.0, player_color(player));
    }
}

fn draw_phase_overlay(
    width: f32,
    height: f32,
    runner: &RunnerGame,
    guide_only: bool,
    tracked_players: usize,
) {
    if runner.phase == RunnerPhase::Running && !guide_only {
        return;
    }
    let panel_width = width.min(560.0) * 0.86;
    let panel_height = if runner.phase == RunnerPhase::GameOver {
        188.0
    } else {
        132.0
    };
    let panel = Rect::new(
        (width - panel_width) * 0.5,
        height * 0.48 - panel_height * 0.5,
        panel_width,
        panel_height,
    );
    draw_round_panel(panel, Color::new(0.015, 0.02, 0.09, 0.9));
    draw_rectangle_lines(
        panel.x,
        panel.y,
        panel.w,
        panel.h,
        2.0,
        Color::new(0.45, 0.32, 0.95, 0.62),
    );
    let (title, subtitle, color) = if runner.phase == RunnerPhase::GameOver {
        let score = runner
            .players
            .iter()
            .map(|player| player.score)
            .max()
            .unwrap_or(0);
        (
            "RUN COMPLETE".to_owned(),
            format!("SCORE {score}  /  CLAP TO RUN AGAIN"),
            Color::from_rgba(251, 113, 133, 255),
        )
    } else if guide_only {
        (
            "CAMERA REQUIRED TO SCORE".to_owned(),
            "Enable motion to steer, jump, squat and clap".to_owned(),
            Color::from_rgba(253, 230, 138, 255),
        )
    } else if tracked_players == 0 {
        (
            "STEP INTO THE FRAME".to_owned(),
            "Your neon runner mirrors your body".to_owned(),
            Color::from_rgba(103, 232, 249, 255),
        )
    } else {
        (
            "LOCKING ON".to_owned(),
            "Stand tall - first obstacle incoming".to_owned(),
            Color::from_rgba(103, 232, 249, 255),
        )
    };
    let title_size = measure_text(&title, None, 30, 1.0);
    draw_text(
        &title,
        panel.x + (panel.w - title_size.width) * 0.5,
        panel.y + 52.0,
        30.0,
        color,
    );
    let subtitle_size = measure_text(&subtitle, None, 16, 1.0);
    draw_text(
        &subtitle,
        panel.x + (panel.w - subtitle_size.width) * 0.5,
        panel.y + 84.0,
        16.0,
        Color::from_rgba(226, 232, 240, 255),
    );
    if runner.phase == RunnerPhase::GameOver {
        let hint = "LEFT/RIGHT = LEAN  /  JUMP  /  SQUAT  /  CLAP";
        let hint_size = measure_text(hint, None, 12, 1.0);
        draw_text(
            hint,
            panel.x + (panel.w - hint_size.width) * 0.5,
            panel.y + 126.0,
            12.0,
            Color::from_rgba(148, 163, 184, 255),
        );
    }
}

fn draw_beat_meter(width: f32, height: f32, audio: AudioVisual) {
    let meter_width = width.min(420.0) * 0.7;
    let x = (width - meter_width) * 0.5;
    let y = if height < 620.0 { 76.0 } else { 82.0 };
    draw_rectangle(x, y, meter_width, 3.0, Color::new(1.0, 1.0, 1.0, 0.12));
    draw_rectangle(
        x,
        y,
        meter_width * audio.phase,
        3.0 + audio.energy * 3.0,
        Color::new(0.25 + audio.energy * 0.4, 0.85, 1.0, 0.9),
    );
}
