pub mod physics;
pub mod piloting;
pub mod thruster;

use bevy::prelude::*;

/// A marker component for vessels.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component, Reflect)]
#[reflect(Default, PartialEq, Component)]
pub struct Vessel;

pub(crate) struct VesselsPlugin {
    pub mode: SetupMode,
}

impl Plugin for VesselsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Vessel>();
        app.register_type::<thruster::Thruster>();
        app.register_type::<thruster::ComputedThrust>();
        app.register_type::<thruster::Dragger>();
        app.register_type::<thruster::ComputedDrag>();

        app.register_relation::<thruster::Influences>();

        app.add_plugins(physics::VesselPhysicsPlugin);
        app.add_plugins(piloting::VesselControllerPlugin { mode: self.mode });
    }
}