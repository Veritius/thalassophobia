pub mod joining;

use shared::prelude::*;

pub(crate) struct MultiplayerPlugin;

impl Plugin for MultiplayerPlugin {
    fn build(&self, app: &mut App) {
        // Server setup, if hosting is enabled
        #[cfg(feature="hosting")]
        app.add_plugins(server::lobbies::LobbyHostingPlugin);
    }
}