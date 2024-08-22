use std::{marker::PhantomData, ops::RangeInclusive};
use crate::{math::transform::{Axis, X, Y, Z}, prelude::*};

/// Imposes limits to the orientation of a vessel around an axis.
/// The component is applied only to local space.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct VesselAngleLimit<R: Axis> {
    /// A coefficient for the amount of torque used to enforce `limit`.
    /// Higher values will correct invalid rotations faster.
    pub force: f32,

    /// The range that the vessel will attempt to stay within.
    /// If the range is exceeded, a counterforce is applied to return it to a valid range.
    /// If obstructed, the force will be continually applied until it is corrected.
    pub limit: Option<RangeInclusive<f32>>,

    /// The range that the vessel must stay within.
    /// If the range is exceeded, the vessel is immediately reset to the nearest valid point.
    /// This ignores the physics engine and will directly alter the object's orientation.
    pub reset: Option<RangeInclusive<f32>>,

    #[reflect(ignore)]
    pub _phantom: PhantomData<R>,
}

impl<R: Axis> Default for VesselAngleLimit<R> {
    fn default() -> Self {
        Self {
            force: 0.0,

            limit: None,
            reset: None,

            _phantom: PhantomData,
        }
    }
}

pub(super) fn vessel_limit_system(
    mut vessels: Query<(
        Option<&VesselAngleLimit<X>>,
        Option<&VesselAngleLimit<Y>>,
        Option<&VesselAngleLimit<Z>>,

        &Mass,
        &mut Transform,
        &mut ExternalAngularImpulse,
    ), Or<(
        With<VesselAngleLimit<X>>,
        With<VesselAngleLimit<Y>>,
        With<VesselAngleLimit<Z>>,
    )>>,
) {
    for (
        lm_x,
        lm_y,
        lm_z,

        mass,
        mut transform,
        mut torque,
    ) in &mut vessels {
        fn angle_limit<R: Axis>(
            current: f32,
            limit: Option<&VesselAngleLimit<R>>,
            mass: &Mass,
            mut transform: Mut<Transform>,
            mut torque: Mut<ExternalAngularImpulse>,
        ) {
            // If there is no limit, we don't care
            if limit.is_none() { return }
            let limit = limit.unwrap();

            'reset: { if let Some(range) = &limit.reset {
                // If the current angle is within range, don't do anything
                if range.contains(&current) { break 'reset; }

                todo!()
            } }

            'limit: { if let Some(range) = &limit.limit {
                // If the current angle is within range, don't do anything
                if range.contains(&current) { break 'limit; }

                #[inline]
                fn abs_diff(a: f32, b: f32) -> f32 {
                    if a > b { a - b } else { b - a }
                }

                // How far past the acceptable range the value is
                let sd = abs_diff(*range.start(), current);
                let ed = abs_diff(*range.end(), current);
                let diff = if ed > sd { sd } else { ed };
                dbg!(sd, ed, diff);

                // Calculate the force to apply, and apply it
                let sign = if diff.is_sign_negative() { 1.0 } else { -1.0 };
                let force = diff.powi(2) * limit.force * sign;
                let turned = transform.rotation * R::vect(force);
                torque.apply_impulse(turned);
            } }
        }

        // Get the orientation of the entity in all three axes
        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

        // Apply to all three axes
        angle_limit(pitch, lm_x, mass, transform.reborrow(), torque.reborrow());
        angle_limit(yaw,   lm_y, mass, transform.reborrow(), torque.reborrow());
        angle_limit(roll,  lm_z, mass, transform.reborrow(), torque.reborrow());
    }
}