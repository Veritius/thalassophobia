use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, States)]
pub enum SimulationState {
    #[default]
    Stopped,
    Running,
}

pub(super) fn pause_simulation(
    mut time: ResMut<Time<Virtual>>,
) {
    time.pause();
}

pub(super) fn unpause_simulation(
    mut time: ResMut<Time<Virtual>>,
) {
    time.unpause();
}