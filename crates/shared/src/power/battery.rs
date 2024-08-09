use crate::prelude::*;
use super::Joule;

/// A device that stores electricity.
#[derive(Debug, Default, Clone, Component, Reflect, Serialize, Deserialize)]
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
}