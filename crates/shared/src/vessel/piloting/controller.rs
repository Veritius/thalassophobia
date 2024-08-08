use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::{disabling::Disabled, math::transform::TranslateSet};
use super::VesselMovements;

/// The 'style' of movement for vessels.
#[derive(Default, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum VesselMoveStyle {
    /// The vessel will not attempt to correct any movements.
    #[default]
    Manual,

    /// The vessel will maintain speed and course.
    Maintain,
}

/// A controller for a submarine.
#[derive(Debug, Component, Reflect)]
pub struct VesselController {
    /// The current movement style of the vessel.
    pub move_style: VesselMoveStyle,

    /// The maximum amount of force that can be applied to translate the vessel.
    pub translate_force: TranslateSet<f32>,

    /// The maximum amount of force that can be applied to rotate the vessel.
    pub rotation_force: TranslateSet<f32>,
}

pub(super) fn vessel_controller_system(
    mut bodies: Query<(
        &mut VesselController,
        &Transform,
        &mut ExternalImpulse,
        &ActionState<VesselMovements>,
    ), Without<Disabled>>,
) {
    for (
        mut controller,
        transform,
        mut impulse,
        actions
    ) in bodies.iter_mut() {
        // Let the person change the control style ahead of time
        if actions.pressed(&VesselMovements::ChangeStyle) {
            controller.move_style = match controller.move_style {
                VesselMoveStyle::Manual => VesselMoveStyle::Maintain,
                VesselMoveStyle::Maintain => VesselMoveStyle::Manual,
            }
        }

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