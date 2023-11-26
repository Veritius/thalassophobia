pub mod simulation;

use bevy::prelude::*;
use simulation::*;

pub(super) fn setup_game_state(app: &mut App) {
    app.register_type::<AppState>();
    app.register_type::<SimulationState>();

    app.add_state::<AppState>();
    app.add_state::<SimulationState>();
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, States)]
pub enum AppState {
    Loading,

    #[default]
    MainMenu,

    InGame,
}