use bevy::prelude::*;

pub fn setup(app: &mut App) {
    // Add Bevy's core functionality
    app.add_plugins(DefaultPlugins);

    // Physics engine
    app.add_plugins(crate::rapier::plugin::RapierPhysicsPlugin::<()>::default());
    #[cfg(feature="phys_debug")]
    app.add_plugins(crate::rapier::render::RapierDebugRenderPlugin::default());

    // Multiplayer functionality
    #[cfg(feature="multiplayer")] {
        app.add_plugins(crate::stardust::plugin::StardustPlugin);
    }

    // Subsystem plugins
    app.add_plugins(crate::movement::MovementInputsPlugin);
    app.add_plugins(crate::state::GameStatePlugin);
}