mod actions;

pub mod controller;

pub use actions::*;

use bevy::app::*;
use controller::VesselController;
use crate::{SetupMode, input::plugin::InputManagerPlugin};

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

        app.add_systems(Update, controller::vessel_controller_system);
    }
}