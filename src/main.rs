//! A drowning simulator.

#![warn(missing_docs)]

mod globals;
mod state;
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
    mainmenu::setup_main_menu(&mut app);

    // Multiplayer
    app.add_plugins(StardustPlugin);
    app.add_plugins(UdpTransportPlugin);

    // Add subsystems
    state::setup_game_state(&mut app);
    structure::setup_structures(&mut app);

    app.run();
}