use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, States)]
pub enum SimulationState {
    #[default]
    Offline,

    Frozen,

    Simulating,
}