use crate::prelude::*;

/// A device that stores electricity.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Battery {
    /// The amount of currently stored energy.
    pub stored: Energy,

    /// The maximum amount of power that can be stored.
    pub capacity: Energy,

    /// The speed at which the battery can recharge.
    pub recharge: Current,

    /// The speed at which electricity can be discharged.
    pub discharge: Current,

    /// The efficiency of recharging the battery.
    /// Ranges from `0.0` (0% efficient) to `1.0` (100% efficient).
    pub efficiency: f32,
}

impl Default for Battery {
    fn default() -> Self {
        Self {
            stored: Energy::new(0.0),
            capacity: Energy::new(0.0),
            recharge: Current::new(0.0),
            discharge: Current::new(0.0),
            efficiency: 1.0,
        }
    }
}