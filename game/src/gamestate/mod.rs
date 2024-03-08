pub mod simulation;

use bevy::prelude::*;
use simulation::*;

pub(super) fn setup_game_state(app: &mut App) {
    app.register_type::<AppState>();
    app.register_type::<SimulationState>();

    app.init_state::<AppState>();
    app.init_state::<SimulationState>();

    app.add_systems(OnEnter(SimulationState::Stopped), pause_simulation);
    app.add_systems(OnEnter(SimulationState::Running), unpause_simulation);
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, States)]
pub enum AppState {
    Loading,

    #[default]
    MainMenu,

    Lobby,

    InGame,
}