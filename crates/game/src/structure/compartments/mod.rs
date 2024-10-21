mod flooding;
mod portal;

pub use flooding::CompartmentWater;
pub use portal::CompartmentPortal;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub(crate) struct CompartmentsPlugin;

impl Plugin for CompartmentsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CompartmentWater>();
        app.register_type::<CompartmentPortal>();
    }
}

/// A compartment in a structure.
#[derive(Debug, Default, Clone, Copy, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Compartment {
    /// The overall volume of the compartment.
    pub volume: Volume,
}

#[derive(Bundle)]
pub struct CompartmentBundle {
    pub main: Compartment,
    pub water: CompartmentWater,
}