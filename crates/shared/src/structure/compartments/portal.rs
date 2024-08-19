use crate::prelude::*;

/// A gap between two compartments.
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct CompartmentPortal {
    /// The smallest cross-section of the portal's volume.
    /// Used to determine flow rate for water calculations.
    pub transfer_area: Area,
}