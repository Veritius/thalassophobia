use crate::{prelude::*, math::curve::FloatCurve};

/// Applies a force to counteract roll in a vessel.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub enum VesselAntiRoll {
    /// Adapts to the body's physical attributes, like mass.
    Adaptive {
        /// A coefficient applied to the calculated force.
        /// Higher values will mean the vessel returns to zero faster.
        coefficient: f32,
    },

    /// Samples from a curve using the vessel's roll.
    /// Doesn't adapt to changes in the vessel.
    Static {
        /// A curve that defines the [`Torque`] applied.
        /// 
        /// The range of possible inputs is from `0.0` to `3.0` radians.
        torque: FloatCurve,
    },
}

/// Applies drag forces to vessels.
pub(super) fn vessel_roll_system(
    mut vessels: Query<(
        &VesselAntiRoll,
        &Mass,
        &mut Transform,
        &mut ExternalAngularImpulse,
    )>,
) {
    for (component, mass, mut transform, mut torque) in &mut vessels {
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
        let force = match component {
            VesselAntiRoll::Adaptive {
                coefficient,
            } => {
                -roll * coefficient * (mass.0 / 4.0)
            },

            VesselAntiRoll::Static {
                torque: curve,
            } => {
                let sign = match roll.is_sign_negative() {
                    true => -1.0,
                    false => 1.0,
                };

                curve.sample(roll.abs()) * sign
            },
        };

        // Apply the torque impulse
        torque.apply_impulse(transform.rotation * Vec3::new(0.0, 0.0, force));
    }
}