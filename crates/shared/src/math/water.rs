use crate::prelude::*;

/// The density of water at its freezing point (0 degrees Celcius).
pub const DENSITY: Density = Density::new_unchecked(
    Weight::new(999),
    Volume::new(1000),
);