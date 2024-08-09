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

    // Multiplayer (added before subsystems)
    #[cfg(feature="multiplayer")]
    app.add_plugins(crate::multiplayer::MultiplayerPlugin);

    // Custom schedules
    app.add_plugins(crate::schedules::SimulationSchedulesPlugin);

    // Subsystem plugins
    app.add_plugins(crate::campaign::CampaignPlugin);
    app.add_plugins(crate::character::movement::PlayerControllerPlugin { mode });
    app.add_plugins(crate::disabling::DisablingPlugin);
    app.add_plugins(crate::package::ContentPackagesPlugin);
    app.add_plugins(crate::power::ElectricityPlugin);
    app.add_plugins(crate::vessel::piloting::VesselControllerPlugin { mode });
}