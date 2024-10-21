mod lobby;

use shared::prelude::*;

pub struct LobbyHostingPlugin;

impl Plugin for LobbyHostingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<lobby::LobbyState>();
        app.add_sub_state::<lobby::LobbyState>();
        app.register_type::<lobby::LobbySettings>();
        app.add_sub_state::<lobby::LobbySettings>();
    }
}