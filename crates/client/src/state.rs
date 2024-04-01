use shared::bevy::prelude::*;
use shared::bevy_ecs;

pub(crate) struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ClientState>();
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum ClientState {
    /// Loading state before the main menu.
    #[default]
    Initial,

    /// In the main menu.
    MainMenu,
}