//! A drowning simulator.

#![warn(missing_docs)]

mod globals;
mod state;
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

    // Game state
    app.add_state::<state::GameState>();

    app.run();
}