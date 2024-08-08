use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::{disabling::Disabled, math::transform::TranslateSet};
use super::VesselMovements;

/// A controller for a submarine.
#[derive(Debug, Component, Reflect)]
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
        &ActionState<VesselMovements>,
    ), Without<Disabled>>,
) {
    for (
        controller,
        transform,
        mut impulse,
        actions
    ) in bodies.iter_mut() {
        // Translation intent value
        let mut translate_intent = Vec3::ZERO;

        // Keyboard movement inputs
        translate_intent.x += actions.clamped_value(&VesselMovements::SideThrust);
        translate_intent.y += actions.clamped_value(&VesselMovements::VerticalThrust);
        translate_intent.z -= actions.clamped_value(&VesselMovements::ForwardThrust);

        // Calculate the force to be applied
        translate_intent *= transform.forward().as_vec3();
        let translate_force = translate_intent * controller.rotation_force;

        // Apply the translation force
        impulse.impulse += translate_force;

        // Rotation intent value
        let mut rotation_intent = Vec3::ZERO;

        // Rotation inputs
        rotation_intent.x += actions.clamped_value(&VesselMovements::Pitch);
        rotation_intent.y += actions.clamped_value(&VesselMovements::Yaw);
        rotation_intent.z += actions.clamped_value(&VesselMovements::Roll);

        // Calculate the force to be applied
        let rotation_force = rotation_intent * controller.rotation_force;

        // Apply the rotation force
        impulse.torque_impulse += rotation_force;
    }
}