use shared::prelude::*;

/// Settings for the lobby, if one is being hosted.
#[derive(Debug, Clone, Resource, Reflect)]
#[reflect(Resource)]
pub struct LobbySettings {
    /// The limit for the number of active connections.
    pub max_players: u32,
}

impl Default for LobbySettings {
    fn default() -> Self {
        Self {
            max_players: 16,
        }
    }
}