mod actions;

pub mod controller;

pub use actions::*;

use bevy::prelude::*;
use controller::VesselController;
use crate::{input::plugin::InputManagerPlugin, state::simulation_running, SetupMode};

pub(crate) struct VesselControllerPlugin;

impl Plugin for VesselControllerPlugin {
    fn build(&self, app: &mut App) {
        let setup_mode = app.world_mut().resource::<SetupMode>();

        match setup_mode {
            SetupMode::Full => {
                app.add_plugins(InputManagerPlugin::<actions::VesselMovements>::default());
            },
            SetupMode::Headless => {
                app.add_plugins(InputManagerPlugin::<actions::VesselMovements>::server());
            },
        }

        app.register_type::<VesselController>();

        app.add_systems(Update, controller::vessel_controller_system
            .run_if(simulation_running()));
    }
}