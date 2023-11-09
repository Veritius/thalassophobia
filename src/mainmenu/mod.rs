mod frontpage;
mod settings;

use bevy::prelude::*;
use crate::state::GameState;

use self::{frontpage::front_page_system, settings::settings_menu_system};

#[derive(Debug, Default, Resource, PartialEq, Eq, Reflect)]
enum MainMenuPage {
    #[default]
    FrontPage,
    Settings,
}

pub(super) fn setup_main_menu(app: &mut App) {
    app.register_type::<MainMenuPage>();

    app.add_systems(OnEnter(GameState::MainMenu), |mut commands: Commands| { commands.init_resource::<MainMenuPage>() });
    app.add_systems(OnExit(GameState::MainMenu), |mut commands: Commands| { commands.remove_resource::<MainMenuPage>() });

    app.add_systems(Update, front_page_system
        .run_if(resource_exists_and_equals(MainMenuPage::FrontPage)));
    app.add_systems(Update, settings_menu_system
        .run_if(resource_exists_and_equals(MainMenuPage::Settings)));
}