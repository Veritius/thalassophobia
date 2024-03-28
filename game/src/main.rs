//! A drowning simulator.

#![warn(missing_docs)]

#[cfg(feature="dev")]
mod devmenu;

mod ui;

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
    app.add_plugins(ui::UiExtensionsPlugin);

    // Development subsystems
    #[cfg(feature="dev")]
    app.add_plugins(devmenu::DevMenuPlugin);

    // Add subsystems
    app.add_plugins(gamestate::GameStatePlugin);
    app.add_plugins(mainmenu::MainMenuPlugin);

    app.run();
}