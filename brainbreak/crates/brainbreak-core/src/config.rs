//! Data-driven game configuration schema (SCRAPER framework).
//!
//! Each field maps to a SCRAPER dimension:
//! - **S**core → `scoring`
//! - **C**hallenge → `challenge`
//! - **R**eward → (renderer reads theme + combo)
//! - **A**esthetics → `theme`
//! - **P**acing → `pacing`
//! - **E**ase → `actions` (max 3)
//! - **R**eplay → `end` + best_score tracking

use serde::Deserialize;

/// Maximum number of action slots per game (simplicity constraint).
pub const MAX_ACTIONS: usize = 3;

/// Top-level game configuration — everything needed to run a custom game.
#[derive(Clone, Debug, Deserialize)]
pub struct GameConfig {
    /// Schema version for forward compatibility (currently 1).
    pub schema_version: u8,
    /// Unique identifier (slug).
    pub id: String,
    /// Display title.
    pub title: String,
    /// Active action bitmasks (max 3; 0 = empty slot).
    /// Bit values: 1=Left, 2=Right, 4=Jump, 8=Squat, 16=L-Up, 32=R-Up, 64=Clap.
    pub actions: [u32; MAX_ACTIONS],
    /// Core mechanic atom.
    pub mechanic: Mechanic,
    /// Scoring parameters (S).
    pub scoring: ScoringConfig,
    /// Pacing / intensity curve (P).
    pub pacing: PacingConfig,
    /// Challenge escalation (C).
    pub challenge: ChallengeConfig,
    /// End / replayability condition (R).
    pub end: EndConfig,
    /// Visual theme (A).
    pub theme: ThemeConfig,
}

impl GameConfig {
    /// Parse from JSON bytes. Returns None on invalid schema.
    pub fn from_json(bytes: &[u8]) -> Option<Self> {
        let config: Self = serde_json::from_slice(bytes).ok()?;
        if config.schema_version != 1 {
            return None;
        }
        if config.title.is_empty() || config.id.is_empty() {
            return None;
        }
        Some(config)
    }

    /// Returns only the non-zero action masks.
    pub fn active_actions(&self) -> impl Iterator<Item = u32> + '_ {
        self.actions.iter().copied().filter(|&a| a != 0)
    }

    /// True if the given triggered mask contains any of this game's actions.
    pub fn matches_action(&self, triggered_mask: u32) -> bool {
        let allowed = self.actions.iter().fold(0, |acc, &a| acc | a);
        triggered_mask & allowed != 0
    }
}

/// Core mechanic atom — determines the primary gameplay verb.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mechanic {
    /// Dodge incoming obstacles (lane switch / jump / squat).
    #[default]
    Dodge,
    /// Pump energy by performing actions on-beat.
    Pump,
    /// Catch falling/flying targets by moving into them.
    Catch,
    /// Hold a pose for a required duration.
    Hold,
    /// Mimic a shown action sequence.
    Pattern,
}

/// Scoring parameters (SCRAPER: Score).
#[derive(Clone, Debug, Deserialize)]
pub struct ScoringConfig {
    /// Score added per combo level (multiplier step).
    #[serde(default = "default_combo_step")]
    pub combo_step: f32,
    /// Bonus multiplier for on-beat actions.
    #[serde(default = "default_on_beat_bonus")]
    pub on_beat_bonus: f32,
    /// Multiplier when 2+ players contribute.
    #[serde(default = "default_coop_bonus")]
    pub coop_bonus: f32,
}

impl Default for ScoringConfig {
    fn default() -> Self {
        Self {
            combo_step: default_combo_step(),
            on_beat_bonus: default_on_beat_bonus(),
            coop_bonus: default_coop_bonus(),
        }
    }
}

/// Pacing / intensity curve (SCRAPER: Pacing).
#[derive(Clone, Debug, Deserialize)]
pub struct PacingConfig {
    /// Track BPM (60–180).
    #[serde(default = "default_bpm")]
    pub bpm: f32,
    /// Session duration in seconds (30–120).
    #[serde(default = "default_session_seconds")]
    pub session_seconds: f32,
    /// Intensity curve shape.
    #[serde(default)]
    pub curve: IntensityCurve,
}

impl Default for PacingConfig {
    fn default() -> Self {
        Self {
            bpm: default_bpm(),
            session_seconds: default_session_seconds(),
            curve: IntensityCurve::default(),
        }
    }
}

/// Intensity curve shape over the session duration.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IntensityCurve {
    /// Rise → Peak → Release (breath arc).
    #[default]
    BreathArc,
    /// Linear ramp from 0 to 1.
    SteadyRamp,
    /// Sinusoidal waves.
    Waves,
}

impl IntensityCurve {
    /// Returns intensity in [0, 1] for progress t in [0, 1].
    pub fn intensity(self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::BreathArc => {
                // Rise (0→0.6), Peak (0.6→0.8), Release (0.8→1.0)
                if t < 0.6 {
                    t / 0.6
                } else if t < 0.8 {
                    1.0
                } else {
                    1.0 - (t - 0.8) / 0.2 * 0.4
                }
            }
            Self::SteadyRamp => t,
            Self::Waves => 0.5 + 0.5 * (t * std::f32::consts::TAU * 2.0).sin(),
        }
    }
}

/// Challenge escalation parameters (SCRAPER: Challenge).
#[derive(Clone, Debug, Deserialize)]
pub struct ChallengeConfig {
    /// Base spawn interval in seconds (lower = harder).
    #[serde(default = "default_spawn_interval")]
    pub spawn_interval: f32,
    /// Speed multiplier ramp over session (1.0 = no ramp).
    #[serde(default = "default_speed_ramp")]
    pub speed_ramp: f32,
    /// Base difficulty 0.0–1.0 (affects density/speed).
    #[serde(default = "default_difficulty")]
    pub difficulty: f32,
}

impl Default for ChallengeConfig {
    fn default() -> Self {
        Self {
            spawn_interval: default_spawn_interval(),
            speed_ramp: default_speed_ramp(),
            difficulty: default_difficulty(),
        }
    }
}

/// End condition (SCRAPER: Replayability).
#[derive(Clone, Debug, Deserialize)]
pub struct EndConfig {
    /// How the game ends.
    #[serde(default)]
    pub mode: EndMode,
    /// Lives (used when mode = LifeBased).
    #[serde(default = "default_lives")]
    pub lives: u8,
    /// Goal score (used when mode = GoalBased).
    #[serde(default = "default_goal_score")]
    pub goal_score: u32,
}

impl Default for EndConfig {
    fn default() -> Self {
        Self {
            mode: EndMode::default(),
            lives: default_lives(),
            goal_score: default_goal_score(),
        }
    }
}

/// How a game session ends.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EndMode {
    /// Session ends when timer expires.
    #[default]
    TimeBased,
    /// Session ends when lives reach 0.
    LifeBased,
    /// Session ends when goal score is reached.
    GoalBased,
}

/// Visual theme (SCRAPER: Aesthetics).
#[derive(Clone, Debug, Deserialize)]
pub struct ThemeConfig {
    /// Primary color as 0xRRGGBB.
    #[serde(default = "default_primary")]
    pub primary: u32,
    /// Secondary/accent color as 0xRRGGBB.
    #[serde(default = "default_secondary")]
    pub secondary: u32,
    /// Visual style preset.
    #[serde(default)]
    pub style: ThemeStyle,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            primary: default_primary(),
            secondary: default_secondary(),
            style: ThemeStyle::default(),
        }
    }
}

/// Visual style preset.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ThemeStyle {
    /// Bright neon on dark background.
    #[default]
    Neon,
    /// Deep space gradients + stars.
    Cosmic,
    /// Clean monochrome.
    Minimal,
}

// --- Serde defaults ---
fn default_combo_step() -> f32 { 0.15 }
fn default_on_beat_bonus() -> f32 { 1.5 }
fn default_coop_bonus() -> f32 { 1.5 }
fn default_bpm() -> f32 { 120.0 }
fn default_session_seconds() -> f32 { 60.0 }
fn default_spawn_interval() -> f32 { 1.2 }
fn default_speed_ramp() -> f32 { 1.5 }
fn default_difficulty() -> f32 { 0.5 }
fn default_lives() -> u8 { 3 }
fn default_goal_score() -> u32 { 100 }
fn default_primary() -> u32 { 0x00E5FF }
fn default_secondary() -> u32 { 0xFF4081 }
