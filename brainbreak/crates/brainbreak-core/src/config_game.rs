//! Generic config-driven game interpreter.
//!
//! Runs any game defined by a [`GameConfig`] — dispatches to mechanic-specific
//! update handlers while sharing a common phase machine, scoring, and pacing.

use crate::PLAYER_CAPACITY;
use crate::config::{EndMode, GameConfig, Mechanic};

/// Countdown duration before active play begins.
const COUNTDOWN_SECONDS: f32 = 2.0;
/// Beat window fraction for on-beat detection.
const ON_BEAT_WINDOW: f32 = 0.35;
/// Maximum entities (obstacles/targets) alive at once.
const MAX_ENTITIES: usize = 8;
/// Pattern sequence length.
const PATTERN_LENGTH: usize = 4;

/// Phase of a config-driven game session.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ConfigPhase {
    #[default]
    Ready,
    Countdown,
    Active,
    Result,
}

/// Outcome of a completed session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigOutcome {
    /// Timer expired or goal reached — positive ending.
    Complete,
    /// Lives depleted — early end.
    Depleted,
}

/// A single moving entity (obstacle or target).
#[derive(Clone, Copy, Debug, Default)]
pub struct Entity {
    /// Lane or horizontal slot (-1, 0, 1 for dodge; x-position for catch).
    pub lane: i8,
    /// Distance from player: 1.0 = horizon, 0.0 = collision line.
    pub distance: f32,
    /// Required action bitmask to resolve (0 = avoid by not being in lane).
    pub action: u32,
    /// True if this entity has been resolved/hit.
    pub resolved: bool,
}

/// Per-player state in a config game.
#[derive(Clone, Copy, Debug, Default)]
pub struct ConfigPlayer {
    pub score: u32,
    pub combo: u16,
    pub best_combo: u16,
    pub lives: u8,
    pub evaluated: bool,
    /// Current lane position (for dodge/catch).
    pub lane: i8,
    /// Hold progress 0→1 (for hold mechanic).
    pub hold_progress: f32,
}

/// Generic game state driven entirely by [`GameConfig`].
#[derive(Clone, Debug)]
pub struct ConfigGame {
    pub phase: ConfigPhase,
    pub outcome: Option<ConfigOutcome>,
    pub players: [ConfigPlayer; PLAYER_CAPACITY],
    pub entities: [Entity; MAX_ENTITIES],
    pub entity_count: usize,
    /// Session elapsed time (seconds).
    pub elapsed: f32,
    /// Countdown remaining.
    pub countdown_remaining: f32,
    /// Current intensity (from pacing curve).
    pub intensity: f32,
    /// Beat tracking.
    pub beat_progress: f32,
    /// Energy accumulator (pump mechanic).
    pub energy: f32,
    /// Pattern sequence (pattern mechanic).
    pub pattern: [u32; PATTERN_LENGTH],
    /// Current pattern index being evaluated.
    pub pattern_index: usize,
    /// True on the frame a hit/score event occurs (for renderer feedback).
    pub hit_flash: bool,
    /// True on the frame a miss/crash occurs.
    pub miss_flash: bool,
    /// Best score across sessions.
    pub best_score: u32,
    /// True if current score is a new best.
    pub new_best: bool,
    // Internal timers
    spawn_timer: f32,
    beat_accumulator: f32,
}

impl ConfigGame {
    /// Create a new game interpreter for the given config.
    pub fn new(_config: &GameConfig) -> Self {
        Self {
            phase: ConfigPhase::Ready,
            outcome: None,
            players: [ConfigPlayer::default(); PLAYER_CAPACITY],
            entities: [Entity::default(); MAX_ENTITIES],
            entity_count: 0,
            elapsed: 0.0,
            countdown_remaining: 0.0,
            intensity: 0.0,
            beat_progress: 0.0,
            energy: 0.0,
            pattern: [0; PATTERN_LENGTH],
            pattern_index: 0,
            hit_flash: false,
            miss_flash: false,
            best_score: 0,
            new_best: false,
            spawn_timer: 0.0,
            beat_accumulator: 0.0,
        }
    }

    /// Transition from Ready → Countdown.
    pub fn start_run(&mut self, config: &GameConfig, evaluation: [bool; PLAYER_CAPACITY]) {
        self.phase = ConfigPhase::Countdown;
        self.countdown_remaining = COUNTDOWN_SECONDS;
        self.elapsed = 0.0;
        self.energy = 0.0;
        self.entity_count = 0;
        self.pattern_index = 0;
        self.outcome = None;
        self.new_best = false;
        for (i, player) in self.players.iter_mut().enumerate() {
            *player = ConfigPlayer {
                evaluated: evaluation[i],
                lives: config.end.lives,
                ..ConfigPlayer::default()
            };
        }
        // Seed pattern from config actions (deterministic).
        let actions: Vec<u32> = config.active_actions().collect();
        for (i, slot) in self.pattern.iter_mut().enumerate() {
            *slot = if actions.is_empty() { 0 } else { actions[i % actions.len()] };
        }
    }

    /// Request replay from Result phase. Returns true if replay started.
    pub fn request_replay(&mut self, config: &GameConfig) -> bool {
        if self.phase != ConfigPhase::Result {
            return false;
        }
        let evaluation = self.players.map(|p| p.evaluated);
        self.start_run(config, evaluation);
        true
    }

    /// Main update tick.
    pub fn update(&mut self, dt: f32, config: &GameConfig, triggered: [u32; PLAYER_CAPACITY]) {
        self.hit_flash = false;
        self.miss_flash = false;

        match self.phase {
            ConfigPhase::Ready => {}
            ConfigPhase::Countdown => {
                self.countdown_remaining -= dt;
                if self.countdown_remaining <= 0.0 {
                    self.phase = ConfigPhase::Active;
                }
            }
            ConfigPhase::Active => {
                self.elapsed += dt;
                let progress = (self.elapsed / config.pacing.session_seconds).clamp(0.0, 1.0);
                self.intensity = config.pacing.curve.intensity(progress);

                // Beat tracking.
                let beat_seconds = 60.0 / config.pacing.bpm;
                self.beat_accumulator += dt;
                self.beat_progress = (self.beat_accumulator % beat_seconds) / beat_seconds;

                // Dispatch to mechanic handler.
                match config.mechanic {
                    Mechanic::Dodge => self.update_dodge(dt, config, triggered),
                    Mechanic::Pump => self.update_pump(dt, config, triggered),
                    Mechanic::Catch => self.update_catch(dt, config, triggered),
                    Mechanic::Hold => self.update_hold(dt, config, triggered),
                    Mechanic::Pattern => self.update_pattern(dt, config, triggered),
                }

                // Check end conditions.
                self.check_end(config);
            }
            ConfigPhase::Result => {}
        }
    }

    /// Total score across all evaluated players.
    pub fn total_score(&self) -> u32 {
        self.players.iter().map(|p| p.score).sum()
    }

    /// Session progress 0→1.
    pub fn progress(&self, config: &GameConfig) -> f32 {
        (self.elapsed / config.pacing.session_seconds).clamp(0.0, 1.0)
    }

    // --- Mechanic handlers ---

    fn update_dodge(&mut self, dt: f32, config: &GameConfig, triggered: [u32; PLAYER_CAPACITY]) {
        let speed = (0.4 + 0.6 * self.intensity) * config.challenge.speed_ramp;

        // Spawn obstacles.
        self.spawn_timer -= dt;
        let interval = config.challenge.spawn_interval * (1.0 - 0.4 * self.intensity);
        if self.spawn_timer <= 0.0 && self.entity_count < MAX_ENTITIES {
            self.spawn_timer = interval;
            let actions: Vec<u32> = config.active_actions().collect();
            let action = if actions.is_empty() { 0 } else { actions[(self.elapsed * 7.0) as usize % actions.len()] };
            let lane = ((self.elapsed * 3.7) as i8 % 3) - 1;
            self.entities[self.entity_count] = Entity {
                lane,
                distance: 1.0,
                action,
                resolved: false,
            };
            self.entity_count += 1;
        }

        // Move entities toward player.
        for entity in self.entities[..self.entity_count].iter_mut() {
            if !entity.resolved {
                entity.distance -= speed * dt;
            }
        }

        // Check player actions against entities in collision range.
        let mask = triggered.iter().fold(0, |acc, &t| acc | t);
        let combo_step = config.scoring.combo_step;
        for pi in 0..PLAYER_CAPACITY {
            if !self.players[pi].evaluated { continue; }
            // Lane movement.
            if mask & 1 != 0 { self.players[pi].lane = (self.players[pi].lane - 1).max(-1); }
            if mask & 2 != 0 { self.players[pi].lane = (self.players[pi].lane + 1).min(1); }

            for ei in 0..self.entity_count {
                if self.entities[ei].resolved || self.entities[ei].distance > 0.15 || self.entities[ei].distance < 0.0 {
                    continue;
                }
                if self.entities[ei].action != 0 && mask & self.entities[ei].action != 0 {
                    // Correct action — resolve.
                    self.entities[ei].resolved = true;
                    let combo_mult = 1.0 + self.players[pi].combo as f32 * combo_step;
                    self.players[pi].combo += 1;
                    self.players[pi].best_combo = self.players[pi].best_combo.max(self.players[pi].combo);
                    self.players[pi].score += (10.0 * combo_mult) as u32;
                    self.hit_flash = true;
                } else if self.entities[ei].distance <= 0.085 && self.entities[ei].lane == self.players[pi].lane {
                    // Collision.
                    self.entities[ei].resolved = true;
                    self.players[pi].combo = 0;
                    self.players[pi].lives = self.players[pi].lives.saturating_sub(1);
                    self.miss_flash = true;
                }
            }
        }

        // Compact: remove passed entities.
        let mut write = 0;
        for read in 0..self.entity_count {
            if self.entities[read].distance > -0.1 {
                self.entities[write] = self.entities[read];
                write += 1;
            }
        }
        self.entity_count = write;
    }

    fn update_pump(&mut self, _dt: f32, config: &GameConfig, triggered: [u32; PLAYER_CAPACITY]) {
        let beat_seconds = 60.0 / config.pacing.bpm;
        let phase_in_beat = self.beat_accumulator % beat_seconds / beat_seconds;
        let on_beat = !(ON_BEAT_WINDOW..=(1.0 - ON_BEAT_WINDOW)).contains(&phase_in_beat);

        let active_players = self.players.iter().filter(|p| p.evaluated).count();
        let coop_mult = if active_players >= 2 { config.scoring.coop_bonus } else { 1.0 };

        for (i, player) in self.players.iter_mut().enumerate() {
            if !player.evaluated || triggered[i] == 0 {
                continue;
            }
            if !config.matches_action(triggered[i]) {
                continue;
            }
            let base = if on_beat { 0.03 } else { 0.01 };
            let combo_mult = 1.0 + player.combo as f32 * config.scoring.combo_step;
            self.energy += base * combo_mult * coop_mult;
            if on_beat {
                player.combo += 1;
                player.best_combo = player.best_combo.max(player.combo);
                self.hit_flash = true;
            } else {
                player.combo = 0;
            }
            player.score += (10.0 * combo_mult) as u32;
        }
        self.energy = self.energy.min(1.0);
    }

    fn update_catch(&mut self, dt: f32, config: &GameConfig, triggered: [u32; PLAYER_CAPACITY]) {
        let speed = 0.3 + 0.5 * self.intensity;

        // Spawn targets.
        self.spawn_timer -= dt;
        let interval = config.challenge.spawn_interval * (1.0 - 0.3 * self.intensity);
        if self.spawn_timer <= 0.0 && self.entity_count < MAX_ENTITIES {
            self.spawn_timer = interval;
            let lane = ((self.elapsed * 5.3) as i8 % 3) - 1;
            self.entities[self.entity_count] = Entity {
                lane,
                distance: 1.0,
                action: 0,
                resolved: false,
            };
            self.entity_count += 1;
        }

        // Move targets down.
        for entity in self.entities[..self.entity_count].iter_mut() {
            entity.distance -= speed * dt;
        }

        // Player movement + catch detection.
        let combo_step = config.scoring.combo_step;
        for (pi, &mask) in triggered.iter().enumerate() {
            if !self.players[pi].evaluated { continue; }
            if mask & 1 != 0 { self.players[pi].lane = (self.players[pi].lane - 1).max(-1); }
            if mask & 2 != 0 { self.players[pi].lane = (self.players[pi].lane + 1).min(1); }

            for ei in 0..self.entity_count {
                if self.entities[ei].resolved { continue; }
                if self.entities[ei].distance <= 0.1 && self.entities[ei].distance >= -0.05
                    && self.entities[ei].lane == self.players[pi].lane
                {
                    self.entities[ei].resolved = true;
                    let combo_mult = 1.0 + self.players[pi].combo as f32 * combo_step;
                    self.players[pi].combo += 1;
                    self.players[pi].best_combo = self.players[pi].best_combo.max(self.players[pi].combo);
                    self.players[pi].score += (10.0 * combo_mult) as u32;
                    self.hit_flash = true;
                }
            }
        }

        // Missed targets (fell past).
        for ei in 0..self.entity_count {
            if !self.entities[ei].resolved && self.entities[ei].distance < -0.05 {
                self.entities[ei].resolved = true;
                for pi in 0..PLAYER_CAPACITY {
                    if self.players[pi].evaluated {
                        self.players[pi].combo = 0;
                    }
                }
                self.miss_flash = true;
            }
        }
        // Compact: remove fallen entities.
        let mut write = 0;
        for read in 0..self.entity_count {
            if self.entities[read].distance > -0.1 {
                self.entities[write] = self.entities[read];
                write += 1;
            }
        }
        self.entity_count = write;
    }

    fn update_hold(&mut self, dt: f32, config: &GameConfig, triggered: [u32; PLAYER_CAPACITY]) {
        let required_duration = 1.0 + self.intensity; // harder = longer holds
        let any_holding = (0..PLAYER_CAPACITY).any(|i| {
            self.players[i].evaluated && config.matches_action(triggered[i])
        });

        let combo_step = config.scoring.combo_step;
        for i in 0..PLAYER_CAPACITY {
            if !self.players[i].evaluated { continue; }
            if any_holding {
                self.players[i].hold_progress += dt / required_duration;
                if self.players[i].hold_progress >= 1.0 {
                    self.players[i].hold_progress = 0.0;
                    let combo_mult = 1.0 + self.players[i].combo as f32 * combo_step;
                    self.players[i].combo += 1;
                    self.players[i].best_combo = self.players[i].best_combo.max(self.players[i].combo);
                    self.players[i].score += (10.0 * combo_mult) as u32;
                    self.hit_flash = true;
                }
            } else {
                // Decay on release.
                self.players[i].hold_progress = (self.players[i].hold_progress - dt * 2.0).max(0.0);
            }
        }
    }

    fn update_pattern(&mut self, _dt: f32, config: &GameConfig, triggered: [u32; PLAYER_CAPACITY]) {
        if self.pattern_index >= PATTERN_LENGTH {
            // Sequence complete — award and reset.
            let combo_step = config.scoring.combo_step;
            for i in 0..PLAYER_CAPACITY {
                if !self.players[i].evaluated { continue; }
                let combo_mult = 1.0 + self.players[i].combo as f32 * combo_step;
                self.players[i].combo += 1;
                self.players[i].best_combo = self.players[i].best_combo.max(self.players[i].combo);
                self.players[i].score += (10.0 * combo_mult) as u32;
            }
            self.hit_flash = true;
            self.pattern_index = 0;
            // Shuffle pattern deterministically.
            let actions: Vec<u32> = config.active_actions().collect();
            if !actions.is_empty() {
                for (i, slot) in self.pattern.iter_mut().enumerate() {
                    *slot = actions[(i + self.elapsed as usize) % actions.len()];
                }
            }
            return;
        }

        let expected = self.pattern[self.pattern_index];
        for (i, &mask) in triggered.iter().enumerate() {
            if !self.players[i].evaluated || mask == 0 { continue; }
            if mask & expected != 0 {
                self.pattern_index += 1;
                self.hit_flash = true;
                self.players[i].combo += 1;
                self.players[i].best_combo = self.players[i].best_combo.max(self.players[i].combo);
                break; // One player advancing is enough (co-op).
            } else if config.matches_action(mask) {
                // Wrong action.
                self.players[i].combo = 0;
                self.miss_flash = true;
            }
        }
    }

    // --- Shared helpers ---

    fn check_end(&mut self, config: &GameConfig) {
        let ended = match config.end.mode {
            EndMode::TimeBased => self.elapsed >= config.pacing.session_seconds,
            EndMode::LifeBased => self.players.iter().filter(|p| p.evaluated).all(|p| p.lives == 0),
            EndMode::GoalBased => self.total_score() >= config.end.goal_score,
        };
        // Pump mechanic also ends at full energy.
        let pump_complete = config.mechanic == Mechanic::Pump && self.energy >= 1.0;

        if ended || pump_complete {
            self.phase = ConfigPhase::Result;
            let score = self.total_score();
            self.new_best = score > self.best_score;
            if self.new_best {
                self.best_score = score;
            }
            self.outcome = Some(match config.end.mode {
                EndMode::LifeBased if self.players.iter().filter(|p| p.evaluated).all(|p| p.lives == 0) => {
                    ConfigOutcome::Depleted
                }
                _ => ConfigOutcome::Complete,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::*;

    fn test_config(mechanic: Mechanic) -> GameConfig {
        GameConfig {
            schema_version: 1,
            id: "test".into(),
            title: "Test Game".into(),
            actions: [4, 8, 64], // Jump, Squat, Clap
            mechanic,
            scoring: ScoringConfig::default(),
            pacing: PacingConfig { session_seconds: 10.0, ..PacingConfig::default() },
            challenge: ChallengeConfig::default(),
            end: EndConfig::default(),
            theme: ThemeConfig::default(),
        }
    }

    fn evaluation() -> [bool; PLAYER_CAPACITY] {
        [true, false, false, false]
    }

    #[test]
    fn config_deserializes_from_json() {
        let json = r#"{
            "schema_version": 1,
            "id": "catch-frenzy",
            "title": "Catch Frenzy",
            "actions": [1, 2, 0],
            "mechanic": "catch",
            "scoring": { "combo_step": 0.2 },
            "pacing": { "bpm": 140, "curve": "waves" },
            "challenge": { "difficulty": 0.7 },
            "end": { "mode": "time-based" },
            "theme": { "style": "cosmic" }
        }"#;
        let config = GameConfig::from_json(json.as_bytes()).unwrap();
        assert_eq!(config.mechanic, Mechanic::Catch);
        assert_eq!(config.actions, [1, 2, 0]);
        assert_eq!(config.pacing.bpm, 140.0);
        assert_eq!(config.pacing.curve, IntensityCurve::Waves);
        assert_eq!(config.theme.style, ThemeStyle::Cosmic);
        assert_eq!(config.scoring.combo_step, 0.2);
        assert_eq!(config.end.lives, 3); // default
    }

    #[test]
    fn config_accepts_studio_export_shape() {
        // Exact JSON shape produced by web/src/studio.tsx buildConfig() for a Catch game.
        // Theme colors are emitted as decimal (0x00E5FF = 58879, 0xFF4081 = 16728193).
        let json = r#"{
            "schema_version": 1,
            "id": "my-motion-game",
            "title": "My Motion Game",
            "actions": [4, 8, 0],
            "mechanic": "catch",
            "scoring": { "combo_step": 0.15, "on_beat_bonus": 1.0, "coop_bonus": 1.0 },
            "pacing": { "bpm": 120, "session_seconds": 60, "curve": "breath-arc" },
            "challenge": { "spawn_interval": 1.2, "speed_ramp": 1.5, "difficulty": 0.5 },
            "end": { "mode": "time-based", "lives": 0, "goal_score": 0 },
            "theme": { "primary": 58879, "secondary": 16728193, "style": "neon" }
        }"#;
        let config = GameConfig::from_json(json.as_bytes()).unwrap();
        assert_eq!(config.mechanic, Mechanic::Catch);
        assert_eq!(config.actions, [4, 8, 0]);
        assert_eq!(config.theme.primary, 0x00E5FF);
        assert_eq!(config.theme.secondary, 0xFF4081);
        assert_eq!(config.theme.style, ThemeStyle::Neon);
        assert_eq!(config.end.mode, EndMode::TimeBased);
        assert_eq!(config.pacing.curve, IntensityCurve::BreathArc);
    }

    #[test]
    fn config_rejects_invalid_schema() {
        let json = r#"{ "schema_version": 99, "id": "x", "title": "X", "actions": [0,0,0], "mechanic": "dodge", "scoring": {}, "pacing": {}, "challenge": {}, "end": {}, "theme": {} }"#;
        assert!(GameConfig::from_json(json.as_bytes()).is_none());
    }

    #[test]
    fn config_rejects_empty_title() {
        let json = r#"{ "schema_version": 1, "id": "x", "title": "", "actions": [0,0,0], "mechanic": "dodge", "scoring": {}, "pacing": {}, "challenge": {}, "end": {}, "theme": {} }"#;
        assert!(GameConfig::from_json(json.as_bytes()).is_none());
    }

    #[test]
    fn matches_action_respects_allowed_slots() {
        let config = test_config(Mechanic::Dodge);
        assert!(config.matches_action(4));  // Jump allowed
        assert!(config.matches_action(64)); // Clap allowed
        assert!(!config.matches_action(1)); // Left not in actions
        assert!(!config.matches_action(0)); // Zero never matches
    }

    #[test]
    fn start_run_enters_countdown() {
        let config = test_config(Mechanic::Pump);
        let mut game = ConfigGame::new(&config);
        assert_eq!(game.phase, ConfigPhase::Ready);
        game.start_run(&config, evaluation());
        assert_eq!(game.phase, ConfigPhase::Countdown);
        assert!(game.players[0].evaluated);
        assert!(!game.players[1].evaluated);
    }

    #[test]
    fn countdown_transitions_to_active() {
        let config = test_config(Mechanic::Dodge);
        let mut game = ConfigGame::new(&config);
        game.start_run(&config, evaluation());
        game.update(1.0, &config, [0; 4]);
        game.update(1.1, &config, [0; 4]);
        assert_eq!(game.phase, ConfigPhase::Active);
    }

    #[test]
    fn pump_on_beat_adds_energy_and_combo() {
        let config = test_config(Mechanic::Pump);
        let mut game = ConfigGame::new(&config);
        game.start_run(&config, evaluation());
        // Skip countdown.
        game.update(2.1, &config, [0; 4]);
        assert_eq!(game.phase, ConfigPhase::Active);
        // Action on first frame (beat_progress near 0 = on-beat).
        game.update(0.016, &config, [4, 0, 0, 0]); // Jump
        assert!(game.energy > 0.0);
        assert_eq!(game.players[0].combo, 1);
    }

    #[test]
    fn time_based_end_completes_session() {
        let config = test_config(Mechanic::Hold);
        let mut game = ConfigGame::new(&config);
        game.start_run(&config, evaluation());
        game.update(2.1, &config, [0; 4]); // past countdown
        game.update(10.1, &config, [0; 4]); // past session
        assert_eq!(game.phase, ConfigPhase::Result);
        assert_eq!(game.outcome, Some(ConfigOutcome::Complete));
    }

    #[test]
    fn life_based_end_depletes() {
        let mut config = test_config(Mechanic::Dodge);
        config.end.mode = EndMode::LifeBased;
        config.end.lives = 1;
        let mut game = ConfigGame::new(&config);
        game.start_run(&config, evaluation());
        game.update(2.1, &config, [0; 4]);
        // Manually deplete lives.
        game.players[0].lives = 0;
        game.update(0.016, &config, [0; 4]);
        assert_eq!(game.phase, ConfigPhase::Result);
        assert_eq!(game.outcome, Some(ConfigOutcome::Depleted));
    }

    #[test]
    fn goal_based_end_triggers_at_target_score() {
        let mut config = test_config(Mechanic::Pump);
        config.end.mode = EndMode::GoalBased;
        config.end.goal_score = 50;
        let mut game = ConfigGame::new(&config);
        game.start_run(&config, evaluation());
        game.update(2.1, &config, [0; 4]);
        game.players[0].score = 60;
        game.update(0.016, &config, [0; 4]);
        assert_eq!(game.phase, ConfigPhase::Result);
        assert_eq!(game.outcome, Some(ConfigOutcome::Complete));
    }

    #[test]
    fn intensity_curves_are_bounded() {
        for curve in [IntensityCurve::BreathArc, IntensityCurve::SteadyRamp, IntensityCurve::Waves] {
            for i in 0..=100 {
                let t = i as f32 / 100.0;
                let v = curve.intensity(t);
                assert!((0.0..=1.0).contains(&v), "{curve:?} at t={t} gave {v}");
            }
            assert!((curve.intensity(0.0) - 0.0).abs() < 0.01 || curve == IntensityCurve::Waves);
        }
        assert!((IntensityCurve::SteadyRamp.intensity(1.0) - 1.0).abs() < 0.001);
        assert!((IntensityCurve::BreathArc.intensity(0.7) - 1.0).abs() < 0.001);
    }

    #[test]
    fn replay_resets_from_result() {
        let config = test_config(Mechanic::Hold);
        let mut game = ConfigGame::new(&config);
        game.start_run(&config, evaluation());
        game.update(2.1, &config, [0; 4]);
        game.update(10.1, &config, [0; 4]);
        assert_eq!(game.phase, ConfigPhase::Result);
        assert!(game.request_replay(&config));
        assert_eq!(game.phase, ConfigPhase::Countdown);
    }

    #[test]
    fn max_three_actions_enforced() {
        let config = test_config(Mechanic::Dodge);
        let active: Vec<u32> = config.active_actions().collect();
        assert!(active.len() <= 3);
        // Config with all zeros.
        let mut empty = config.clone();
        empty.actions = [0, 0, 0];
        assert_eq!(empty.active_actions().count(), 0);
    }

    #[test]
    fn pattern_advance_on_correct_action() {
        let config = test_config(Mechanic::Pattern);
        let mut game = ConfigGame::new(&config);
        game.start_run(&config, evaluation());
        game.update(2.1, &config, [0; 4]); // past countdown
        let expected = game.pattern[0];
        game.update(0.016, &config, [expected, 0, 0, 0]);
        assert_eq!(game.pattern_index, 1);
        assert!(game.hit_flash);
    }

    #[test]
    fn best_score_tracked_across_sessions() {
        let config = test_config(Mechanic::Hold);
        let mut game = ConfigGame::new(&config);
        game.start_run(&config, evaluation());
        game.update(2.1, &config, [0; 4]);
        game.players[0].score = 42;
        game.update(10.1, &config, [0; 4]);
        assert_eq!(game.best_score, 42);
        assert!(game.new_best);
        // Second run with lower score.
        game.request_replay(&config);
        game.update(2.1, &config, [0; 4]);
        game.update(10.1, &config, [0; 4]);
        assert_eq!(game.best_score, 42);
        assert!(!game.new_best);
    }
}
