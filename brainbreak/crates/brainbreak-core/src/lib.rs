pub mod config;
pub mod config_game;
pub mod game_state;
pub mod pose;
pub mod runner;
pub mod runtime;
pub mod supernova;

pub use config::GameConfig;
pub use config_game::{ConfigGame, ConfigOutcome, ConfigPhase};

pub const PLAYER_CAPACITY: usize = 4;
pub const KEYPOINT_COUNT: usize = 17;

pub use game_state::{GameMode, GameState};
pub use pose::{Action, ActionSample, Keypoint, PoseFrame, PoseRecognizer, RecognizerConfig};
pub use runner::{
    RUNNER_COLLISION_DISTANCE, RUNNER_COUNTDOWN_SECONDS, RUNNER_OBSTACLE_CAPACITY,
    RUNNER_SESSION_SECONDS, RunnerFeedback, RunnerGame, RunnerHazard, RunnerObstacle,
    RunnerOutcome, RunnerPhase, RunnerPlayer, RunnerResultEvent,
};
pub use runtime::{MotionInputFrame, MotionRuntime, PlayerMotionState};
pub use supernova::{
    SUPERNOVA_COUNTDOWN_SECONDS, SUPERNOVA_DANCE_SECONDS, SUPERNOVA_DROP_SECONDS,
    SUPERNOVA_FREEZE_SECONDS, SUPERNOVA_SESSION_SECONDS, SupernovaFeedback, SupernovaGame,
    SupernovaOutcome, SupernovaPhase, SupernovaPlayer,
};

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
