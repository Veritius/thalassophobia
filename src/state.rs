use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, States)]
pub enum GameState {
    /// In the main menu.
    #[default]
    MainMenu,

    /// In a loading screen.
    Loading,

    /// Running.
    Simulating {
        state: SimulationState,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Hash)]
pub enum SimulationState {
    /// Playing by yourself.
    Singleplayer,

    /// Connected to a remote lobby as a client.
    Multiplayer,

    /// Hosting a lobby and also playing in it.
    HostingLobby,
}