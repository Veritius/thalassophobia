mod flooding;

pub use flooding::CompartmentWater;

use crate::prelude::*;

/// A compartment in a structure.
#[derive(Debug, Default, Clone, Copy, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Compartment {
    /// The overall volume of the compartment.
    pub volume: Litre,
}

#[derive(Bundle)]
pub struct CompartmentBundle {
    pub main: Compartment,
    pub water: CompartmentWater,
}