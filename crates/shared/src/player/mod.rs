use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use crate::{state::simulation_running, SetupMode};

pub mod movement;
pub mod controller;
pub mod queries;

pub(crate) struct PlayerCharacterPlugin;

impl Plugin for PlayerCharacterPlugin {
    fn build(&self, app: &mut App) {
        let setup_mode = app.world.resource::<SetupMode>();

        match setup_mode {
            SetupMode::Headless => {
                app.add_plugins(InputManagerPlugin::<movement::GroundedHumanMovements>::server());
                app.add_plugins(InputManagerPlugin::<movement::FloatingHumanMovements>::server());
            },
            SetupMode::Full => {
                app.add_plugins(InputManagerPlugin::<movement::GroundedHumanMovements>::default());
                app.add_plugins(InputManagerPlugin::<movement::FloatingHumanMovements>::default());
            },
        }

        app.add_systems(Update, (
            controller::touching_ground_system,
            controller::grounded_rotation_system,
            controller::grounded_movement_system,
        ).chain()
            .run_if(simulation_running())
            .in_set(PlayerCharacterSystemSet::Controller));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub enum PlayerCharacterSystemSet {
    Controller,
}