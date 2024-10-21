mod actions;

pub mod controller;

pub use actions::*;

use bevy::prelude::*;
use controller::VesselController;
use leafwing_input_manager::plugin::InputManagerPlugin;
use crate::simulation::ticking::SimulationUpdate;

pub(crate) struct VesselControllerPlugin;

impl Plugin for VesselControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<actions::VesselMovements>::default());

        app.register_type::<VesselController>();

        app.add_systems(SimulationUpdate, controller::vessel_controller_system);
    }
}