use serde::{Deserialize, Serialize};

use crate::PLAYER_CAPACITY;
use crate::game_state::GameMode;
use crate::pose::Action;

pub const RUNNER_OBSTACLE_CAPACITY: usize = 12;

const RUNNER_BPM: f32 = 126.0;
const RUNNER_BEAT_SECONDS: f32 = 60.0 / RUNNER_BPM;
pub const RUNNER_COLLISION_DISTANCE: f32 = 0.085;
pub const RUNNER_COUNTDOWN_SECONDS: f32 = 2.0;
pub const RUNNER_SESSION_SECONDS: f32 = 90.0;

/// Breath arc phase boundaries (seconds within the session).
const BREATH_RISE_END: f32 = 55.0;
const BREATH_PEAK_END: f32 = 75.0;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunnerPhase {
    #[default]
    Ready,
    Starting,
    Running,
    Paused,
    PauseResuming,
    TrackingHold,
    Resuming,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RunnerOutcome {
    BreakComplete,
    EnergySpent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RunnerResultEvent {
    pub mode: GameMode,
    pub outcome: RunnerOutcome,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RunnerPlayer {
    pub lane: i8,
    pub score: u32,
    pub best_score: u32,
    pub combo: u16,
    pub best_combo: u16,
    pub lives: u8,
    pub new_best: bool,
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
            best_combo: 0,
            lives: 3,
            new_best: false,
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
    pub mode: GameMode,
    pub phase: RunnerPhase,
    pub players: [RunnerPlayer; PLAYER_CAPACITY],
    pub obstacles: [Option<RunnerObstacle>; RUNNER_OBSTACLE_CAPACITY],
    pub feedback: [RunnerFeedback; PLAYER_CAPACITY],
    pub beat_index: u32,
    pub beat_progress: f32,
    pub world_distance: f32,
    pub speed: f32,
    pub ready_players: [bool; PLAYER_CAPACITY],
    pub countdown_remaining: f32,
    pub result_selection: GameMode,
    pub result_outcome: Option<RunnerOutcome>,
    beat_accumulator: f32,
    pub(crate) run_elapsed_seconds: f32,
    score_fraction: [f32; PLAYER_CAPACITY],
    next_obstacle_id: u32,
    pattern_cursor: usize,
    next_run_request: Option<GameMode>,
    pending_result_event: Option<RunnerResultEvent>,
}

impl Default for RunnerGame {
    fn default() -> Self {
        Self::new()
    }
}

impl RunnerGame {
    pub fn new() -> Self {
        Self::with_mode(GameMode::MirrorBeat)
    }

    pub fn with_mode(mode: GameMode) -> Self {
        Self {
            mode,
            phase: RunnerPhase::Ready,
            players: [RunnerPlayer::default(); PLAYER_CAPACITY],
            obstacles: [None; RUNNER_OBSTACLE_CAPACITY],
            feedback: [RunnerFeedback::None; PLAYER_CAPACITY],
            beat_index: 0,
            beat_progress: 0.0,
            world_distance: 0.0,
            speed: 0.205,
            ready_players: [false; PLAYER_CAPACITY],
            countdown_remaining: 0.0,
            result_selection: mode,
            result_outcome: None,
            beat_accumulator: 0.0,
            run_elapsed_seconds: 0.0,
            score_fraction: [0.0; PLAYER_CAPACITY],
            next_obstacle_id: 1,
            pattern_cursor: 0,
            next_run_request: None,
            pending_result_event: None,
        }
    }

    pub fn set_mode(&mut self, mode: GameMode) {
        if self.mode != mode {
            *self = Self::with_mode(mode);
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
            let duo_available = evaluation_enabled[0] && evaluation_enabled[1];
            if !duo_available && self.result_selection == GameMode::DuoGroove {
                self.result_selection = GameMode::MirrorBeat;
            }
            let move_left = triggered
                .into_iter()
                .zip(self.players.iter())
                .any(|(mask, player)| player.evaluated && mask & Action::MoveLeft.mask() != 0);
            let move_right = triggered
                .into_iter()
                .zip(self.players.iter())
                .any(|(mask, player)| player.evaluated && mask & Action::MoveRight.mask() != 0);
            if move_left != move_right {
                self.result_selection = if move_left {
                    self.result_selection.previous_with_duo(duo_available)
                } else {
                    self.result_selection.next_with_duo(duo_available)
                };
            }
            let play_requested = triggered
                .into_iter()
                .zip(self.players.iter())
                .any(|(mask, player)| player.evaluated && mask & Action::Clap.mask() != 0);
            if play_requested {
                self.next_run_request = Some(self.result_selection);
            }
            return;
        }
        if self.phase == RunnerPhase::Running {
            if Self::pause_requested(triggered, evaluation_enabled) {
                self.begin_paused();
                return;
            }
            if !self.has_required_evaluation(evaluation_enabled) {
                self.begin_tracking_hold();
                return;
            }
        }
        if self.phase == RunnerPhase::Paused {
            self.update_pause_ready_players(triggered, evaluation_enabled);
            if self.ready_to_run(evaluation_enabled) {
                self.begin_pause_resuming_countdown();
            }
            return;
        }
        if self.phase == RunnerPhase::TrackingHold {
            self.update_ready_players(active, triggered, evaluation_enabled);
            if self.ready_to_run(evaluation_enabled) {
                self.begin_resuming_countdown();
            }
            return;
        }
        if matches!(
            self.phase,
            RunnerPhase::Starting | RunnerPhase::PauseResuming | RunnerPhase::Resuming
        ) {
            if !self.has_required_evaluation(evaluation_enabled) {
                match self.phase {
                    RunnerPhase::Starting => self.return_to_ready(),
                    RunnerPhase::PauseResuming => self.return_to_paused(),
                    RunnerPhase::Resuming => self.begin_tracking_hold(),
                    _ => unreachable!(),
                }
                return;
            }
            self.advance_countdown(dt);
            return;
        }
        if self.phase == RunnerPhase::Ready {
            self.update_ready_players(active, triggered, evaluation_enabled);
            if self.ready_to_run(evaluation_enabled) {
                self.begin_starting_countdown();
            }
            return;
        }

        self.run_elapsed_seconds = (self.run_elapsed_seconds + dt).min(RUNNER_SESSION_SECONDS);
        if self.run_elapsed_seconds >= RUNNER_SESSION_SECONDS {
            self.finish_run(RunnerOutcome::BreakComplete);
            return;
        }

        self.apply_actions(active, triggered);
        self.advance_beat(dt);
        let intensity = self.breath_intensity();
        self.speed = (0.205 + self.world_distance / 12_000.0).min(0.31) * (1.0 + intensity * 0.18);
        self.world_distance += (18.0 + self.speed * 22.0) * dt;

        for (index, player) in self.players.iter_mut().enumerate() {
            if !player.evaluated || player.lives == 0 {
                continue;
            }
            self.score_fraction[index] += dt * (9.0 + self.speed * 14.0 + intensity * 8.0);
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
            self.finish_run(RunnerOutcome::EnergySpent);
        }
    }

    pub fn run_progress(&self) -> f32 {
        (self.run_elapsed_seconds / RUNNER_SESSION_SECONDS).clamp(0.0, 1.0)
    }

    pub fn run_remaining_seconds(&self) -> f32 {
        (RUNNER_SESSION_SECONDS - self.run_elapsed_seconds).max(0.0)
    }

    /// Normalized emotional intensity (0..1) following a Rise → Peak → Release arc.
    ///
    /// - **Rise** (0–55 s): smoothstep 0 → 1, building energy.
    /// - **Peak** (55–75 s): held at 1.0, maximum density and reward.
    /// - **Release** (75–90 s): cosine ease-out 1 → 0.3, triumphant cool-down.
    pub fn breath_intensity(&self) -> f32 {
        let t = self.run_elapsed_seconds;
        if t < BREATH_RISE_END {
            let x = t / BREATH_RISE_END;
            x * x * (3.0 - 2.0 * x)
        } else if t < BREATH_PEAK_END {
            1.0
        } else {
            let x = ((t - BREATH_PEAK_END) / (RUNNER_SESSION_SECONDS - BREATH_PEAK_END)).min(1.0);
            0.3 + 0.7 * (1.0 - x) * (1.0 - x)
        }
    }

    fn has_required_evaluation(&self, evaluation_enabled: [bool; PLAYER_CAPACITY]) -> bool {
        if self.mode == GameMode::DuoGroove {
            evaluation_enabled[0] && evaluation_enabled[1]
        } else {
            evaluation_enabled.into_iter().any(|enabled| enabled)
        }
    }

    fn pause_requested(
        triggered: [u32; PLAYER_CAPACITY],
        evaluation_enabled: [bool; PLAYER_CAPACITY],
    ) -> bool {
        triggered
            .into_iter()
            .zip(evaluation_enabled)
            .any(|(mask, enabled)| enabled && mask & Action::Pause.mask() != 0)
    }

    fn update_ready_players(
        &mut self,
        active: [u32; PLAYER_CAPACITY],
        triggered: [u32; PLAYER_CAPACITY],
        evaluation_enabled: [bool; PLAYER_CAPACITY],
    ) {
        for index in 0..PLAYER_CAPACITY {
            if evaluation_enabled[index] {
                if triggered[index] != 0 || active[index] != 0 {
                    self.ready_players[index] = true;
                }
            } else {
                self.ready_players[index] = false;
            }
        }
    }

    fn ready_to_run(&self, evaluation_enabled: [bool; PLAYER_CAPACITY]) -> bool {
        if self.mode == GameMode::DuoGroove {
            evaluation_enabled[0]
                && evaluation_enabled[1]
                && self.ready_players[0]
                && self.ready_players[1]
        } else {
            self.ready_players
                .into_iter()
                .zip(evaluation_enabled)
                .any(|(ready, enabled)| ready && enabled)
        }
    }

    fn update_pause_ready_players(
        &mut self,
        triggered: [u32; PLAYER_CAPACITY],
        evaluation_enabled: [bool; PLAYER_CAPACITY],
    ) {
        for index in 0..PLAYER_CAPACITY {
            if evaluation_enabled[index] {
                if triggered[index] & Action::Clap.mask() != 0 {
                    self.ready_players[index] = true;
                }
            } else {
                self.ready_players[index] = false;
            }
        }
    }

    fn begin_paused(&mut self) {
        self.phase = RunnerPhase::Paused;
        self.obstacles = [None; RUNNER_OBSTACLE_CAPACITY];
        self.ready_players = [false; PLAYER_CAPACITY];
        self.countdown_remaining = 0.0;
    }

    pub(crate) fn finish_run(&mut self, outcome: RunnerOutcome) {
        self.phase = RunnerPhase::GameOver;
        self.obstacles = [None; RUNNER_OBSTACLE_CAPACITY];
        self.ready_players = [false; PLAYER_CAPACITY];
        self.countdown_remaining = 0.0;
        self.result_outcome = Some(outcome);
        for player in &mut self.players {
            player.new_best = player.score > player.best_score;
            player.best_score = player.best_score.max(player.score);
        }
        self.result_selection = self.mode;
        self.pending_result_event = Some(RunnerResultEvent {
            mode: self.mode,
            outcome,
        });
    }

    fn begin_tracking_hold(&mut self) {
        self.phase = RunnerPhase::TrackingHold;
        self.obstacles = [None; RUNNER_OBSTACLE_CAPACITY];
        self.ready_players = [false; PLAYER_CAPACITY];
        self.countdown_remaining = 0.0;
    }

    fn begin_starting_countdown(&mut self) {
        self.phase = RunnerPhase::Starting;
        self.countdown_remaining = RUNNER_COUNTDOWN_SECONDS;
    }

    fn begin_resuming_countdown(&mut self) {
        self.phase = RunnerPhase::Resuming;
        self.countdown_remaining = RUNNER_COUNTDOWN_SECONDS;
    }

    fn begin_pause_resuming_countdown(&mut self) {
        self.phase = RunnerPhase::PauseResuming;
        self.countdown_remaining = RUNNER_COUNTDOWN_SECONDS;
    }

    fn advance_countdown(&mut self, dt: f32) {
        self.countdown_remaining = (self.countdown_remaining - dt).max(0.0);
        if self.countdown_remaining <= 0.000_1 {
            self.countdown_remaining = 0.0;
            self.phase = RunnerPhase::Running;
        }
    }

    fn return_to_ready(&mut self) {
        self.phase = RunnerPhase::Ready;
        self.ready_players = [false; PLAYER_CAPACITY];
        self.countdown_remaining = 0.0;
    }

    fn return_to_paused(&mut self) {
        self.phase = RunnerPhase::Paused;
        self.ready_players = [false; PLAYER_CAPACITY];
        self.countdown_remaining = 0.0;
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
            let spawn_interval = if self.breath_intensity() > 0.75 {
                2
            } else if self.breath_intensity() > 0.35 {
                3
            } else {
                4
            };
            if self.beat_index % spawn_interval == 1 {
                let (kind, lane) = RUNNER_PATTERN[self.pattern_cursor % RUNNER_PATTERN.len()];
                self.spawn(kind, lane);
                self.pattern_cursor = self.pattern_cursor.wrapping_add(1);
            }
        }
        self.beat_progress = self.beat_accumulator / RUNNER_BEAT_SECONDS;
    }

    pub(crate) fn spawn(&mut self, kind: RunnerHazard, lane: i8) {
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
                        player.best_combo = player.best_combo.max(player.combo);
                        player.score = player
                            .score
                            .saturating_add(250 + u32::from(player.combo.min(20)) * 15);
                        self.feedback[player_index] = RunnerFeedback::BeatPickup;
                    }
                } else if success {
                    player.combo = player.combo.saturating_add(1);
                    player.best_combo = player.best_combo.max(player.combo);
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

    pub fn take_next_run_request(&mut self) -> Option<GameMode> {
        self.next_run_request.take()
    }

    pub fn take_result_event(&mut self) -> Option<RunnerResultEvent> {
        self.pending_result_event.take()
    }

    pub fn start_run(&mut self, mode: GameMode, evaluation_enabled: [bool; PLAYER_CAPACITY]) {
        let best_scores = self
            .players
            .map(|player| player.best_score.max(player.score));
        *self = Self::with_mode(mode);
        for index in 0..PLAYER_CAPACITY {
            self.players[index].best_score = best_scores[index];
            self.players[index].evaluated = evaluation_enabled[index];
        }
        let ready_to_run = if mode == GameMode::DuoGroove {
            evaluation_enabled[0] && evaluation_enabled[1]
        } else {
            evaluation_enabled.into_iter().any(|enabled| enabled)
        };
        if ready_to_run {
            self.begin_starting_countdown();
        }
    }
}
