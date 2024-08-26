use crate::{math::transform::AxisSet3D, prelude::*};
use super::VesselMovements;

/// A controller for a submarine.
#[derive(Debug, Clone, Component, Reflect)]
#[reflect(from_reflect = false, Component)]
pub struct VesselController {
    /// The maximum amount of force that can be applied to translate the vessel.
    pub translate_force: AxisSet3D<f32>,

    /// The maximum amount of force that can be applied to rotate the vessel.
    pub rotation_force: AxisSet3D<f32>,
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
        let mut absolute_translate_intent = Vec3::ZERO;
        let mut relative_translate_intent = Vec3::ZERO;

        // Horizontal movement inputs
        relative_translate_intent.x += actions.clamped_value(&VesselMovements::SideThrust);
        relative_translate_intent.z -= actions.clamped_value(&VesselMovements::ForwardThrust);
        absolute_translate_intent.y += actions.clamped_value(&VesselMovements::VerticalThrust);

        let absolute_intent_forces = absolute_translate_intent * controller.translate_force;
        let relative_intent_forces = relative_translate_intent * controller.translate_force;
        let aligned_relative_forces = transform.rotation.mul_vec3(relative_intent_forces);
        let translate_impulse = absolute_intent_forces +  aligned_relative_forces;

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