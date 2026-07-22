use crate::PLAYER_CAPACITY;
use crate::game_state::{GameMode, GameState};
use crate::pose::{Action, PoseFrame, PoseRecognizer, RecognizerConfig};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PlayerMotionState {
    pub pose: Option<PoseFrame>,
    pub calibration: f32,
    pub evaluation_enabled: bool,
    pub active: u32,
    pub triggered: u32,
    pub hit: bool,
    pub miss: bool,
}

impl PlayerMotionState {
    pub fn quality(self) -> f32 {
        self.pose.map_or(0.0, |pose| pose.quality)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MotionInputFrame {
    pub local_poses: [Option<PoseFrame>; 2],
    pub fallback_actions: [u32; 2],
    pub remote_actions: [u32; 2],
    pub evaluation_enabled: [bool; PLAYER_CAPACITY],
    pub custom_target: Option<u32>,
}

impl Default for MotionInputFrame {
    fn default() -> Self {
        Self {
            local_poses: [None; 2],
            fallback_actions: [0; 2],
            remote_actions: [0; 2],
            evaluation_enabled: [true; PLAYER_CAPACITY],
            custom_target: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MotionRuntime {
    pub game: GameState,
    pub players: [PlayerMotionState; PLAYER_CAPACITY],
    recognizers: [PoseRecognizer; 2],
}

impl Default for MotionRuntime {
    fn default() -> Self {
        Self::new(GameMode::MirrorBeat, RecognizerConfig::default())
    }
}

impl MotionRuntime {
    pub fn new(mode: GameMode, recognizer_config: RecognizerConfig) -> Self {
        Self {
            game: GameState::new(mode),
            players: [PlayerMotionState::default(); PLAYER_CAPACITY],
            recognizers: [
                PoseRecognizer::new(recognizer_config),
                PoseRecognizer::new(recognizer_config),
            ],
        }
    }

    pub fn set_mode(&mut self, mode: GameMode) {
        self.game.set_mode(mode);
        self.players = [PlayerMotionState::default(); PLAYER_CAPACITY];
        for recognizer in &mut self.recognizers {
            recognizer.reset();
        }
    }

    pub fn update(&mut self, dt: f32, input: MotionInputFrame) {
        for player in &mut self.players {
            player.triggered = 0;
            player.hit = false;
            player.miss = false;
        }

        for player in 0..2 {
            let previous_id = self.players[player].pose.map(|pose| pose.tracked_id);
            let next_id = input.local_poses[player].map(|pose| pose.tracked_id);
            if previous_id.is_some() && next_id.is_some() && previous_id != next_id {
                self.recognizers[player].reset();
            } else if next_id.is_none() {
                self.recognizers[player].lose_tracking();
            }
            self.players[player].pose = input.local_poses[player];
            self.players[player].evaluation_enabled = input.evaluation_enabled[player];
            let detected = input.local_poses[player]
                .map(|pose| self.recognizers[player].update(&pose))
                .unwrap_or_default();
            self.players[player].active = detected.active;
            self.players[player].calibration = if input.local_poses[player].is_some() {
                self.recognizers[player].calibration_progress()
            } else {
                0.0
            };
            self.players[player].triggered = detected.triggered | input.fallback_actions[player];
        }
        for player in 0..2 {
            let index = player + 2;
            self.players[index].pose = None;
            self.players[index].evaluation_enabled = input.evaluation_enabled[index];
            self.players[index].active = input.remote_actions[player];
            self.players[index].triggered = input.remote_actions[player];
        }

        let scores_before = self.game.scores;
        let evaluated_gameplay_triggered = self.players.map(|player| {
            if player.evaluation_enabled {
                player.triggered & Action::GAMEPLAY_MASK
            } else {
                0
            }
        });
        self.game
            .update(dt, evaluated_gameplay_triggered, input.custom_target);
        for (index, player) in self.players.iter_mut().enumerate() {
            let attempted = evaluated_gameplay_triggered[index] != 0;
            player.hit = attempted && self.game.scores[index] > scores_before[index];
            player.miss = attempted && !player.hit;
        }
    }

    pub fn local_triggered(&self) -> [u32; 2] {
        [self.players[0].triggered, self.players[1].triggered]
    }

    pub fn local_evaluated_triggered(&self) -> [u32; 2] {
        [0, 1].map(|player| {
            if self.players[player].evaluation_enabled {
                self.players[player].triggered
            } else {
                0
            }
        })
    }
}
