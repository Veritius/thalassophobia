mod drag;
mod limit;

pub use drag::VesselDrag;
pub use limit::VesselAngleLimit;

use crate::{math::transform::{X, Y, Z}, prelude::*};

pub(crate) struct VesselPhysicsPlugin;

impl Plugin for VesselPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<VesselAngleLimit<X>>();
        app.register_type::<VesselAngleLimit<Y>>();
        app.register_type::<VesselAngleLimit<Z>>();

        app.add_systems(SimulationUpdate, (
            drag::vessel_drag_system,
            limit::vessel_limit_system,
        ));
    }
}