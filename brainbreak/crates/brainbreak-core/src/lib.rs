use serde::{Deserialize, Serialize};

pub const PLAYER_CAPACITY: usize = 4;
pub const KEYPOINT_COUNT: usize = 17;

#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Keypoint {
    pub x: f32,
    pub y: f32,
    pub confidence: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
}

impl Action {
    pub const ALL: [Self; 7] = [
        Self::MoveLeft,
        Self::MoveRight,
        Self::Jump,
        Self::Squat,
        Self::LeftUp,
        Self::RightUp,
        Self::Clap,
    ];

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

#[derive(Clone, Debug)]
pub struct PoseRecognizer {
    neutral_hip_y: Option<f32>,
    neutral_samples: u16,
    previous_active: u32,
    last_trigger_ms: [f64; 7],
}

impl Default for PoseRecognizer {
    fn default() -> Self {
        Self {
            neutral_hip_y: None,
            neutral_samples: 0,
            previous_active: 0,
            last_trigger_ms: [-1_000.0; 7],
        }
    }
}

impl PoseRecognizer {
    pub fn update(&mut self, pose: &PoseFrame) -> ActionSample {
        if pose.quality < 0.25 {
            self.previous_active = 0;
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

        let neutral = self.neutral_hip_y.get_or_insert(hip_y);
        if self.neutral_samples < 60 {
            *neutral = (*neutral * f32::from(self.neutral_samples) + hip_y)
                / f32::from(self.neutral_samples + 1);
            self.neutral_samples += 1;
        } else if (hip_y - *neutral).abs() < 0.04 {
            *neutral = *neutral * 0.995 + hip_y * 0.005;
        }

        let mut active = 0;
        if shoulder_center_x < 0.40 {
            active |= Action::MoveLeft.mask();
        } else if shoulder_center_x > 0.60 {
            active |= Action::MoveRight.mask();
        }
        if hip_y < *neutral - 0.065 {
            active |= Action::Jump.mask();
        } else if hip_y > *neutral + 0.075 {
            active |= Action::Squat.mask();
        }
        if lw.confidence > 0.25 && lw.y < ls.y - 0.035 {
            active |= Action::LeftUp.mask();
        }
        if rw.confidence > 0.25 && rw.y < rs.y - 0.035 {
            active |= Action::RightUp.mask();
        }
        let wrist_distance = ((lw.x - rw.x).powi(2) + (lw.y - rw.y).powi(2)).sqrt();
        if lw.confidence > 0.25 && rw.confidence > 0.25 && wrist_distance < shoulder_width * 0.38 {
            active |= Action::Clap.mask();
        }

        let rising = active & !self.previous_active;
        let mut triggered = 0;
        for (index, action) in Action::ALL.iter().enumerate() {
            if rising & action.mask() != 0
                && pose.timestamp_ms - self.last_trigger_ms[index] >= 220.0
            {
                triggered |= action.mask();
                self.last_trigger_ms[index] = pose.timestamp_ms;
            }
        }
        self.previous_active = active;
        ActionSample { active, triggered }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GameMode {
    MirrorBeat,
    BeatStrike,
    DuoGroove,
}

impl GameMode {
    pub const fn title(self) -> &'static str {
        match self {
            Self::MirrorBeat => "MIRROR BEAT",
            Self::BeatStrike => "BEAT STRIKE",
            Self::DuoGroove => "DUO GROOVE",
        }
    }

    pub const fn next(self) -> Self {
        match self {
            Self::MirrorBeat => Self::BeatStrike,
            Self::BeatStrike => Self::DuoGroove,
            Self::DuoGroove => Self::MirrorBeat,
        }
    }
}

const MIRROR_SEQUENCE: [Action; 12] = [
    Action::LeftUp,
    Action::RightUp,
    Action::Clap,
    Action::Squat,
    Action::MoveLeft,
    Action::MoveRight,
    Action::Jump,
    Action::Clap,
    Action::LeftUp,
    Action::RightUp,
    Action::Squat,
    Action::Jump,
];

const STRIKE_SEQUENCE: [Action; 10] = [
    Action::MoveLeft,
    Action::MoveRight,
    Action::LeftUp,
    Action::RightUp,
    Action::Clap,
    Action::MoveRight,
    Action::MoveLeft,
    Action::Clap,
    Action::RightUp,
    Action::LeftUp,
];

const DUO_SEQUENCE: [Action; 8] = [
    Action::LeftUp,
    Action::RightUp,
    Action::Clap,
    Action::Squat,
    Action::RightUp,
    Action::LeftUp,
    Action::Jump,
    Action::Clap,
];

#[derive(Clone, Debug)]
pub struct GameState {
    pub mode: GameMode,
    pub beat_index: u32,
    pub beat_progress: f32,
    pub target: Action,
    pub scores: [u32; PLAYER_CAPACITY],
    pub combos: [u16; PLAYER_CAPACITY],
    beat_seconds: f32,
    accumulator: f32,
    hit_this_beat: [bool; PLAYER_CAPACITY],
    duo_bonus_awarded: [bool; 2],
}

impl Default for GameState {
    fn default() -> Self {
        Self::new(GameMode::MirrorBeat)
    }
}

impl GameState {
    pub fn new(mode: GameMode) -> Self {
        let (target, beat_seconds) = match mode {
            GameMode::MirrorBeat => (MIRROR_SEQUENCE[0], 1.45),
            GameMode::BeatStrike => (STRIKE_SEQUENCE[0], 1.0),
            GameMode::DuoGroove => (DUO_SEQUENCE[0], 1.25),
        };
        Self {
            mode,
            beat_index: 0,
            beat_progress: 0.0,
            target,
            scores: [0; PLAYER_CAPACITY],
            combos: [0; PLAYER_CAPACITY],
            beat_seconds,
            accumulator: 0.0,
            hit_this_beat: [false; PLAYER_CAPACITY],
            duo_bonus_awarded: [false; 2],
        }
    }

    pub fn set_mode(&mut self, mode: GameMode) {
        *self = Self::new(mode);
    }

    pub fn update(
        &mut self,
        dt: f32,
        triggered: [u32; PLAYER_CAPACITY],
        custom_target: Option<u32>,
    ) {
        self.accumulator += dt.clamp(0.0, 0.05);
        while self.accumulator >= self.beat_seconds {
            self.accumulator -= self.beat_seconds;
            self.beat_index = self.beat_index.wrapping_add(1);
            self.target = match self.mode {
                GameMode::MirrorBeat => {
                    MIRROR_SEQUENCE[self.beat_index as usize % MIRROR_SEQUENCE.len()]
                }
                GameMode::BeatStrike => {
                    STRIKE_SEQUENCE[self.beat_index as usize % STRIKE_SEQUENCE.len()]
                }
                GameMode::DuoGroove => DUO_SEQUENCE[self.beat_index as usize % DUO_SEQUENCE.len()],
            };
            self.hit_this_beat = [false; PLAYER_CAPACITY];
            self.duo_bonus_awarded = [false; 2];
        }
        self.beat_progress = self.accumulator / self.beat_seconds;
        if let Some(custom) = custom_target.and_then(Action::from_mask) {
            self.target = custom;
        }
        let target_mask = self.target.mask();
        for (player, mask) in triggered.into_iter().enumerate() {
            if mask & target_mask != 0 && !self.hit_this_beat[player] {
                self.hit_this_beat[player] = true;
                self.combos[player] = self.combos[player].saturating_add(1);
                self.scores[player] = self.scores[player]
                    .saturating_add(100 + u32::from(self.combos[player]).min(20) * 10);
            } else if mask != 0 {
                self.combos[player] = 0;
            }
        }
        if self.mode == GameMode::DuoGroove {
            for (pair, players) in [[0, 1], [2, 3]].into_iter().enumerate() {
                if !self.duo_bonus_awarded[pair]
                    && players.into_iter().all(|player| self.hit_this_beat[player])
                {
                    self.duo_bonus_awarded[pair] = true;
                    for player in players {
                        self.scores[player] = self.scores[player].saturating_add(150);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
