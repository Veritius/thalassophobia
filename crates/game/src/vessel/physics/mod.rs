mod limit;

pub use limit::VesselAngleLimit;

use bevy::prelude::*;
use crate::{math::transform::{X, Y, Z}, simulation::ticking::SimulationUpdate};

pub(crate) struct VesselPhysicsPlugin;

impl Plugin for VesselPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<VesselAngleLimit<X>>();
        app.register_type::<VesselAngleLimit<Y>>();
        app.register_type::<VesselAngleLimit<Z>>();

        app.add_systems(SimulationUpdate,
            limit::vessel_limit_system,
        );
    }
}