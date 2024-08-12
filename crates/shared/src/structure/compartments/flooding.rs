use crate::prelude::*;
use super::*;

/// How flooded a [`Compartment`] is.
#[derive(Debug, Default, Clone, Copy, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CompartmentWater {
    volume: Volume,
}

impl CompartmentWater {
    /// Returns the weight of the flooded compartment.
    pub fn weight(
        &self,
        compartment: &Compartment,
        gravity: Force,
    ) -> Weight {
        let volume = self.volume.min(compartment.volume);

        todo!()
    }
}