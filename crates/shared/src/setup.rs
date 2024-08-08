use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SetupMode {
    Full,
    Headless,
}

pub fn setup(app: &mut App, mode: SetupMode) {
    // Physics engine
    app.add_plugins(crate::rapier::plugin::RapierPhysicsPlugin::<()>::default());
    #[cfg(feature="phys_debug")]
    app.add_plugins(crate::rapier::render::RapierDebugRenderPlugin::default());

    // Multiplayer (added before subsystems)
    #[cfg(feature="multiplayer")]
    app.add_plugins(crate::multiplayer::MultiplayerPlugin);

    // Subsystem plugins
    app.add_plugins(crate::campaign::CampaignPlugin);
    app.add_plugins(crate::character::movement::PlayerControllerPlugin { mode });
    app.add_plugins(crate::disabling::DisablingPlugin);
    app.add_plugins(crate::package::ContentPackagesPlugin);
    app.add_plugins(crate::state::GameStatePlugin);
    app.add_plugins(crate::vessel::piloting::VesselControllerPlugin { mode });
}