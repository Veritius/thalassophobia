use crate::prelude::*;

/// A source of electricity.
#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct PowerSource {
    /// The amount of energy the power source supplies.
    pub supply: f32,

    /// The fraction of `supply` that is being used, from `0.0` to `1.0`.
    pub load: f32,
}