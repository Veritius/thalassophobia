use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A source of electricity.
#[derive(Debug, Default, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct PowerSource {
    /// The amount of energy the power source supplies.
    pub supply: Energy,
}

/// The calculated values from a [`PowerSource`].
#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct CalculatedPowerSource {
    pub(super) load: f32,
}

impl CalculatedPowerSource {
    /// The fraction of `supply` that is being used by sinks, from `0.0` to `1.0`.
    pub fn load(&self) -> f32 {
        self.load
    }
}

#[derive(Bundle)]
pub struct PowerSourceBundle {
    pub component: PowerSource,
    pub calculated: CalculatedPowerSource,
}