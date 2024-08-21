mod drag;
mod roll;

pub use drag::VesselDrag;
pub use roll::VesselAntiRoll;

use crate::prelude::*;

pub(crate) struct VesselPhysicsPlugin;

impl Plugin for VesselPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(SimulationUpdate, (
            drag::vessel_drag_system,
            roll::vessel_roll_system,
        ));
    }
}