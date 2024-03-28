//! A drowning simulator.

#![warn(missing_docs)]

#[cfg(feature="dev")]
mod devmenu;

mod gamestate;
mod mainmenu;

#[cfg(feature="multiplayer")]
mod multiplayer;

use bevy::prelude::*;
use bevy_rapier3d::prelude::RapierPhysicsPlugin;

fn main() {
    let mut app = App::new();

    // External plugins essential for game functionality
    app.add_plugins((
        DefaultPlugins,
        RapierPhysicsPlugin::<()>::default(),
    ));

    // Set up multiplayer
    #[cfg(feature="multiplayer")]
    app.add_plugins(multiplayer::MultiplayerPlugin);

    // UI (will be removed in future when bevy_ui is better)
    app.add_plugins(bevy_egui::EguiPlugin);

    // Development subsystems
    #[cfg(feature="dev")]
    app.add_plugins(devmenu::DevMenuPlugin);

    // Add subsystems
    gamestate::setup_game_state(&mut app);
    mainmenu::setup_main_menu(&mut app);

    app.run();
}