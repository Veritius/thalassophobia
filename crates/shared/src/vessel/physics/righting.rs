use crate::{math::curve::FloatCurve, prelude::*};

/// Attempts to bring the vessel into a better orientation automatically.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct VesselRighting {
    pub force: FloatCurve,
}

/// Attempts to bring vessels into better orientations.
pub(super) fn vessel_righting_system(
    mut vessels: Query<(
        &Transform,
        &mut ExternalAngularImpulse,
    )>,
) {
    for (transform, impulse) in &mut vessels {

    }
}