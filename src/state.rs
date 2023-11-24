use bevy::prelude::*;

pub(super) fn setup_game_state(app: &mut App) {
    app.register_type::<SimulationState>();
    app.register_type::<MultiplayerState>();

    app.add_state::<SimulationState>();
    app.add_state::<MultiplayerState>();
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, States)]
#[reflect(PartialEq)]
pub enum SimulationState {
    /// In the main menu.
    #[default]
    MainMenu,

    /// In a loading screen.
    Loading,

    /// Running.
    Simulating,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, States)]
#[reflect(PartialEq)]
pub enum MultiplayerState {
    #[default]
    Inactive,
    Standby,
    Hosting,
    Client,
}