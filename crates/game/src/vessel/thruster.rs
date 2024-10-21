use crate::math::{units::*, curve::FloatCurve, transform::AxisSet3D};
use bevy::prelude::*;
use aery::prelude::*;
use serde::{Deserialize, Serialize};

/// A source of force that vessels need to move.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Thruster {
    /// The amount of force the thruster applies in each direction,
    /// relative to the orientation of the entity.
    pub force: AxisSet3D<Force>,

    /// Coefficient for the amount of force available, based on immersion into water.
    /// When fully immersed, the `x` is `1.0`. When fully out of the water, the `x` is `0.0`.
    pub efficiency: FloatCurve,
}

impl Default for Thruster {
    fn default() -> Self {
        Self {
            force: AxisSet3D::default(),
            efficiency: FloatCurve::linear_points([
                Vec2::new(0.0, 0.0),
                Vec2::new(1.0, 1.0),
            ]),
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