mod frontpage;

use bevy::prelude::*;
use crate::state::GameState;

use self::frontpage::front_page_system;

#[derive(Debug, Default, Resource, PartialEq, Eq)]
enum MainMenuPage {
    #[default]
    FrontPage,
}

pub fn setup_main_menu(app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), |mut commands: Commands| { commands.init_resource::<MainMenuPage>() });
    app.add_systems(OnExit(GameState::MainMenu), |mut commands: Commands| { commands.remove_resource::<MainMenuPage>() });

    app.add_systems(Update, front_page_system
        .run_if(resource_exists_and_equals(MainMenuPage::FrontPage)));
}