use crate::{math::{curve::FloatCurve, transform::TranslateSet}, prelude::*};

/// Drag applied to the vessel as it moves.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct VesselDrag {
    pub translate: TranslateSet<FloatCurve>,
    pub rotation: TranslateSet<FloatCurve>,
}

/// Applies drag forces to vessels.
pub(super) fn vessel_drag_system(
    mut vessels: Query<(
        &LinearVelocity,
        &VesselDrag,
        &mut ExternalImpulse,
        &mut ExternalAngularImpulse,
    )>,
) {
    for (velocity, drag, mut impulse, mut torque) in &mut vessels {

    }
}