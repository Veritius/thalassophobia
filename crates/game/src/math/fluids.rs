use super::units::*;

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
    pub const DENSITY: Density = Density::new(0.999_842_83);

    pub fn water_pressure(
        gravity: Force,
        depth: Length,
    ) -> Pressure {
        hydrostatic_pressure(
            DENSITY,
            gravity,
            depth,
        )
    }
}