use crate::{math::{curve::FloatCurve, transform::AxisSet3D}, prelude::*};

/// Drag applied to the vessel as it moves.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct VesselDrag {
    pub translate: AxisSet3D<FloatCurve>,
    pub rotation: AxisSet3D<FloatCurve>,
}

/// Applies drag forces to vessels.
pub(super) fn vessel_drag_system(
    mut vessels: Query<(
        &VesselDrag,
        &LinearVelocity,
        &mut ExternalImpulse,
        &mut ExternalAngularImpulse,
    )>,
) {
    for (drag, velocity, mut impulse, mut torque) in &mut vessels {

    }
}