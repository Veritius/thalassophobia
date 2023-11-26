//! A drowning simulator.

#![warn(missing_docs)]

#[cfg(feature="dev")]
mod devmenu;

mod compartments;
mod gamestate;
mod globals;
mod guns;
mod health;
mod items;
mod mainmenu;
mod movement;
mod settings;
mod structure;

use bevy::prelude::*;
use bevy_rapier3d::prelude::RapierPhysicsPlugin;
use bevy_stardust::prelude::*;
use bevy_egui::EguiPlugin;

fn main() {
    let mut app = App::new();

    // External plugins essential for game functionality
    app.add_plugins((
        DefaultPlugins,
        RapierPhysicsPlugin::<()>::default(),
        StardustPlugin,
    ));

    // Transport layers for Stardust
    app.add_plugins(UdpTransportPlugin);

    // UI (will be removed in future when bevy_ui is better)
    app.add_plugins(EguiPlugin);

    // Development subsystems
    #[cfg(feature="dev")] {
        devmenu::setup_dev_menu(&mut app);
    }

    // Add subsystems
    compartments::setup_compartments(&mut app);
    gamestate::setup_game_state(&mut app);
    guns::setup_guns(&mut app);
    health::setup_health(&mut app);
    items::setup_items(&mut app);
    mainmenu::setup_main_menu(&mut app);
    movement::setup_movement(&mut app);
    settings::setup_settings(&mut app);
    structure::setup_structures(&mut app);

    app.run();
}