use crate::{math::transform::TranslateSet, prelude::*};
use super::VesselMovements;

/// A controller for a submarine.
#[derive(Debug, Clone, Component, Reflect)]
#[reflect(from_reflect = false, Component)]
pub struct VesselController {
    /// The maximum amount of force that can be applied to translate the vessel.
    pub translate_force: TranslateSet<f32>,

    /// The maximum amount of force that can be applied to rotate the vessel.
    pub rotation_force: TranslateSet<f32>,
}

pub(super) fn vessel_controller_system(
    mut bodies: Query<(
        &VesselController,
        &Transform,
        &mut ExternalImpulse,
        &mut ExternalAngularImpulse,
        &ActionState<VesselMovements>,
    ), Without<Disabled>>,
) {
    for (
        controller,
        transform,
        mut impulse,
        mut torque,
        actions
    ) in bodies.iter_mut() {
        // Translation intent value
        let mut translate_intent = Vec3::ZERO;

        // Horizontal movement inputs
        translate_intent.x += actions.clamped_value(&VesselMovements::SideThrust);
        translate_intent.z -= actions.clamped_value(&VesselMovements::ForwardThrust);

        // Calculate the force to be applied
        translate_intent = transform.rotation.mul_vec3(translate_intent);

        // Vertical input handling
        // This is separate so that horizontal movements are relative to the vessel's orientation,
        // but vertical movements are not. Also clamping to avoid exploits.
        translate_intent.y += actions.value(&VesselMovements::VerticalThrust);
        translate_intent.y = translate_intent.y.clamp(-1.0, 1.0);

        // Calculate the force to apply to the vessel
        let translate_impulse = translate_intent * controller.rotation_force;

        // Apply the translation force
        impulse.apply_impulse(translate_impulse);

        // Rotation intent value
        let mut rotation_intent = Vec3::ZERO;

        // Rotation inputs
        rotation_intent.x += actions.clamped_value(&VesselMovements::Pitch);
        rotation_intent.y -= actions.clamped_value(&VesselMovements::Yaw);
        rotation_intent.z += actions.clamped_value(&VesselMovements::Roll);

        // Calculate the force to be applied
        let rotation_impulse = {
            let force = rotation_intent * controller.rotation_force;
            transform.rotation.mul_vec3(force)
        };

        // Apply the rotation force
        torque.apply_impulse(rotation_impulse);
    }
}