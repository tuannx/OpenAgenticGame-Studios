//! Supernova Freeze Party — a Freeze-Dance brain break for ages 4–7.
//!
//! Core loop built on the classic children's game "Freeze Dance" (a.k.a.
//! Frozen Freeze / Red-Light-Green-Light): dance while the music plays, then
//! hold perfectly still when it stops. Every kid already knows the rules, so
//! the learning curve is zero. Perfect freezes earn stars; dancing charges a
//! shared energy core that finally bursts into a Supernova celebration.

use serde::{Deserialize, Serialize};

use crate::PLAYER_CAPACITY;
use crate::pose::Action;

/// Beats per minute for the Supernova track (matches global 126 BPM).
const SUPERNOVA_BPM: f32 = 126.0;
const SUPERNOVA_BEAT_SECONDS: f32 = 60.0 / SUPERNOVA_BPM;

/// Dance phase duration: music on, move to charge energy.
pub const SUPERNOVA_DANCE_SECONDS: f32 = 8.0;
/// Freeze phase duration: music stops, hold still.
pub const SUPERNOVA_FREEZE_SECONDS: f32 = 3.0;
/// Drop (supernova celebration) animation duration.
pub const SUPERNOVA_DROP_SECONDS: f32 = 3.0;
/// Countdown before the first dance round.
pub const SUPERNOVA_COUNTDOWN_SECONDS: f32 = 2.0;
/// Total session safety cap (time-boxed positive break).
pub const SUPERNOVA_SESSION_SECONDS: f32 = 75.0;

/// Beat window: actions within this fraction of a beat are "on-beat".
const ON_BEAT_WINDOW: f32 = 0.35;
/// Energy added per on-beat dance action (before combo multiplier).
const BASE_ENERGY_PER_HIT: f32 = 0.03;
/// Energy added for off-beat actions (still rewards movement).
const OFF_BEAT_ENERGY: f32 = 0.01;
/// Combo multiplier step: each combo level adds this to the multiplier.
const COMBO_MULTIPLIER_STEP: f32 = 0.15;
/// Maximum combo multiplier cap.
const MAX_COMBO_MULTIPLIER: f32 = 3.0;
/// Co-op bonus: energy multiplier when 2+ players are dancing.
const COOP_ENERGY_BONUS: f32 = 1.5;
/// Reaction-time grace at the start of a freeze (no penalty).
const FREEZE_GRACE_SECONDS: f32 = 0.35;
/// Allowed cumulative movement time during a freeze (forgives jitter).
const FREEZE_MOVE_BUDGET: f32 = 0.25;
/// Energy bonus awarded for a perfect freeze.
const PERFECT_FREEZE_ENERGY: f32 = 0.16;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SupernovaPhase {
    #[default]
    Ready,
    Countdown,
    /// Music on — dance to charge the core.
    Dance,
    /// Music stops — hold perfectly still.
    Freeze,
    /// Supernova celebration.
    Drop,
    Result,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SupernovaOutcome {
    /// Energy reached 100% — full supernova.
    FullSupernova,
    /// Session time expired before full energy — partial burst.
    TimeExpired,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SupernovaPlayer {
    pub energy_contributed: f32,
    pub hits: u16,
    pub on_beat_hits: u16,
    pub combo: u16,
    pub best_combo: u16,
    pub evaluated: bool,
}

impl Default for SupernovaPlayer {
    fn default() -> Self {
        Self {
            energy_contributed: 0.0,
            hits: 0,
            on_beat_hits: 0,
            combo: 0,
            best_combo: 0,
            evaluated: false,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SupernovaFeedback {
    /// True on the frame an action was on-beat.
    pub on_beat: bool,
    /// True on the frame an action was registered (any timing).
    pub hit: bool,
}

/// Deterministic Supernova Freeze Party game state.
///
/// Players alternate DANCE rounds (move on-beat to charge a shared core) with
/// FREEZE rounds (hold still to earn stars and bonus energy). When the core
/// fills, it goes supernova.
#[derive(Clone, Debug)]
pub struct SupernovaGame {
    pub phase: SupernovaPhase,
    pub players: [SupernovaPlayer; PLAYER_CAPACITY],
    pub feedback: [SupernovaFeedback; PLAYER_CAPACITY],
    /// Shared core energy: 0.0 → 1.0
    pub energy: f32,
    /// Current combo (shared across players for co-op feel).
    pub combo: u16,
    pub best_combo: u16,
    /// Stars earned from perfect freezes.
    pub freeze_stars: u8,
    /// Current dance/freeze round number (1-based).
    pub round: u8,
    /// Final score computed at drop.
    pub score: u32,
    pub best_score: u32,
    pub new_best: bool,
    pub outcome: Option<SupernovaOutcome>,
    /// Countdown remaining (seconds).
    pub countdown_remaining: f32,
    /// Time remaining in the current Dance/Freeze/Drop phase.
    pub phase_timer: f32,
    /// Total session time remaining (safety cap).
    pub session_remaining: f32,
    /// Freeze progress 0.0 → 1.0 (for the freeze meter).
    pub freeze_progress: f32,
    /// True on the exact frame a perfect freeze is awarded (renderer pop).
    pub perfect_freeze: bool,
    /// True on frames where someone is moving during a freeze (wobble).
    pub freeze_wobble: bool,
    /// Beat tracking.
    pub beat_index: u32,
    pub beat_progress: f32,
    /// True on the exact frame the drop triggers (for renderer explosion).
    pub drop_triggered: bool,
    /// Accumulated time for beat tracking.
    beat_accumulator: f32,
    /// Cooldown to prevent double-triggering on same action.
    hit_cooldown: [f32; PLAYER_CAPACITY],
    /// Elapsed time within the current freeze phase.
    freeze_elapsed: f32,
    /// Cumulative movement time during the current freeze.
    freeze_move_time: f32,
}

impl Default for SupernovaGame {
    fn default() -> Self {
        Self::new()
    }
}

impl SupernovaGame {
    pub fn new() -> Self {
        Self {
            phase: SupernovaPhase::Ready,
            players: [SupernovaPlayer::default(); PLAYER_CAPACITY],
            feedback: [SupernovaFeedback::default(); PLAYER_CAPACITY],
            energy: 0.0,
            combo: 0,
            best_combo: 0,
            freeze_stars: 0,
            round: 0,
            score: 0,
            best_score: 0,
            new_best: false,
            outcome: None,
            countdown_remaining: 0.0,
            phase_timer: 0.0,
            session_remaining: SUPERNOVA_SESSION_SECONDS,
            freeze_progress: 0.0,
            perfect_freeze: false,
            freeze_wobble: false,
            beat_index: 0,
            beat_progress: 0.0,
            drop_triggered: false,
            beat_accumulator: 0.0,
            hit_cooldown: [0.0; PLAYER_CAPACITY],
            freeze_elapsed: 0.0,
            freeze_move_time: 0.0,
        }
    }

    /// Start a new run (transitions Ready → Countdown).
    pub fn start_run(&mut self, evaluation_enabled: [bool; PLAYER_CAPACITY]) {
        self.phase = SupernovaPhase::Countdown;
        self.countdown_remaining = SUPERNOVA_COUNTDOWN_SECONDS;
        self.energy = 0.0;
        self.combo = 0;
        self.best_combo = 0;
        self.freeze_stars = 0;
        self.round = 0;
        self.score = 0;
        self.new_best = false;
        self.outcome = None;
        self.phase_timer = 0.0;
        self.session_remaining = SUPERNOVA_SESSION_SECONDS;
        self.freeze_progress = 0.0;
        self.perfect_freeze = false;
        self.freeze_wobble = false;
        self.beat_index = 0;
        self.beat_progress = 0.0;
        self.beat_accumulator = 0.0;
        self.drop_triggered = false;
        self.hit_cooldown = [0.0; PLAYER_CAPACITY];
        self.freeze_elapsed = 0.0;
        self.freeze_move_time = 0.0;
        for (i, player) in self.players.iter_mut().enumerate() {
            *player = SupernovaPlayer {
                evaluated: evaluation_enabled[i],
                ..SupernovaPlayer::default()
            };
        }
    }

    /// Advance the game by `dt` seconds.
    ///
    /// `active` is a bitmask of currently-held actions per player (used for
    /// freeze/stillness detection). `triggered` is the edge-triggered bitmask
    /// (used for dance hits).
    pub fn update(&mut self, dt: f32, active: [u32; PLAYER_CAPACITY], triggered: [u32; PLAYER_CAPACITY]) {
        // Clear per-frame feedback.
        self.feedback = [SupernovaFeedback::default(); PLAYER_CAPACITY];
        self.drop_triggered = false;
        self.perfect_freeze = false;
        self.freeze_wobble = false;

        match self.phase {
            SupernovaPhase::Ready => {}
            SupernovaPhase::Countdown => self.update_countdown(dt),
            SupernovaPhase::Dance => self.update_dance(dt, triggered),
            SupernovaPhase::Freeze => self.update_freeze(dt, active),
            SupernovaPhase::Drop => self.update_drop(dt),
            SupernovaPhase::Result => {}
        }
    }

    fn update_countdown(&mut self, dt: f32) {
        self.countdown_remaining -= dt;
        if self.countdown_remaining <= 0.0 {
            self.begin_dance_round();
        }
    }

    fn begin_dance_round(&mut self) {
        self.phase = SupernovaPhase::Dance;
        self.phase_timer = SUPERNOVA_DANCE_SECONDS;
        self.round = self.round.saturating_add(1);
        self.freeze_progress = 0.0;
        self.freeze_elapsed = 0.0;
        self.freeze_move_time = 0.0;
    }

    fn begin_freeze_round(&mut self) {
        self.phase = SupernovaPhase::Freeze;
        self.phase_timer = SUPERNOVA_FREEZE_SECONDS;
        self.freeze_progress = 0.0;
        self.freeze_elapsed = 0.0;
        self.freeze_move_time = 0.0;
    }

    fn update_dance(&mut self, dt: f32, triggered: [u32; PLAYER_CAPACITY]) {
        // Advance beat clock.
        self.beat_accumulator += dt.clamp(0.0, 0.05);
        while self.beat_accumulator >= SUPERNOVA_BEAT_SECONDS {
            self.beat_accumulator -= SUPERNOVA_BEAT_SECONDS;
            self.beat_index = self.beat_index.wrapping_add(1);
        }
        self.beat_progress = self.beat_accumulator / SUPERNOVA_BEAT_SECONDS;

        // Decay cooldowns.
        for cooldown in self.hit_cooldown.iter_mut() {
            *cooldown = (*cooldown - dt).max(0.0);
        }

        // Count active players for co-op bonus.
        let active_players = self.players.iter().filter(|p| p.evaluated).count().max(1);
        let coop_multiplier = if active_players >= 2 {
            COOP_ENERGY_BONUS
        } else {
            1.0
        };

        let gameplay_mask = Action::GAMEPLAY_MASK;
        for (i, &mask) in triggered.iter().enumerate() {
            if !self.players[i].evaluated {
                continue;
            }
            let action_mask = mask & gameplay_mask;
            if action_mask == 0 || self.hit_cooldown[i] > 0.0 {
                continue;
            }

            // Register hit.
            self.hit_cooldown[i] = 0.18; // ~180ms cooldown between hits.
            self.players[i].hits += 1;

            // Check if on-beat.
            let on_beat = self.is_on_beat();
            self.feedback[i] = SupernovaFeedback { on_beat, hit: true };

            if on_beat {
                self.players[i].on_beat_hits += 1;
                self.players[i].combo += 1;
                self.players[i].best_combo = self.players[i].best_combo.max(self.players[i].combo);
                self.combo += 1;
                self.best_combo = self.best_combo.max(self.combo);
            } else {
                self.players[i].combo = 0;
                self.combo = 0;
            }

            // Calculate energy gain.
            let combo_mult =
                (1.0 + self.combo as f32 * COMBO_MULTIPLIER_STEP).min(MAX_COMBO_MULTIPLIER);
            let base = if on_beat {
                BASE_ENERGY_PER_HIT
            } else {
                OFF_BEAT_ENERGY
            };
            let gain = base * combo_mult * coop_multiplier;
            self.energy = (self.energy + gain).min(1.0);
            self.players[i].energy_contributed += gain;
        }

        // Full energy → supernova.
        if self.energy >= 1.0 {
            self.trigger_drop(SupernovaOutcome::FullSupernova);
            return;
        }

        // Dance round over → FREEZE!
        self.phase_timer -= dt;
        if self.phase_timer <= 0.0 {
            self.begin_freeze_round();
            return;
        }

        // Session safety cap.
        self.session_remaining -= dt;
        if self.session_remaining <= 0.0 {
            self.trigger_drop(SupernovaOutcome::TimeExpired);
        }
    }

    fn update_freeze(&mut self, dt: f32, active: [u32; PLAYER_CAPACITY]) {
        self.freeze_elapsed += dt;
        self.phase_timer -= dt;
        self.freeze_progress =
            (self.freeze_elapsed / SUPERNOVA_FREEZE_SECONDS).clamp(0.0, 1.0);

        // Detect movement after the reaction-time grace period.
        let gameplay_mask = Action::GAMEPLAY_MASK;
        let anyone_moving = self
            .players
            .iter()
            .zip(active.iter())
            .any(|(player, &mask)| player.evaluated && (mask & gameplay_mask) != 0);

        if self.freeze_elapsed > FREEZE_GRACE_SECONDS && anyone_moving {
            self.freeze_move_time += dt;
            self.freeze_wobble = true;
        }

        // Freeze round over → evaluate and return to dance.
        if self.phase_timer <= 0.0 {
            let perfect = self.freeze_move_time <= FREEZE_MOVE_BUDGET;
            if perfect {
                self.freeze_stars = self.freeze_stars.saturating_add(1);
                self.energy = (self.energy + PERFECT_FREEZE_ENERGY).min(1.0);
                self.perfect_freeze = true;
            }
            if self.energy >= 1.0 {
                self.trigger_drop(SupernovaOutcome::FullSupernova);
            } else {
                self.begin_dance_round();
            }
            return;
        }

        // Session safety cap.
        self.session_remaining -= dt;
        if self.session_remaining <= 0.0 {
            self.trigger_drop(SupernovaOutcome::TimeExpired);
        }
    }

    fn update_drop(&mut self, dt: f32) {
        self.phase_timer -= dt;
        if self.phase_timer <= 0.0 {
            self.phase = SupernovaPhase::Result;
        }
    }

    fn trigger_drop(&mut self, outcome: SupernovaOutcome) {
        self.outcome = Some(outcome);
        self.drop_triggered = true;

        // Calculate final score: energy + freeze stars + combo.
        let energy_score = (self.energy * 1000.0) as u32;
        let star_bonus = u32::from(self.freeze_stars) * 150;
        let combo_bonus = (self.best_combo as f32 * 40.0) as u32;
        let full_bonus = if outcome == SupernovaOutcome::FullSupernova {
            500
        } else {
            0
        };
        self.score = energy_score + star_bonus + combo_bonus + full_bonus;

        // Check for new best.
        self.new_best = self.score > self.best_score;
        if self.new_best {
            self.best_score = self.score;
        }

        // Enter drop animation phase.
        self.phase = SupernovaPhase::Drop;
        self.phase_timer = SUPERNOVA_DROP_SECONDS;
    }

    /// Check if current beat position is within the on-beat window.
    fn is_on_beat(&self) -> bool {
        self.beat_progress <= ON_BEAT_WINDOW || self.beat_progress >= (1.0 - ON_BEAT_WINDOW)
    }

    /// Request replay (from Result phase).
    pub fn request_replay(&mut self) -> bool {
        if self.phase != SupernovaPhase::Result {
            return false;
        }
        let eval: Vec<bool> = self.players.iter().map(|p| p.evaluated).collect();
        let mut arr = [false; PLAYER_CAPACITY];
        for (i, &e) in eval.iter().enumerate().take(PLAYER_CAPACITY) {
            arr[i] = e;
        }
        self.start_run(arr);
        true
    }

    /// Number of evaluated (active) players.
    pub fn active_player_count(&self) -> usize {
        self.players.iter().filter(|p| p.evaluated).count()
    }

    /// Energy as a percentage (0–100) for display.
    pub fn energy_percent(&self) -> u8 {
        (self.energy * 100.0).round().clamp(0.0, 100.0) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn eval_one() -> [bool; PLAYER_CAPACITY] {
        [true, false, false, false]
    }

    fn eval_two() -> [bool; PLAYER_CAPACITY] {
        [true, true, false, false]
    }

    const NO_ACTIVE: [u32; PLAYER_CAPACITY] = [0; PLAYER_CAPACITY];

    #[test]
    fn new_game_starts_in_ready() {
        let game = SupernovaGame::new();
        assert_eq!(game.phase, SupernovaPhase::Ready);
        assert_eq!(game.energy, 0.0);
        assert_eq!(game.score, 0);
        assert_eq!(game.freeze_stars, 0);
    }

    #[test]
    fn start_run_enters_countdown() {
        let mut game = SupernovaGame::new();
        game.start_run(eval_one());
        assert_eq!(game.phase, SupernovaPhase::Countdown);
        assert_eq!(game.countdown_remaining, SUPERNOVA_COUNTDOWN_SECONDS);
        assert!(game.players[0].evaluated);
        assert!(!game.players[1].evaluated);
    }

    #[test]
    fn countdown_transitions_to_dance() {
        let mut game = SupernovaGame::new();
        game.start_run(eval_one());
        game.update(2.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        assert_eq!(game.phase, SupernovaPhase::Dance);
        assert_eq!(game.round, 1);
    }

    #[test]
    fn on_beat_action_adds_energy() {
        let mut game = SupernovaGame::new();
        game.start_run(eval_one());
        game.update(2.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        assert_eq!(game.phase, SupernovaPhase::Dance);

        game.beat_progress = 0.0;
        game.beat_accumulator = 0.0;
        game.update(0.001, NO_ACTIVE, [Action::Jump.mask(), 0, 0, 0]);
        assert!(game.energy > 0.0);
        assert!(game.feedback[0].on_beat);
        assert!(game.feedback[0].hit);
        assert_eq!(game.combo, 1);
    }

    #[test]
    fn off_beat_action_adds_less_energy_and_resets_combo() {
        let mut game = SupernovaGame::new();
        game.start_run(eval_one());
        game.update(2.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);

        game.beat_accumulator = SUPERNOVA_BEAT_SECONDS * 0.5;
        game.beat_progress = 0.5;
        game.combo = 5;
        game.update(0.001, NO_ACTIVE, [Action::Squat.mask(), 0, 0, 0]);
        assert!(game.energy > 0.0);
        assert!(!game.feedback[0].on_beat);
        assert_eq!(game.combo, 0);
    }

    #[test]
    fn dance_round_transitions_to_freeze() {
        let mut game = SupernovaGame::new();
        game.start_run(eval_one());
        game.update(2.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        assert_eq!(game.phase, SupernovaPhase::Dance);

        game.update(SUPERNOVA_DANCE_SECONDS + 0.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        assert_eq!(game.phase, SupernovaPhase::Freeze);
    }

    #[test]
    fn perfect_freeze_awards_star_and_energy() {
        let mut game = SupernovaGame::new();
        game.start_run(eval_one());
        game.update(2.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        game.update(SUPERNOVA_DANCE_SECONDS + 0.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        assert_eq!(game.phase, SupernovaPhase::Freeze);

        // Hold perfectly still for the whole freeze.
        let energy_before = game.energy;
        game.update(SUPERNOVA_FREEZE_SECONDS + 0.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        assert_eq!(game.freeze_stars, 1);
        assert!(game.energy > energy_before);
        assert_eq!(game.phase, SupernovaPhase::Dance); // back to dancing
        assert_eq!(game.round, 2);
    }

    #[test]
    fn moving_during_freeze_loses_star() {
        let mut game = SupernovaGame::new();
        game.start_run(eval_one());
        game.update(2.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        game.update(SUPERNOVA_DANCE_SECONDS + 0.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        assert_eq!(game.phase, SupernovaPhase::Freeze);

        // Move (hold Jump active) for longer than the move budget.
        let moving_active = [Action::Jump.mask(), 0, 0, 0];
        game.update(0.5, moving_active, [0; PLAYER_CAPACITY]);
        assert!(game.freeze_wobble);
        game.update(SUPERNOVA_FREEZE_SECONDS, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        assert_eq!(game.freeze_stars, 0); // no star
    }

    #[test]
    fn brief_jitter_during_freeze_is_forgiven() {
        let mut game = SupernovaGame::new();
        game.start_run(eval_one());
        game.update(2.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        game.update(SUPERNOVA_DANCE_SECONDS + 0.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        assert_eq!(game.phase, SupernovaPhase::Freeze);

        // A single brief jitter frame (within move budget) should be forgiven.
        let moving_active = [Action::Jump.mask(), 0, 0, 0];
        game.update(0.5, NO_ACTIVE, [0; PLAYER_CAPACITY]); // grace + still
        game.update(0.1, moving_active, [0; PLAYER_CAPACITY]); // brief jitter
        game.update(SUPERNOVA_FREEZE_SECONDS, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        assert_eq!(game.freeze_stars, 1); // still perfect
    }

    #[test]
    fn full_energy_triggers_supernova() {
        let mut game = SupernovaGame::new();
        game.start_run(eval_one());
        game.update(2.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);

        game.energy = 0.99;
        game.beat_progress = 0.0;
        game.beat_accumulator = 0.0;
        game.update(0.001, NO_ACTIVE, [Action::Jump.mask(), 0, 0, 0]);
        assert_eq!(game.phase, SupernovaPhase::Drop);
        assert_eq!(game.outcome, Some(SupernovaOutcome::FullSupernova));
        assert!(game.drop_triggered);
        assert!(game.score > 0);
    }

    #[test]
    fn drop_transitions_to_result() {
        let mut game = SupernovaGame::new();
        game.start_run(eval_one());
        game.update(2.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        game.energy = 1.0;
        game.beat_progress = 0.0;
        game.beat_accumulator = 0.0;
        game.update(0.001, NO_ACTIVE, [Action::Jump.mask(), 0, 0, 0]);
        assert_eq!(game.phase, SupernovaPhase::Drop);

        game.update(SUPERNOVA_DROP_SECONDS + 0.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        assert_eq!(game.phase, SupernovaPhase::Result);
    }

    #[test]
    fn replay_from_result_restarts() {
        let mut game = SupernovaGame::new();
        game.start_run(eval_one());
        game.update(2.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        game.energy = 1.0;
        game.beat_progress = 0.0;
        game.beat_accumulator = 0.0;
        game.update(0.001, NO_ACTIVE, [Action::Jump.mask(), 0, 0, 0]);
        game.update(SUPERNOVA_DROP_SECONDS + 0.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        assert_eq!(game.phase, SupernovaPhase::Result);

        let best = game.best_score;
        assert!(game.request_replay());
        assert_eq!(game.phase, SupernovaPhase::Countdown);
        assert_eq!(game.best_score, best);
        assert_eq!(game.energy, 0.0);
        assert_eq!(game.freeze_stars, 0);
    }

    #[test]
    fn coop_two_players_pump_faster() {
        let mut solo = SupernovaGame::new();
        solo.start_run(eval_one());
        solo.update(2.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        solo.beat_progress = 0.0;
        solo.beat_accumulator = 0.0;
        solo.update(0.001, NO_ACTIVE, [Action::Jump.mask(), 0, 0, 0]);
        let solo_energy = solo.energy;

        let mut coop = SupernovaGame::new();
        coop.start_run(eval_two());
        coop.update(2.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        coop.beat_progress = 0.0;
        coop.beat_accumulator = 0.0;
        coop.update(0.001, NO_ACTIVE, [Action::Jump.mask(), Action::Jump.mask(), 0, 0]);
        let coop_energy = coop.energy;

        assert!(coop_energy > solo_energy * 1.4);
    }

    #[test]
    fn new_best_score_tracked() {
        let mut game = SupernovaGame::new();
        game.best_score = 100;
        game.start_run(eval_one());
        game.update(2.1, NO_ACTIVE, [0; PLAYER_CAPACITY]);
        game.energy = 1.0;
        game.beat_progress = 0.0;
        game.beat_accumulator = 0.0;
        game.update(0.001, NO_ACTIVE, [Action::Jump.mask(), 0, 0, 0]);
        assert!(game.score > 100);
        assert!(game.new_best);
        assert_eq!(game.best_score, game.score);
    }
}
