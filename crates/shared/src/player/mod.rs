use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use crate::state::simulation_running;

pub mod movement;
pub mod controller;

pub(crate) struct PlayerCharacterPlugin;

impl Plugin for PlayerCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<movement::GroundedHumanMovements>::server());
        app.add_plugins(InputManagerPlugin::<movement::FloatingHumanMovements>::server());

        app.add_systems(Update, (
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