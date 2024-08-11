use crate::prelude::*;
use super::Joule;

/// A device that uses electricity.
#[derive(Debug, Default, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct PowerSink {
    /// The amount of energy the power sink uses.
    pub load: Joule,

    /// The fraction of `load` which is satisfied, from `0.0` to `1.0`.
    pub supply: f32,
}

/// The calculated values from a [`PowerSink`].
#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct CalculatedPowerSink {
    pub(super) supply: f32,
}

impl CalculatedPowerSink {
    /// The fraction of `load` which is satisfied, from `0.0` to `1.0`.
    pub fn supply(&self) -> f32 {
        self.supply
    }
}

#[derive(Bundle)]
pub struct PowerSinkBundle {
    pub component: PowerSink,
    pub calculated: CalculatedPowerSink,
}
