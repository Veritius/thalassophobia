use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::{disabling::Disabled, vessel::{RotationSet, TranslateSet}};
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
        let fwd: Vec3 = transform.forward().into();
        let rgt: Vec3 = transform.right().into();
        let up: Vec3 = transform.up().into();

        // Translation intent value
        let mut translate_intent = Vec3::ZERO;

        // Keyboard movement inputs
        if actions.pressed(&VesselMovements::MoveUp    ) { translate_intent += up;  }
        if actions.pressed(&VesselMovements::MoveDown  ) { translate_intent -= up;  }
        if actions.pressed(&VesselMovements::MoveLeft  ) { translate_intent += rgt; }
        if actions.pressed(&VesselMovements::MoveRight ) { translate_intent -= rgt; }
        if actions.pressed(&VesselMovements::MoveFwd   ) { translate_intent += fwd; }
        if actions.pressed(&VesselMovements::MoveBack  ) { translate_intent -= fwd; }

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
    }
}