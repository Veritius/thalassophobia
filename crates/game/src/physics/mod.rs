pub mod masks;

use bevy::prelude::*;
use crate::simulation::ticking::SimulationUpdate;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // Physics simulations are only run in the simulation update tick
        app.add_plugins(avian3d::PhysicsPlugins::new(SimulationUpdate));
    }
}