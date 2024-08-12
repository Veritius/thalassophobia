mod drag;

pub use drag::VesselDrag;

use crate::prelude::*;

pub(crate) struct VesselPhysicsPlugin;

impl Plugin for VesselPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(SimulationUpdate, drag::vessel_drag_system);
    }
}