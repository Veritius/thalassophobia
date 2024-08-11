use crate::prelude::*;
use super::Joule;

/// A device that stores electricity.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Battery {
    /// The amount of currently stored energy.
    pub stored: Joule,

    /// The maximum amount of power that can be stored.
    pub capacity: Joule,

    /// The speed at which the battery can recharge.
    pub recharge: u32,

    /// The speed at which electricity can be discharged.
    pub discharge: u32,

    /// The efficiency of recharging the battery.
    /// Ranges from `0.0` (0% efficient) to `1.0` (100% efficient).
    pub efficiency: f32,
}

impl Default for Battery {
    fn default() -> Self {
        Self {
            stored: Joule::from(0),
            capacity: Joule::from(0),
            recharge: 1,
            discharge: 1,
            efficiency: 1.0,
        }
    }
}