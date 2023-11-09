use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum GameState {
    /// In the main menu.
    #[default]
    MainMenu,

    /// In a loading screen.
    Loading,

    /// Running. See [NetworkState] for more.
    Simulating,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum NetworkState {
    /// Not simulating anything.
    #[default]
    Unconnected,

    /// Playing by yourself.
    Singleplayer,

    /// Connected to a remote lobby as a client.
    Multiplayer,

    /// Hosting a lobby and also playing in it.
    HostingLobby,
}