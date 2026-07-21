use serde::{Deserialize, Serialize};

pub const PLAYER_CAPACITY: usize = 4;
pub const KEYPOINT_COUNT: usize = 17;

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

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecognizerConfig {
    pub minimum_pose_quality: f32,
    pub minimum_keypoint_confidence: f32,
    pub lateral_move_distance: f32,
    pub jump_distance: f32,
    pub squat_distance: f32,
    pub raised_hand_distance: f32,
    pub clap_width_ratio: f32,
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
            clap_width_ratio: 0.38,
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
    last_trigger_ms: [f64; 7],
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
            last_trigger_ms: [-1_000.0; 7],
        }
    }

    pub fn update(&mut self, pose: &PoseFrame) -> ActionSample {
        if pose.quality < self.config.minimum_pose_quality {
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
        if lw.confidence > self.config.minimum_keypoint_confidence
            && lw.y < ls.y - self.config.raised_hand_distance
        {
            active |= Action::LeftUp.mask();
        }
        if rw.confidence > self.config.minimum_keypoint_confidence
            && rw.y < rs.y - self.config.raised_hand_distance
        {
            active |= Action::RightUp.mask();
        }
        let wrist_distance = ((lw.x - rw.x).powi(2) + (lw.y - rw.y).powi(2)).sqrt();
        if lw.confidence > self.config.minimum_keypoint_confidence
            && rw.confidence > self.config.minimum_keypoint_confidence
            && wrist_distance < shoulder_width * self.config.clap_width_ratio
        {
            active |= Action::Clap.mask();
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
    }

    pub fn calibration_progress(&self) -> f32 {
        f32::from(self.neutral_samples) / 60.0
    }

    pub fn reset(&mut self) {
        let config = self.config;
        *self = Self::new(config);
    }
}

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
        let evaluated_triggered = self.players.map(|player| {
            if player.evaluation_enabled {
                player.triggered
            } else {
                0
            }
        });
        self.game
            .update(dt, evaluated_triggered, input.custom_target);
        for (index, player) in self.players.iter_mut().enumerate() {
            let attempted = evaluated_triggered[index] != 0;
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

pub const RUNNER_OBSTACLE_CAPACITY: usize = 12;

const RUNNER_BPM: f32 = 126.0;
const RUNNER_BEAT_SECONDS: f32 = 60.0 / RUNNER_BPM;
const RUNNER_COLLISION_DISTANCE: f32 = 0.085;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunnerPhase {
    #[default]
    Ready,
    Running,
    GameOver,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunnerHazard {
    LaneBlock,
    Hurdle,
    OverheadGate,
    BeatOrb,
}

impl RunnerHazard {
    pub const fn cue(self) -> &'static str {
        match self {
            Self::LaneBlock => "SWITCH LANE",
            Self::Hurdle => "JUMP",
            Self::OverheadGate => "SQUAT",
            Self::BeatOrb => "CLAP",
        }
    }

    pub const fn required_action(self) -> Option<Action> {
        match self {
            Self::LaneBlock => None,
            Self::Hurdle => Some(Action::Jump),
            Self::OverheadGate => Some(Action::Squat),
            Self::BeatOrb => Some(Action::Clap),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunnerObstacle {
    pub id: u32,
    pub lane: i8,
    pub kind: RunnerHazard,
    /// `1.0` is the horizon and `0.0` is the player collision line.
    pub distance: f32,
    resolved_players: u8,
}

impl RunnerObstacle {
    fn is_resolved_for(self, player: usize) -> bool {
        self.resolved_players & (1 << player) != 0
    }

    fn resolve_for(&mut self, player: usize) {
        self.resolved_players |= 1 << player;
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RunnerFeedback {
    #[default]
    None,
    Dodge,
    Crash,
    BeatPickup,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RunnerPlayer {
    pub lane: i8,
    pub score: u32,
    pub best_score: u32,
    pub combo: u16,
    pub lives: u8,
    pub evaluated: bool,
    pub jump_time: f32,
    pub slide_time: f32,
    pub pulse_time: f32,
    pub crash_time: f32,
}

impl Default for RunnerPlayer {
    fn default() -> Self {
        Self {
            lane: 0,
            score: 0,
            best_score: 0,
            combo: 0,
            lives: 3,
            evaluated: false,
            jump_time: 0.0,
            slide_time: 0.0,
            pulse_time: 0.0,
            crash_time: 0.0,
        }
    }
}

const RUNNER_PATTERN: [(RunnerHazard, i8); 16] = [
    (RunnerHazard::Hurdle, 0),
    (RunnerHazard::BeatOrb, 0),
    (RunnerHazard::LaneBlock, -1),
    (RunnerHazard::OverheadGate, 0),
    (RunnerHazard::LaneBlock, 1),
    (RunnerHazard::Hurdle, -1),
    (RunnerHazard::BeatOrb, 1),
    (RunnerHazard::OverheadGate, 1),
    (RunnerHazard::LaneBlock, 0),
    (RunnerHazard::Hurdle, 1),
    (RunnerHazard::BeatOrb, -1),
    (RunnerHazard::OverheadGate, -1),
    (RunnerHazard::LaneBlock, -1),
    (RunnerHazard::Hurdle, 0),
    (RunnerHazard::LaneBlock, 1),
    (RunnerHazard::BeatOrb, 0),
];

#[derive(Clone, Debug)]
pub struct RunnerGame {
    pub phase: RunnerPhase,
    pub players: [RunnerPlayer; PLAYER_CAPACITY],
    pub obstacles: [Option<RunnerObstacle>; RUNNER_OBSTACLE_CAPACITY],
    pub feedback: [RunnerFeedback; PLAYER_CAPACITY],
    pub beat_index: u32,
    pub beat_progress: f32,
    pub world_distance: f32,
    pub speed: f32,
    beat_accumulator: f32,
    score_fraction: [f32; PLAYER_CAPACITY],
    next_obstacle_id: u32,
    pattern_cursor: usize,
}

impl Default for RunnerGame {
    fn default() -> Self {
        Self::new()
    }
}

impl RunnerGame {
    pub fn new() -> Self {
        Self {
            phase: RunnerPhase::Ready,
            players: [RunnerPlayer::default(); PLAYER_CAPACITY],
            obstacles: [None; RUNNER_OBSTACLE_CAPACITY],
            feedback: [RunnerFeedback::None; PLAYER_CAPACITY],
            beat_index: 0,
            beat_progress: 0.0,
            world_distance: 0.0,
            speed: 0.205,
            beat_accumulator: 0.0,
            score_fraction: [0.0; PLAYER_CAPACITY],
            next_obstacle_id: 1,
            pattern_cursor: 0,
        }
    }

    pub fn update(
        &mut self,
        dt: f32,
        active: [u32; PLAYER_CAPACITY],
        triggered: [u32; PLAYER_CAPACITY],
        evaluation_enabled: [bool; PLAYER_CAPACITY],
    ) {
        let dt = dt.clamp(0.0, 0.05);
        self.feedback = [RunnerFeedback::None; PLAYER_CAPACITY];
        for (player, enabled) in self.players.iter_mut().zip(evaluation_enabled) {
            player.evaluated = enabled;
            player.jump_time = (player.jump_time - dt).max(0.0);
            player.slide_time = (player.slide_time - dt).max(0.0);
            player.pulse_time = (player.pulse_time - dt).max(0.0);
            player.crash_time = (player.crash_time - dt).max(0.0);
        }

        let has_evaluated_player = self.players.iter().any(|player| player.evaluated);
        if self.phase == RunnerPhase::GameOver {
            let restart_requested = triggered
                .into_iter()
                .zip(self.players.iter())
                .any(|(mask, player)| player.evaluated && mask & Action::Clap.mask() != 0);
            if restart_requested {
                self.restart(evaluation_enabled);
            }
            return;
        }
        if self.phase == RunnerPhase::Ready {
            if !has_evaluated_player {
                return;
            }
            self.phase = RunnerPhase::Running;
        }

        self.apply_actions(active, triggered);
        self.advance_beat(dt);
        self.speed = (0.205 + self.world_distance / 12_000.0).min(0.31);
        self.world_distance += (18.0 + self.speed * 22.0) * dt;

        for (index, player) in self.players.iter_mut().enumerate() {
            if !player.evaluated || player.lives == 0 {
                continue;
            }
            self.score_fraction[index] += dt * (11.0 + self.speed * 18.0);
            let whole_points = self.score_fraction[index].floor() as u32;
            if whole_points > 0 {
                player.score = player.score.saturating_add(whole_points);
                self.score_fraction[index] -= whole_points as f32;
            }
        }

        self.advance_obstacles(dt);
        let all_evaluated_players_out = has_evaluated_player
            && self
                .players
                .iter()
                .filter(|player| player.evaluated)
                .all(|player| player.lives == 0);
        if all_evaluated_players_out {
            self.phase = RunnerPhase::GameOver;
            for player in &mut self.players {
                player.best_score = player.best_score.max(player.score);
            }
        }
    }

    pub fn next_cue(&self) -> Option<RunnerObstacle> {
        self.obstacles
            .iter()
            .flatten()
            .filter(|obstacle| obstacle.distance >= 0.0)
            .min_by(|a, b| a.distance.total_cmp(&b.distance))
            .copied()
    }

    fn apply_actions(&mut self, active: [u32; PLAYER_CAPACITY], triggered: [u32; PLAYER_CAPACITY]) {
        for index in 0..PLAYER_CAPACITY {
            let player = &mut self.players[index];
            if !player.evaluated || player.lives == 0 {
                continue;
            }
            let mask = triggered[index];
            let moved_left = mask & Action::MoveLeft.mask() != 0;
            let moved_right = mask & Action::MoveRight.mask() != 0;
            if moved_left != moved_right {
                player.lane = if moved_left {
                    (player.lane - 1).max(-1)
                } else {
                    (player.lane + 1).min(1)
                };
            }
            if mask & Action::Jump.mask() != 0 || active[index] & Action::Jump.mask() != 0 {
                player.jump_time = player.jump_time.max(0.72);
            }
            if mask & Action::Squat.mask() != 0 || active[index] & Action::Squat.mask() != 0 {
                player.slide_time = player.slide_time.max(0.78);
            }
            if mask & Action::Clap.mask() != 0 {
                player.pulse_time = 0.36;
            }
        }
    }

    fn advance_beat(&mut self, dt: f32) {
        self.beat_accumulator += dt;
        while self.beat_accumulator >= RUNNER_BEAT_SECONDS {
            self.beat_accumulator -= RUNNER_BEAT_SECONDS;
            self.beat_index = self.beat_index.wrapping_add(1);
            let spawn_interval = if self.beat_index < 24 { 4 } else { 3 };
            if self.beat_index % spawn_interval == 1 {
                let (kind, lane) = RUNNER_PATTERN[self.pattern_cursor % RUNNER_PATTERN.len()];
                self.spawn(kind, lane);
                self.pattern_cursor = self.pattern_cursor.wrapping_add(1);
            }
        }
        self.beat_progress = self.beat_accumulator / RUNNER_BEAT_SECONDS;
    }

    fn spawn(&mut self, kind: RunnerHazard, lane: i8) {
        let slot = self
            .obstacles
            .iter()
            .position(Option::is_none)
            .unwrap_or_else(|| {
                self.obstacles
                    .iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| {
                        let a_distance = a.map_or(f32::INFINITY, |item| item.distance);
                        let b_distance = b.map_or(f32::INFINITY, |item| item.distance);
                        a_distance.total_cmp(&b_distance)
                    })
                    .map_or(0, |(index, _)| index)
            });
        self.obstacles[slot] = Some(RunnerObstacle {
            id: self.next_obstacle_id,
            lane: lane.clamp(-1, 1),
            kind,
            distance: 1.0,
            resolved_players: 0,
        });
        self.next_obstacle_id = self.next_obstacle_id.wrapping_add(1).max(1);
    }

    fn advance_obstacles(&mut self, dt: f32) {
        for obstacle in self.obstacles.iter_mut().flatten() {
            obstacle.distance -= self.speed * dt;
            if obstacle.distance > RUNNER_COLLISION_DISTANCE {
                continue;
            }
            for player_index in 0..PLAYER_CAPACITY {
                if obstacle.is_resolved_for(player_index) {
                    continue;
                }
                obstacle.resolve_for(player_index);
                let player = &mut self.players[player_index];
                if !player.evaluated || player.lives == 0 {
                    continue;
                }
                let same_lane = player.lane == obstacle.lane;
                let success = match obstacle.kind {
                    RunnerHazard::LaneBlock => !same_lane,
                    RunnerHazard::Hurdle => !same_lane || player.jump_time > 0.0,
                    RunnerHazard::OverheadGate => !same_lane || player.slide_time > 0.0,
                    RunnerHazard::BeatOrb => same_lane && player.pulse_time > 0.0,
                };
                if obstacle.kind == RunnerHazard::BeatOrb {
                    if success {
                        player.combo = player.combo.saturating_add(1);
                        player.score = player
                            .score
                            .saturating_add(250 + u32::from(player.combo.min(20)) * 15);
                        self.feedback[player_index] = RunnerFeedback::BeatPickup;
                    }
                } else if success {
                    player.combo = player.combo.saturating_add(1);
                    player.score = player
                        .score
                        .saturating_add(120 + u32::from(player.combo.min(20)) * 10);
                    self.feedback[player_index] = RunnerFeedback::Dodge;
                } else {
                    player.lives = player.lives.saturating_sub(1);
                    player.combo = 0;
                    player.crash_time = 0.48;
                    self.feedback[player_index] = RunnerFeedback::Crash;
                }
            }
        }
        for slot in &mut self.obstacles {
            if slot.is_some_and(|obstacle| obstacle.distance < -0.14) {
                *slot = None;
            }
        }
    }

    fn restart(&mut self, evaluation_enabled: [bool; PLAYER_CAPACITY]) {
        let best_scores = self
            .players
            .map(|player| player.best_score.max(player.score));
        *self = Self::new();
        for index in 0..PLAYER_CAPACITY {
            self.players[index].best_score = best_scores[index];
            self.players[index].evaluated = evaluation_enabled[index];
        }
        if evaluation_enabled.into_iter().any(|enabled| enabled) {
            self.phase = RunnerPhase::Running;
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
        assert_eq!(game.feedback[0], RunnerFeedback::Dodge);
    }

    #[test]
    fn collision_costs_one_life_once() {
        let mut game = RunnerGame::new();
        game.spawn(RunnerHazard::LaneBlock, 0);
        game.obstacles[0].as_mut().unwrap().distance = RUNNER_COLLISION_DISTANCE + 0.001;
        let evaluation = [true, false, false, false];
        game.update(0.02, [0; PLAYER_CAPACITY], [0; PLAYER_CAPACITY], evaluation);
        assert_eq!(game.players[0].lives, 2);
        game.update(0.02, [0; PLAYER_CAPACITY], [0; PLAYER_CAPACITY], evaluation);
        assert_eq!(game.players[0].lives, 2);
    }

    #[test]
    fn clap_restarts_after_game_over_and_preserves_best_score() {
        let mut game = RunnerGame::new();
        game.phase = RunnerPhase::GameOver;
        game.players[0].evaluated = true;
        game.players[0].score = 1_240;
        game.players[0].lives = 0;
        game.update(
            0.01,
            [0; PLAYER_CAPACITY],
            [Action::Clap.mask(), 0, 0, 0],
            [true, false, false, false],
        );
        assert_eq!(game.phase, RunnerPhase::Running);
        assert_eq!(game.players[0].lives, 3);
        assert_eq!(game.players[0].score, 0);
        assert_eq!(game.players[0].best_score, 1_240);
    }
}
