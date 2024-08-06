use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::{disabling::Disabled, math::transform::{RotationSet, TranslateSet}};
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
    pub rotation_force: RotationSet<f32>,
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

        // Some values for maths we use later on
        let fwd: Vec3 = transform.forward().into();
        let rgt: Vec3 = transform.right().into();
        let up: Vec3 = transform.up().into();

        // Translation intent value
        let mut translate_intent = Vec3::ZERO;

        // Keyboard movement inputs
        if actions.pressed(&VesselMovements::MoveUp   ) { translate_intent += up;  }
        if actions.pressed(&VesselMovements::MoveDown ) { translate_intent -= up;  }
        if actions.pressed(&VesselMovements::MoveLeft ) { translate_intent += rgt; }
        if actions.pressed(&VesselMovements::MoveRight) { translate_intent -= rgt; }
        if actions.pressed(&VesselMovements::MoveFwd  ) { translate_intent += fwd; }
        if actions.pressed(&VesselMovements::MoveBack ) { translate_intent -= fwd; }

        // Controller movement inputs
        if let Some(axis_pair) = actions.axis_pair(&VesselMovements::FwdSide) {
            let axes = axis_pair.xy();
            let vect = Vec3::new(axes.x, 0.0, axes.y) * fwd;
            translate_intent += vect;
        }

        // Clamp the intent to within a valid range
        // We intentionally do not normalise this
        translate_intent = translate_intent.clamp(
            Vec3::splat(-1.0),
            Vec3::splat(1.0),
        );

        // Calculate the force to be applied
        let translate_force = Vec3::new(
            translate_intent.x * if translate_intent.x > 0.0 { controller.translate_force.left } else { controller.translate_force.right },
            translate_intent.y * if translate_intent.y > 0.0 { controller.translate_force.up   } else { controller.translate_force.down  },
            translate_intent.z * if translate_intent.z > 0.0 { controller.translate_force.fwd  } else { controller.translate_force.back  },
        );

        // Apply the translation force
        impulse.impulse += translate_force;

        // Rotation intent value
        let mut rotation_intent = Vec3::ZERO;

        // Keyboard rotation inputs
        if actions.pressed(&VesselMovements::PitchUp  ) { rotation_intent += Vec3::Y     }
        if actions.pressed(&VesselMovements::PitchDown) { rotation_intent += Vec3::NEG_Y }
        if actions.pressed(&VesselMovements::YawLeft  ) { rotation_intent += Vec3::Z     }
        if actions.pressed(&VesselMovements::YawRight ) { rotation_intent += Vec3::NEG_Z }
        if actions.pressed(&VesselMovements::RollLeft ) { rotation_intent += Vec3::X     }
        if actions.pressed(&VesselMovements::RollRight) { rotation_intent += Vec3::NEG_X }

        // Controller/mouse movement inputs
        if let Some(axis_pair) = actions.axis_pair(&VesselMovements::PitchYaw) {
            let axes = axis_pair.xy();
            let vect = Vec3::new(0.0, axes.x, axes.y) * fwd;
            rotation_intent += vect;
        }

        // Clamp the intent to within a valid range
        // We intentionally do not normalise this
        rotation_intent = rotation_intent.clamp(
            Vec3::splat(-1.0),
            Vec3::splat(1.0),
        );

        // Calculate the force to be applied
        let rotation_force = Vec3::new(
            rotation_intent.x * if rotation_intent.x > 0.0 { controller.rotation_force.pitch_up  } else { controller.rotation_force.pitch_down },
            rotation_intent.y * if rotation_intent.y > 0.0 { controller.rotation_force.yaw_left  } else { controller.rotation_force.yaw_right  },
            rotation_intent.z * if rotation_intent.z > 0.0 { controller.rotation_force.roll_left } else { controller.rotation_force.roll_right },
        );

        // Apply the rotation force
        impulse.torque_impulse += rotation_force;
    }
}