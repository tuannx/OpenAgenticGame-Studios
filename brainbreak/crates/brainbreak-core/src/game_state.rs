use serde::{Deserialize, Serialize};

use crate::PLAYER_CAPACITY;
use crate::pose::Action;

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

    pub const fn previous(self) -> Self {
        match self {
            Self::MirrorBeat => Self::DuoGroove,
            Self::BeatStrike => Self::MirrorBeat,
            Self::DuoGroove => Self::BeatStrike,
        }
    }

    pub(crate) const fn next_with_duo(self, duo_available: bool) -> Self {
        if duo_available {
            return self.next();
        }
        match self {
            Self::MirrorBeat => Self::BeatStrike,
            Self::BeatStrike | Self::DuoGroove => Self::MirrorBeat,
        }
    }

    pub(crate) const fn previous_with_duo(self, duo_available: bool) -> Self {
        if duo_available {
            return self.previous();
        }
        match self {
            Self::MirrorBeat => Self::BeatStrike,
            Self::BeatStrike | Self::DuoGroove => Self::MirrorBeat,
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
