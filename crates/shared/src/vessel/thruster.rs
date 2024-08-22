use crate::{math::transform::AxisSet3D, prelude::*};

/// A source of force that vessels need to move.
#[derive(Debug, Default, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Thruster {
    /// Sets whether the thruster needs to be submerged to function.
    /// If set to `true`, force will be zero outside of water.
    pub needs_water: bool,

    /// The maximum force that the thruster can output.
    pub force: Force,
}

/// A source of drag that resists the movement of vessels.
#[derive(Debug, Default, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Dragger {
    /// Sets whether the dragger needs to be submerged to function.
    /// If set to `true`, force will be zero outside of water.
    pub needs_water: bool,

    /// The amount of force the dragger applies in each direction,
    /// relative to the orientation of the entity.
    pub drag: AxisSet3D<Force>,
}