use serde::{Deserialize, Serialize};

use crate::KEYPOINT_COUNT;

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Keypoint {
    pub x: f32,
    pub y: f32,
    pub confidence: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PoseFrame {
    pub tracked_id: u32,
    pub quality: f32,
    pub timestamp_ms: f64,
    pub keypoints: [Keypoint; KEYPOINT_COUNT],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[repr(u32)]
pub enum Action {
    MoveLeft = 1 << 0,
    MoveRight = 1 << 1,
    Jump = 1 << 2,
    Squat = 1 << 3,
    LeftUp = 1 << 4,
    RightUp = 1 << 5,
    Clap = 1 << 6,
    Pause = 1 << 7,
    /// Both wrists clearly below hips — Pull-to-Drop release without Hands landmarks.
    HandsDown = 1 << 8,
}

impl Action {
    pub const ALL: [Self; 9] = [
        Self::MoveLeft,
        Self::MoveRight,
        Self::Jump,
        Self::Squat,
        Self::LeftUp,
        Self::RightUp,
        Self::Clap,
        Self::Pause,
        Self::HandsDown,
    ];

    pub const GAMEPLAY_MASK: u32 = Self::MoveLeft.mask()
        | Self::MoveRight.mask()
        | Self::Jump.mask()
        | Self::Squat.mask()
        | Self::LeftUp.mask()
        | Self::RightUp.mask()
        | Self::Clap.mask();

    /// Supernova Drop release verbs once the core is trembling (no MediaPipe Hands).
    pub const DROP_RELEASE_MASK: u32 = Self::Squat.mask() | Self::HandsDown.mask();

    pub const fn mask(self) -> u32 {
        self as u32
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::MoveLeft => "MOVE LEFT",
            Self::MoveRight => "MOVE RIGHT",
            Self::Jump => "JUMP",
            Self::Squat => "SQUAT",
            Self::LeftUp => "LEFT HAND UP",
            Self::RightUp => "RIGHT HAND UP",
            Self::Clap => "CLAP",
            Self::Pause => "PAUSE",
            Self::HandsDown => "HANDS DOWN",
        }
    }

    pub fn from_mask(mask: u32) -> Option<Self> {
        Self::ALL.into_iter().find(|action| action.mask() == mask)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ActionSample {
    pub active: u32,
    pub triggered: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecognizerConfig {
    pub minimum_pose_quality: f32,
    pub minimum_keypoint_confidence: f32,
    pub lateral_move_distance: f32,
    pub jump_distance: f32,
    pub squat_distance: f32,
    pub raised_hand_distance: f32,
    /// Wrists must sit this far below hips to count as a reach-down / pull.
    pub hands_down_distance: f32,
    pub clap_width_ratio: f32,
    pub pause_hold_ms: f64,
    pub cooldown_ms: f64,
}

impl Default for RecognizerConfig {
    fn default() -> Self {
        Self {
            minimum_pose_quality: 0.25,
            minimum_keypoint_confidence: 0.25,
            lateral_move_distance: 0.075,
            jump_distance: 0.065,
            squat_distance: 0.075,
            raised_hand_distance: 0.035,
            hands_down_distance: 0.05,
            clap_width_ratio: 0.38,
            pause_hold_ms: 1_000.0,
            cooldown_ms: 220.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PoseRecognizer {
    config: RecognizerConfig,
    neutral_hip_y: Option<f32>,
    neutral_center_x: Option<f32>,
    neutral_samples: u16,
    previous_active: u32,
    last_trigger_ms: [f64; 9],
    pause_started_ms: Option<f64>,
}

impl Default for PoseRecognizer {
    fn default() -> Self {
        Self::new(RecognizerConfig::default())
    }
}

impl PoseRecognizer {
    pub fn new(config: RecognizerConfig) -> Self {
        Self {
            config,
            neutral_hip_y: None,
            neutral_center_x: None,
            neutral_samples: 0,
            previous_active: 0,
            last_trigger_ms: [-1_000.0; 9],
            pause_started_ms: None,
        }
    }

    pub fn update(&mut self, pose: &PoseFrame) -> ActionSample {
        if pose.quality < self.config.minimum_pose_quality {
            self.previous_active = 0;
            self.pause_started_ms = None;
            return ActionSample::default();
        }

        const LEFT_SHOULDER: usize = 5;
        const RIGHT_SHOULDER: usize = 6;
        const LEFT_WRIST: usize = 9;
        const RIGHT_WRIST: usize = 10;
        const LEFT_HIP: usize = 11;
        const RIGHT_HIP: usize = 12;

        let ls = pose.keypoints[LEFT_SHOULDER];
        let rs = pose.keypoints[RIGHT_SHOULDER];
        let lw = pose.keypoints[LEFT_WRIST];
        let rw = pose.keypoints[RIGHT_WRIST];
        let lh = pose.keypoints[LEFT_HIP];
        let rh = pose.keypoints[RIGHT_HIP];
        let shoulder_center_x = (ls.x + rs.x) * 0.5;
        let hip_y = (lh.y + rh.y) * 0.5;
        let shoulder_width = (ls.x - rs.x).abs().max(0.08);

        let neutral_hip_y = self.neutral_hip_y.get_or_insert(hip_y);
        let neutral_center_x = self.neutral_center_x.get_or_insert(shoulder_center_x);
        if self.neutral_samples < 60 {
            *neutral_hip_y = (*neutral_hip_y * f32::from(self.neutral_samples) + hip_y)
                / f32::from(self.neutral_samples + 1);
            *neutral_center_x = (*neutral_center_x * f32::from(self.neutral_samples)
                + shoulder_center_x)
                / f32::from(self.neutral_samples + 1);
            self.neutral_samples += 1;
        } else {
            if (hip_y - *neutral_hip_y).abs() < 0.04 {
                *neutral_hip_y = *neutral_hip_y * 0.995 + hip_y * 0.005;
            }
            if (shoulder_center_x - *neutral_center_x).abs() < 0.04 {
                *neutral_center_x = *neutral_center_x * 0.995 + shoulder_center_x * 0.005;
            }
        }

        let mut active = 0;
        if shoulder_center_x < *neutral_center_x - self.config.lateral_move_distance {
            active |= Action::MoveLeft.mask();
        } else if shoulder_center_x > *neutral_center_x + self.config.lateral_move_distance {
            active |= Action::MoveRight.mask();
        }
        if hip_y < *neutral_hip_y - self.config.jump_distance {
            active |= Action::Jump.mask();
        } else if hip_y > *neutral_hip_y + self.config.squat_distance {
            active |= Action::Squat.mask();
        }
        let left_hand_up = lw.confidence > self.config.minimum_keypoint_confidence
            && lw.y < ls.y - self.config.raised_hand_distance;
        let right_hand_up = rw.confidence > self.config.minimum_keypoint_confidence
            && rw.y < rs.y - self.config.raised_hand_distance;
        if left_hand_up {
            active |= Action::LeftUp.mask();
        }
        if right_hand_up {
            active |= Action::RightUp.mask();
        }
        let left_hand_down = lw.confidence > self.config.minimum_keypoint_confidence
            && lh.confidence > self.config.minimum_keypoint_confidence
            && lw.y > lh.y + self.config.hands_down_distance;
        let right_hand_down = rw.confidence > self.config.minimum_keypoint_confidence
            && rh.confidence > self.config.minimum_keypoint_confidence
            && rw.y > rh.y + self.config.hands_down_distance;
        if left_hand_down && right_hand_down && !left_hand_up && !right_hand_up {
            active |= Action::HandsDown.mask();
        }
        let wrist_distance = ((lw.x - rw.x).powi(2) + (lw.y - rw.y).powi(2)).sqrt();
        if lw.confidence > self.config.minimum_keypoint_confidence
            && rw.confidence > self.config.minimum_keypoint_confidence
            && wrist_distance < shoulder_width * self.config.clap_width_ratio
        {
            active |= Action::Clap.mask();
        }
        let pause_pose = left_hand_up
            && right_hand_up
            && wrist_distance >= shoulder_width * self.config.clap_width_ratio;
        if pause_pose {
            // Reserve the complete dwell for the control channel. Without this,
            // the constituent hand-up actions could score before Pause rises.
            active = 0;
            let started_ms = self.pause_started_ms.get_or_insert(pose.timestamp_ms);
            if pose.timestamp_ms - *started_ms >= self.config.pause_hold_ms {
                active |= Action::Pause.mask();
            }
        } else {
            self.pause_started_ms = None;
        }

        let rising = active & !self.previous_active;
        let mut triggered = 0;
        for (index, action) in Action::ALL.iter().enumerate() {
            if rising & action.mask() != 0
                && pose.timestamp_ms - self.last_trigger_ms[index] >= self.config.cooldown_ms
            {
                triggered |= action.mask();
                self.last_trigger_ms[index] = pose.timestamp_ms;
            }
        }
        self.previous_active = active;
        ActionSample { active, triggered }
    }

    pub fn lose_tracking(&mut self) {
        self.previous_active = 0;
        self.pause_started_ms = None;
    }

    pub fn calibration_progress(&self) -> f32 {
        f32::from(self.neutral_samples) / 60.0
    }

    pub fn reset(&mut self) {
        let config = self.config;
        *self = Self::new(config);
    }
}
