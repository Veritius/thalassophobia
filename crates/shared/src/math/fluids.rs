use crate::prelude::*;

pub fn hydrostatic_pressure(
    density: Density,
    gravity: Force,
    depth: Length,
) -> Pressure {
    let pressure = density.inner() * gravity.inner() * depth.inner();
    return Pressure::new(pressure);
}

pub mod water {
    use super::*;

    /// The density of water at its freezing point (0 degrees Celcius).
    pub const DENSITY: Density = Density::new_unchecked(
        Weight::new(999),
        Volume::new(1000),
    );
}