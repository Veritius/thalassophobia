use bevy::{app::PluginGroupBuilder, prelude::*};

pub use crate::initialisation::InitialLoadingPlugin;
pub use crate::physics::PhysicsPlugin;

#[cfg(feature="devtools")]
pub use crate::devtools::DevtoolsPlugin;

pub struct EssentialPlugins;

impl PluginGroup for EssentialPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(crate::players::PlayerPlugin)
    }
}

pub struct MechanicPlugins;

impl PluginGroup for MechanicPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(crate::power::ElectricityPlugin)
            .add(crate::structure::StructuresPlugin)
            .add(crate::vessel::VesselsPlugin)
    }
}

pub struct ControllerPlugins;

impl PluginGroup for ControllerPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(crate::character::movement::PlayerControllerPlugin)
            .add(crate::vessel::piloting::VesselControllerPlugin)
    }
}