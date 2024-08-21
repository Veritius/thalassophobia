use crate::{prelude::*, math::curve::FloatCurve};

/// Drag applied to the vessel as it moves.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct VesselAntiRoll {
    pub force: FloatCurve,
}

/// Applies drag forces to vessels.
pub(super) fn vessel_roll_system(
    mut vessels: Query<(
        &VesselAntiRoll,
        &mut Transform,
        &mut ExternalAngularImpulse,
    )>,
) {
    for (component, mut transform, mut torque) in &mut vessels {
        // Get the roll value for the entity
        let (_, _, roll) = transform.rotation.to_euler(EulerRot::YXZ);

        if roll.abs() > 3.0 {
            // If this is reached it means it's completely flipped
            // We can't rely on a counterforce to correct it in this case
            // Instead we just manually set the roll to zero
            // This probably causes physics issues but eh
            transform.rotate_local_z(-roll);
            continue;
        }

        // Calculate a counterforce to correct the incorrect rotation
        let force = -roll * 0.3; // TODO: Use FloatCurve
        let force = transform.rotation * Vec3::new(0.0, 0.0, force);
        torque.apply_impulse(force);
    }
}