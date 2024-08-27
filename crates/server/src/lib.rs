mod settings;

use shared::prelude::*;

pub use settings::LobbySettings;

pub struct LobbyHostingPlugin;

impl Plugin for LobbyHostingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ServerState>();
        app.add_sub_state::<ServerState>();

        app.register_type::<LobbySettings>();
    }
}

/// Whether or not the lobby is open.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, SubStates)]
#[source(Initialisation = Initialisation::Finished)]
pub enum ServerState {
    /// No lobby is being hosted.
    #[default]
    Offline,

    /// A lobby is being hosted.
    Active,
}