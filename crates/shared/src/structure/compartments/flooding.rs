use crate::prelude::*;
use super::*;

/// How flooded a [`Compartment`] is.
#[derive(Debug, Default, Clone, Copy, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CompartmentWater {
    volume: Volume,
}

impl CompartmentWater {
    /// Returns the [weight](Weight) of the flooded compartment.
    pub fn weight(
        &self,
        compartment: &Compartment,
    ) -> Weight {
        let volume = self.volume.min(compartment.volume);
        let weight = crate::math::water::DENSITY * volume;
        return weight;
    }
}