use crate::prelude::*;

/// A device that uses electricity.
#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct PowerSink {
    /// The amount of energy the power sink uses.
    pub load: f32,

    /// The fraction of `load` which is satisfied, from `0.0` to `1.0`.
    pub supply: f32,
}