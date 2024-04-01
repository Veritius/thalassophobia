mod ui;

use shared::bevy::prelude::*;
use crate::state::ClientState;

pub(crate) struct InitialLoadingPlugin;

impl Plugin for InitialLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(ClientState::Initial), ui::spawn_loading_screen);
        app.add_systems(OnExit(ClientState::Initial), ui::despawn_loading_screen);
    }
}