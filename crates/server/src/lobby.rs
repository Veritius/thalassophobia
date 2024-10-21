use shared::prelude::*;

/// Whether or not the lobby is open.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, SubStates)]
#[source(Initialisation = Initialisation::Finished)]
pub enum LobbyState {
    /// No lobby is being hosted.
    #[default]
    Offline,

    /// A lobby is being hosted.
    Active,
}

/// Settings for the lobby, if one is being hosted.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect, SubStates)]
#[source(LobbyState = LobbyState::Active)]
pub struct LobbySettings {
    /// Whether or not new connections should be accepted.
    pub listening: bool,

    /// The limit for the number of active connections.
    pub max_players: u32,
}

impl Default for LobbySettings {
    fn default() -> Self {
        Self {
            listening: true,
            max_players: 16,
        }
    }
}