//! A drowning simulator.

#![warn(missing_docs)]

#[cfg(feature="dev")]
mod devmenu;

mod gamestate;
mod mainmenu;
mod multiplayer;

use bevy::prelude::*;
use bevy_rapier3d::prelude::RapierPhysicsPlugin;
use bevy_egui::EguiPlugin;

fn main() {
    let mut app = App::new();

    // External plugins essential for game functionality
    app.add_plugins((
        DefaultPlugins,
        RapierPhysicsPlugin::<()>::default(),
    ));

    // Set up multiplayer
    multiplayer::setup_multiplayer(&mut app);

    // UI (will be removed in future when bevy_ui is better)
    app.add_plugins(EguiPlugin);

    // Development subsystems
    #[cfg(feature="dev")] {
        devmenu::setup_dev_menu(&mut app);
    }

    // Add subsystems
    gamestate::setup_game_state(&mut app);
    mainmenu::setup_main_menu(&mut app);

    app.run();
}