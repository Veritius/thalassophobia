//! A drowning simulator.

#![warn(missing_docs)]

mod globals;
mod state;
mod settings;
mod mainmenu;
mod structure;

use bevy::prelude::*;
use bevy_stardust::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    // Bevy plugins
    app.add_plugins(DefaultPlugins);

    // UI
    app.add_plugins(EguiPlugin);
    app.add_plugins(WorldInspectorPlugin::new());

    // Multiplayer
    app.add_plugins(StardustPlugin);
    app.add_plugins(UdpTransportPlugin);

    // Add subsystems
    state::setup_game_state(&mut app);
    settings::setup_settings(&mut app);
    mainmenu::setup_main_menu(&mut app);
    structure::setup_structures(&mut app);

    app.run();
}