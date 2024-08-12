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
        &VesselRighting,
        &GlobalTransform,
        &mut ExternalAngularImpulse,
    )>,
) {
    const TARGET: Vec2 = Vec2::splat(0.0);

    for (righting, transform, mut impulse) in &mut vessels {
        // Compute pitch and roll
        let (_, pitch, roll) = transform.compute_transform().rotation.to_euler(EulerRot::YXZ);

        // Calculate how 'wrong' the vessel's orientation is
        let cur_rot = Vec2::new(pitch, roll);
        let offset = TARGET - cur_rot;
        let dist = offset.length();

        // Decide how much force is needed to right the vessel this tick
        let force = righting.force.sample(dist);

        // Apply righting force
        impulse.apply_impulse(Vec3 {
            x: offset.x * force,
            y: 0.0,
            z: offset.y * force,
        });
    }
}