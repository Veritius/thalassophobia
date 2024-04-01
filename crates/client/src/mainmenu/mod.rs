mod build;

use shared::bevy::prelude::*;
use shared::bevy_ecs;
use shared::state::*;
use self::build::build_main_menu_system;

pub(crate) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Neutral), build_main_menu_system);
    }
}

#[derive(Component)]
struct MainMenuRoot;