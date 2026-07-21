#[cfg(target_arch = "wasm32")]
use brainbreak_core::Keypoint;
use brainbreak_core::{Action, GameMode, GameState, PLAYER_CAPACITY, PoseFrame, PoseRecognizer};
use macroquad::prelude::*;

#[cfg(target_arch = "wasm32")]
const POSE_BUFFER_BYTES: usize = 512;

#[cfg(target_arch = "wasm32")]
unsafe extern "C" {
    fn bb_copy_pose(destination: *mut u8, capacity: u32) -> u32;
    fn bb_take_remote_actions(player: u32) -> u32;
    fn bb_send_local_action(player: u32, mask: u32);
    fn bb_network_status() -> u32;
    fn bb_custom_target(beat_index: u32) -> u32;
    fn bb_take_gamepad_actions(player: u32) -> u32;
}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn brainbreak_bridge_crate_version() -> u32 {
    1
}

fn window_conf() -> Conf {
    Conf {
        window_title: "BrainBreak Motion Party".to_owned(),
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

fn keyboard_actions() -> u32 {
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
    if is_key_pressed(KeyCode::Q) {
        mask |= Action::LeftUp.mask();
    }
    if is_key_pressed(KeyCode::E) {
        mask |= Action::RightUp.mask();
    }
    if is_key_pressed(KeyCode::C) || is_key_pressed(KeyCode::Enter) {
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

fn custom_target(_beat_index: u32) -> Option<u32> {
    #[cfg(target_arch = "wasm32")]
    {
        let mask = unsafe { bb_custom_target(_beat_index) };
        return (mask != 0).then_some(mask);
    }
    #[cfg(not(target_arch = "wasm32"))]
    None
}

fn draw_round_rect(x: f32, y: f32, w: f32, h: f32, color: Color) {
    draw_rectangle(x + 16.0, y, w - 32.0, h, color);
    draw_rectangle(x, y + 16.0, w, h - 32.0, color);
    draw_circle(x + 16.0, y + 16.0, 16.0, color);
    draw_circle(x + w - 16.0, y + 16.0, 16.0, color);
    draw_circle(x + 16.0, y + h - 16.0, 16.0, color);
    draw_circle(x + w - 16.0, y + h - 16.0, 16.0, color);
}

fn mode_color(mode: GameMode) -> Color {
    match mode {
        GameMode::MirrorBeat => Color::from_rgba(131, 92, 246, 255),
        GameMode::BeatStrike => Color::from_rgba(236, 72, 153, 255),
        GameMode::DuoGroove => Color::from_rgba(16, 185, 129, 255),
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = GameState::default();
    let mut recognizers = [PoseRecognizer::default(), PoseRecognizer::default()];

    loop {
        let dt = get_frame_time().min(0.05);
        if is_key_pressed(KeyCode::Key1) {
            game.set_mode(GameMode::MirrorBeat);
        } else if is_key_pressed(KeyCode::Key2) {
            game.set_mode(GameMode::BeatStrike);
        } else if is_key_pressed(KeyCode::Key3) {
            game.set_mode(GameMode::DuoGroove);
        } else if is_key_pressed(KeyCode::Tab) {
            game.set_mode(game.mode.next());
        }

        let mut actions = [0_u32; PLAYER_CAPACITY];
        let poses = browser_poses();
        let player_count = poses.len().max(1);
        for (index, pose) in poses.iter().take(2).enumerate() {
            actions[index] |= recognizers[index].update(pose).triggered;
        }
        actions[0] |= keyboard_actions();

        #[cfg(target_arch = "wasm32")]
        unsafe {
            actions[0] |= bb_take_gamepad_actions(0);
            actions[1] |= bb_take_gamepad_actions(1);
            for (player, mask) in actions.iter().take(2).copied().enumerate() {
                if mask != 0 {
                    bb_send_local_action(player as u32, mask);
                }
            }
            actions[2] |= bb_take_remote_actions(0);
            actions[3] |= bb_take_remote_actions(1);
        }

        game.update(dt, actions, custom_target(game.beat_index));

        let width = screen_width();
        let height = screen_height();
        let accent = mode_color(game.mode);
        clear_background(Color::from_rgba(8, 12, 28, 255));
        draw_circle(
            width * 0.15,
            height * 0.22,
            width * 0.22,
            Color::new(accent.r, accent.g, accent.b, 0.12),
        );
        draw_circle(
            width * 0.88,
            height * 0.82,
            width * 0.28,
            Color::new(0.12, 0.75, 0.95, 0.10),
        );

        let margin = (width * 0.045).max(24.0);
        draw_text("BRAINBREAK", margin, 58.0, 34.0, WHITE);
        draw_text(game.mode.title(), margin, 94.0, 22.0, accent);
        let status = match network_status() {
            3 => "HOST ONLINE",
            2 => "P2P CONNECTED",
            1 => "CONNECTING",
            _ => "LOCAL PARTY",
        };
        let status_width = measure_text(status, None, 18, 1.0).width;
        draw_text(
            status,
            width - margin - status_width,
            54.0,
            18.0,
            Color::from_rgba(148, 163, 184, 255),
        );

        let cue_w = (width * 0.56).min(720.0);
        let cue_h = (height * 0.38).min(310.0);
        let cue_x = (width - cue_w) * 0.5;
        let cue_y = height * 0.18;
        draw_round_rect(
            cue_x,
            cue_y,
            cue_w,
            cue_h,
            Color::from_rgba(20, 28, 55, 235),
        );
        draw_rectangle(
            cue_x + 20.0,
            cue_y + cue_h - 18.0,
            (cue_w - 40.0) * game.beat_progress,
            6.0,
            accent,
        );
        draw_rectangle_lines(
            cue_x + 20.0,
            cue_y + cue_h - 18.0,
            cue_w - 40.0,
            6.0,
            1.0,
            Color::from_rgba(71, 85, 105, 255),
        );
        let ready = if game.beat_progress < 0.55 {
            "GET READY"
        } else {
            "MOVE!"
        };
        let ready_size = measure_text(ready, None, 24, 1.0);
        draw_text(
            ready,
            width * 0.5 - ready_size.width * 0.5,
            cue_y + 62.0,
            24.0,
            Color::from_rgba(148, 163, 184, 255),
        );
        let target = game.target.label();
        let font_size = if width < 680.0 { 42 } else { 64 };
        let target_size = measure_text(target, None, font_size, 1.0);
        draw_text(
            target,
            width * 0.5 - target_size.width * 0.5,
            cue_y + cue_h * 0.58,
            font_size as f32,
            WHITE,
        );

        let displayed_players = if network_status() >= 2 {
            4
        } else {
            player_count.max(2)
        };
        let gap = 12.0;
        let card_w = ((width - margin * 2.0 - gap * (displayed_players as f32 - 1.0))
            / displayed_players as f32)
            .min(260.0);
        let cards_total =
            card_w * displayed_players as f32 + gap * (displayed_players as f32 - 1.0);
        let start_x = (width - cards_total) * 0.5;
        let card_y = cue_y + cue_h + 28.0;
        for player in 0..displayed_players {
            let x = start_x + player as f32 * (card_w + gap);
            draw_round_rect(x, card_y, card_w, 104.0, Color::from_rgba(15, 23, 42, 245));
            draw_text(
                format!("P{}", player + 1),
                x + 16.0,
                card_y + 30.0,
                18.0,
                Color::from_rgba(148, 163, 184, 255),
            );
            draw_text(
                game.scores[player].to_string(),
                x + 16.0,
                card_y + 67.0,
                34.0,
                WHITE,
            );
            draw_text(
                format!("x{}", game.combos[player]),
                x + card_w - 54.0,
                card_y + 66.0,
                18.0,
                accent,
            );
        }

        let help =
            "1/2/3 MODE  •  CAMERA / KEYBOARD / GAMEPAD  •  Q/E HANDS  •  SPACE JUMP  •  C CLAP";
        let help_size = measure_text(help, None, 15, 1.0);
        draw_text(
            help,
            width * 0.5 - help_size.width * 0.5,
            height - 30.0,
            15.0,
            Color::from_rgba(100, 116, 139, 255),
        );

        next_frame().await;
    }
}
