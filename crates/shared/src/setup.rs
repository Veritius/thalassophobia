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
        app.add_plugins(crate::multiplayer::StardustPlugin);
        app.add_plugins(crate::multiplayer::udp::UdpTransportPlugin::balanced(crate::multiplayer::UDP_APPLICATION_VERSION));
    }

    // Subsystem plugins
    app.add_plugins(crate::disabling::DisablingPlugin);
    app.add_plugins(crate::player::PlayerCharacterPlugin);
    app.add_plugins(crate::state::GameStatePlugin);
}