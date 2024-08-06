use std::fmt::Display;
use bevy::prelude::*;

pub(crate) struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Reflect, States)]
pub enum GameState {
    /// No game loaded.
    /// This is usually a 'main menu' state.
    #[default]
    Neutral,

    /// Game loading/generating.
    Loading,

    /// Simulation is loaded and running.
    Simulating,

    /// Simulation is loaded, but paused.
    Paused,

    /// Simulation is shutting down.
    Closing,
}

impl GameState {
    pub fn is_loaded(&self) -> bool {
        match self {
            GameState::Simulating => true,
            GameState::Paused => true,
            _ => false,
        }
    }

    pub fn is_unloaded(&self) -> bool {
        !self.is_loaded()
    }

    pub fn is_paused(&self) -> bool {
        *self == GameState::Paused
    }

    pub fn is_running(&self) -> bool {
        *self == GameState::Simulating
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GameState::Neutral => "Neutral",
            GameState::Loading => "Loading",
            GameState::Simulating => "Simulating",
            GameState::Paused => "Paused",
            GameState::Closing => "Closing",
        })
    }
}

pub fn simulation_loaded() -> impl Fn(Res<State<GameState>>) -> bool + Clone {
    |state| { state.is_loaded() }
}

pub fn simulation_unloaded() -> impl Fn(Res<State<GameState>>) -> bool + Clone {
    |state| { state.is_unloaded() }
}

pub fn simulation_paused() -> impl Fn(Res<State<GameState>>) -> bool + Clone {
    |state| { state.is_paused() }
}

pub fn simulation_running() -> impl Fn(Res<State<GameState>>) -> bool + Clone {
    |state| { state.is_running() }
}