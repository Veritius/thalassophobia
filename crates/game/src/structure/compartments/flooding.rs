use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::math::{fluids::water::DENSITY, units::*};

/// How flooded a [`Compartment`](super::Compartment) is.
#[derive(Debug, Default, Clone, Copy, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CompartmentWater {
    volume: Volume,
}

impl CompartmentWater {
    /// Returns the [weight](Weight) of the flooded compartment.
    pub fn weight(&self) -> Weight {
        return self.volume * DENSITY;
    }
}