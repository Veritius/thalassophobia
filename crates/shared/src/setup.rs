use bevy::prelude::*;

#[derive(Resource, Clone, Copy, PartialEq, Eq)]
pub enum SetupMode {
    Full,
    Headless,
}

pub fn pre_setup(app: &mut App) {
    // Add Bevy core plugins
    app.add_plugins(DefaultPlugins);

    // Physics engine
    app.add_plugins(crate::rapier::plugin::RapierPhysicsPlugin::<()>::default());
    #[cfg(feature="phys_debug")]
    app.add_plugins(crate::rapier::render::RapierDebugRenderPlugin::default());

    // Multiplayer functionality
    #[cfg(feature="multiplayer")] {
        app.add_plugins(crate::multiplayer::StardustPlugin);
    }

    // Subsystem plugins
    app.add_plugins(crate::campaign::CampaignPlugin);
    app.add_plugins(crate::controller::PlayerCharacterPlugin);
    app.add_plugins(crate::disabling::DisablingPlugin);
    app.add_plugins(crate::state::GameStatePlugin);
}

pub fn post_setup(app: &mut App) {
    // Multiplayer functionality
    #[cfg(feature="multiplayer")] {
        app.add_plugins(crate::multiplayer::udp::UdpTransportPlugin::balanced(crate::multiplayer::UDP_APPLICATION_VERSION));
    }

    // Remove setup mode resource
    app.world.remove_resource::<SetupMode>();
}