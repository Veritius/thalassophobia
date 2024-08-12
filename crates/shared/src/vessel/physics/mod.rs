mod drag;
mod righting;

pub use drag::VesselDrag;
pub use righting::VesselRighting;

use crate::prelude::*;

pub(crate) struct VesselPhysicsPlugin;

impl Plugin for VesselPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(SimulationUpdate, (
            drag::vessel_drag_system,
            righting::vessel_righting_system,
        ));
    }
}