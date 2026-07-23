#[cfg(target_arch = "wasm32")]
use brainbreak_core::Keypoint;
use brainbreak_core::{GameMode, PoseFrame};
use macroquad::prelude::*;

use crate::visuals::AudioVisual;

#[cfg(target_arch = "wasm32")]
pub const POSE_BUFFER_BYTES: usize = 512;

#[cfg(target_arch = "wasm32")]
const MIRROR_ART_PATH: &str = "/assets/game_mode_mirror.webp";
#[cfg(target_arch = "wasm32")]
const STRIKE_ART_PATH: &str = "/assets/game_mode_strike.webp";
#[cfg(target_arch = "wasm32")]
const DUO_ART_PATH: &str = "/assets/game_mode_duo.webp";

#[cfg(not(target_arch = "wasm32"))]
pub const MIRROR_ART_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../web/public/assets/game_mode_mirror.webp"
);
#[cfg(not(target_arch = "wasm32"))]
pub const STRIKE_ART_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../web/public/assets/game_mode_strike.webp"
);
#[cfg(not(target_arch = "wasm32"))]
pub const DUO_ART_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../web/public/assets/game_mode_duo.webp"
);

pub const MODE_ART_MAX_ENCODED_BYTES: usize = 512 * 1024;
const MODE_ART_MAX_DIMENSION: u32 = 1024;
const MODE_ART_MAX_DECODE_BYTES: u64 = 8 * 1024 * 1024;

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
    fn bb_reduce_motion() -> u32;
    fn bb_play_feedback(kind: u32, combo: u32);
    fn bb_game_mode() -> u32;
    fn bb_set_game_mode(mode: u32);
    fn bb_record_run_outcome(mode: u32, outcome: u32);
    fn bb_copy_game_config(destination: *mut u8, capacity: u32) -> u32;
    fn bb_supernova_event(kind: u32, value: u32);
}

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub extern "C" fn brainbreak_bridge_crate_version() -> u32 {
    7
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
pub fn browser_poses() -> Vec<PoseFrame> {
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
pub fn browser_poses() -> Vec<PoseFrame> {
    Vec::new()
}

pub fn keyboard_actions(restart: bool) -> u32 {
    use brainbreak_core::Action;
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
    if is_key_pressed(KeyCode::P) {
        mask |= Action::Pause.mask();
    }
    mask
}

pub fn network_status() -> u32 {
    #[cfg(target_arch = "wasm32")]
    return unsafe { bb_network_status() };
    #[cfg(not(target_arch = "wasm32"))]
    return 0;
}

pub fn evaluation_enabled(_player: u32) -> bool {
    #[cfg(target_arch = "wasm32")]
    return unsafe { bb_evaluation_enabled(_player) != 0 };
    #[cfg(not(target_arch = "wasm32"))]
    false
}

pub fn reduce_motion() -> bool {
    #[cfg(target_arch = "wasm32")]
    return unsafe { bb_reduce_motion() != 0 };
    #[cfg(not(target_arch = "wasm32"))]
    false
}

pub fn audio_visual() -> AudioVisual {
    #[cfg(target_arch = "wasm32")]
    return unsafe {
        AudioVisual {
            phase: bb_audio_beat_phase().clamp(0.0, 1.0),
            pulse: bb_audio_pulse().clamp(0.0, 1.0),
            energy: bb_audio_energy().clamp(0.0, 1.0),
        }
    };
    #[cfg(not(target_arch = "wasm32"))]
    {
        let phase = ((get_time() as f32) * 140.0 / 60.0).fract();
        let distance = phase.min(1.0 - phase);
        AudioVisual {
            phase,
            pulse: (-distance * 13.0).exp(),
            energy: 0.18,
        }
    }
}

pub fn player_color(player: usize) -> Color {
    match player {
        0 => Color::from_rgba(34, 211, 238, 255),
        1 => Color::from_rgba(196, 181, 253, 255),
        2 => Color::from_rgba(251, 191, 36, 255),
        _ => Color::from_rgba(52, 211, 153, 255),
    }
}

pub fn current_game_mode() -> GameMode {
    #[cfg(target_arch = "wasm32")]
    return match unsafe { bb_game_mode() } {
        1 => GameMode::BeatStrike,
        2 => GameMode::DuoGroove,
        _ => GameMode::MirrorBeat,
    };
    #[cfg(not(target_arch = "wasm32"))]
    GameMode::MirrorBeat
}

/// Raw numeric game mode from the JS bridge (0=Mirror, 1=Strike, 2=Duo, 3=Supernova, 4=Custom).
#[cfg(target_arch = "wasm32")]
pub fn raw_game_mode() -> u32 {
    unsafe { bb_game_mode() }
}

/// Maximum bytes for a game config JSON payload.
#[cfg(target_arch = "wasm32")]
pub const GAME_CONFIG_MAX_BYTES: usize = 4096;

/// Load the active custom game config from the JS bridge (WASM only).
#[cfg(target_arch = "wasm32")]
pub fn load_game_config() -> Option<brainbreak_core::GameConfig> {
    let mut buffer = [0u8; GAME_CONFIG_MAX_BYTES];
    let written = unsafe { bb_copy_game_config(buffer.as_mut_ptr(), buffer.len() as u32) };
    if written == 0 {
        return None;
    }
    brainbreak_core::GameConfig::from_json(&buffer[..written as usize])
}

#[cfg(target_arch = "wasm32")]
pub const fn game_mode_value(mode: GameMode) -> u32 {
    match mode {
        GameMode::MirrorBeat => 0,
        GameMode::BeatStrike => 1,
        GameMode::DuoGroove => 2,
    }
}

/// Notify the JS party director of a Supernova phase event (music/voice cues).
/// No-op on native builds (no JS bridge present).
pub fn supernova_event(kind: u32, value: u32) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        bb_supernova_event(kind, value);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (kind, value);
    }
}

/// Quantized motion SFX with combo pitch. No-op on native builds.
pub fn play_feedback(kind: u32, combo: u32) {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        bb_play_feedback(kind, combo);
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (kind, combo);
    }
}

// --- WASM-only re-exports for the game loop ---

#[cfg(target_arch = "wasm32")]
pub fn take_gamepad_actions(player: u32) -> u32 {
    unsafe { bb_take_gamepad_actions(player) }
}

#[cfg(target_arch = "wasm32")]
pub fn take_remote_actions(player: u32) -> u32 {
    unsafe { bb_take_remote_actions(player) }
}

#[cfg(target_arch = "wasm32")]
pub fn send_local_action(player: u32, mask: u32) {
    unsafe { bb_send_local_action(player, mask) };
}

#[cfg(target_arch = "wasm32")]
pub fn record_run_outcome(mode: u32, outcome: u32) {
    unsafe { bb_record_run_outcome(mode, outcome) };
}

#[cfg(target_arch = "wasm32")]
pub fn set_game_mode(mode: u32) {
    unsafe { bb_set_game_mode(mode) };
}

// --- Mode Art ---

#[derive(Default)]
pub struct ModeArt {
    mirror: Option<Texture2D>,
    strike: Option<Texture2D>,
    duo: Option<Texture2D>,
}

pub struct DecodedModeArt {
    pub width: u16,
    pub height: u16,
    pub rgba: Vec<u8>,
}

impl ModeArt {
    pub async fn load() -> Self {
        Self {
            mirror: load_mode_texture(MIRROR_ART_PATH).await,
            strike: load_mode_texture(STRIKE_ART_PATH).await,
            duo: load_mode_texture(DUO_ART_PATH).await,
        }
    }

    pub fn texture(&self, mode: GameMode) -> Option<&Texture2D> {
        match mode {
            GameMode::MirrorBeat => self.mirror.as_ref(),
            GameMode::BeatStrike => self.strike.as_ref(),
            GameMode::DuoGroove => self.duo.as_ref(),
        }
    }
}

async fn load_mode_texture(path: &str) -> Option<Texture2D> {
    let bytes = load_file(path).await.ok()?;
    let decoded = decode_mode_art(&bytes)?;
    let texture = Texture2D::from_rgba8(decoded.width, decoded.height, &decoded.rgba);
    texture.set_filter(FilterMode::Linear);
    Some(texture)
}

pub fn decode_mode_art(bytes: &[u8]) -> Option<DecodedModeArt> {
    if bytes.is_empty() || bytes.len() > MODE_ART_MAX_ENCODED_BYTES {
        return None;
    }
    let mut limits = image::io::Limits::default();
    limits.max_image_width = Some(MODE_ART_MAX_DIMENSION);
    limits.max_image_height = Some(MODE_ART_MAX_DIMENSION);
    limits.max_alloc = Some(MODE_ART_MAX_DECODE_BYTES);
    let mut reader =
        image::io::Reader::with_format(std::io::Cursor::new(bytes), image::ImageFormat::WebP);
    reader.limits(limits);
    let decoded = reader.decode().ok()?.to_rgba8();
    let width = u16::try_from(decoded.width()).ok()?;
    let height = u16::try_from(decoded.height()).ok()?;
    Some(DecodedModeArt {
        width,
        height,
        rgba: decoded.into_raw(),
    })
}
