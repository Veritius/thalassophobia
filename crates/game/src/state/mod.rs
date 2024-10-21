mod initial;
mod sim_tick;

use bevy::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
pub enum Initialisation {
    #[default]
    Loading,
    Finished,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, SubStates)]
#[source(Initialisation = Initialisation::Finished)]
pub enum LoadState {
    #[default]
    Unloaded,
    Loaded,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, SubStates)]
#[source(LoadState = LoadState::Loaded)]
pub enum Simulation {
    #[default]
    Paused,
    Running,
}