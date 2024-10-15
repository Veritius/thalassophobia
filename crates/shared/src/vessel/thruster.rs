use crate::{math::transform::AxisSet3D, prelude::*};

/// A source of force that vessels need to move.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Thruster {
    /// Sets whether the thruster needs to be submerged to function.
    /// If set to `true`, force will be zero outside of water.
    pub needs_water: bool,

    /// The amount of force the thruster applies in each direction,
    /// relative to the orientation of the entity.
    pub force: AxisSet3D<Force>,
}

impl Default for Thruster {
    fn default() -> Self {
        Self {
            needs_water: true,
            force: AxisSet3D::default(),
        }
    }
}

/// Computed thrust force for a vessel, from [`Thruster`] components.
#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct ComputedThrust {
    force: AxisSet3D<Force>,
}

/// A source of drag that resists the movement of vessels.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Dragger {
    /// The amount of drag the dragger applies in each direction,
    /// relative to the orientation of the entity.
    pub drag: AxisSet3D<Force>,
}

impl Default for Dragger {
    fn default() -> Self {
        Self {
            drag: AxisSet3D::default(),
        }
    }
}

/// Computed drag force for a vessel, from [`Dragger`] components.
#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Component)]
pub struct ComputedDrag {
    drag: AxisSet3D<Force>,
}

/// Relation from [`Thruster`] and [`Dragger`] entities to [`Vessel`](super::Vessel) entities.
/// When this relation exists, the vessel will consider their influence in force calculations.
#[derive(Relation)]
#[aery(Poly)]
pub struct Influences;