use bevy::prelude::*;
use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SetupMode {
    Full,
    Headless,
}

pub fn setup(app: &mut App, mode: SetupMode) {
    // Physics engine
    app.add_plugins(crate::avian::PhysicsPlugins::new(SimulationUpdate));

    // Major plugins that are added before subsystems
    app.add_plugins(crate::initial::InitialLoadingPlugin);
    app.add_plugins(crate::simulation::SimulationStatePlugin);
    app.add_plugins(crate::players::PlayerPlugin);

    // Significant plugins that are added before subsystems, but after major plugins
    app.add_plugins(crate::multiplayer::MultiplayerPlugin);

    // Subsystem plugins
    app.add_plugins(crate::bodies::BodyPlugin);
    app.add_plugins(crate::campaign::CampaignPlugin);
    app.add_plugins(crate::character::movement::PlayerControllerPlugin { mode });
    app.add_plugins(crate::disabling::DisablingPlugin);
    app.add_plugins(crate::living::LivingPlugin);
    app.add_plugins(crate::package::ContentPackagesPlugin);
    app.add_plugins(crate::power::ElectricityPlugin);
    app.add_plugins(crate::structure::compartments::CompartmentsPlugin);
    app.add_plugins(crate::structure::StructuresPlugin);
    app.add_plugins(crate::vessel::VesselsPlugin { mode });
}