mod frontpage;
mod joingame;
mod lobby;

use bevy::prelude::*;
use crate::gamestate::AppState;

use self::{
    frontpage::front_page_system,
    joingame::join_game_menu_system,
    lobby::lobby_menu_system,
};

pub(crate) struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MainMenuPage>();

        app.add_systems(OnEnter(AppState::MainMenu), |mut commands: Commands| { commands.init_resource::<MainMenuPage>() });
        app.add_systems(OnExit(AppState::MainMenu), |mut commands: Commands| { commands.remove_resource::<MainMenuPage>() });
    
        // Menu systems
        app.add_systems(Update, front_page_system
            .run_if(in_state(AppState::MainMenu))
            .run_if(resource_exists_and_equals(MainMenuPage::FrontPage)));
        app.add_systems(Update, join_game_menu_system
            .run_if(in_state(AppState::MainMenu))
            .run_if(resource_exists_and_equals(MainMenuPage::JoinGame)));
    
        // Lobby UI
        app.add_systems(Update, lobby_menu_system
            .run_if(in_state(AppState::Lobby)));
    }
}

#[derive(Debug, Default, Resource, PartialEq, Eq, Reflect)]
#[reflect(Resource)]
enum MainMenuPage {
    #[default]
    FrontPage,
    JoinGame,
}