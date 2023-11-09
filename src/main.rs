//! A drowning simulator.

#![warn(missing_docs)]

mod globals;
mod state;
mod settings;
mod devmenu;
mod mainmenu;
mod structure;

use bevy::prelude::*;
use bevy_stardust::prelude::*;
use bevy_egui::EguiPlugin;

fn main() {
    let mut app = App::new();

    // Bevy plugins
    app.add_plugins(DefaultPlugins);

    // UI
    app.add_plugins(EguiPlugin);

    // Multiplayer
    app.add_plugins(StardustPlugin);
    app.add_plugins(UdpTransportPlugin);

    // Add subsystems
    state::setup_game_state(&mut app);
    settings::setup_settings(&mut app);
    devmenu::setup_dev_menu(&mut app);
    mainmenu::setup_main_menu(&mut app);
    structure::setup_structures(&mut app);

    app.run();
}