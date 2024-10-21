pub mod ticking;

use bevy::prelude::*;
use crate::initialisation::Initialisation;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, SubStates)]
#[source(Initialisation = Initialisation::Finished)]
pub enum SimulationLoaded {
    #[default]
    Unloaded,
    Loaded,
}

/// Disabled entities are frozen in time and will not be ticked by most game systems.
#[derive(Debug, Default, Clone, Copy, Component, PartialEq, Eq, PartialOrd, Ord, Reflect, Hash)]
#[reflect(Default, Component, PartialEq, Hash)]
pub struct Disabled;