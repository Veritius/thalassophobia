//! A drowning simulator.

#![warn(missing_docs)]

mod globals;
mod state;
mod settings;
#[cfg(feature="dev")]
mod devmenu;
mod mainmenu;
mod structure;
mod movement;

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

    // Add subsystems
    state::setup_game_state(&mut app);
    settings::setup_settings(&mut app);
    #[cfg(feature="dev")]
    devmenu::setup_dev_menu(&mut app);
    mainmenu::setup_main_menu(&mut app);
    structure::setup_structures(&mut app);

    app.run();
}