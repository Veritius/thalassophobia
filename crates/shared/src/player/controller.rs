use bevy::prelude::*;
use bevy_rapier3d::dynamics::ExternalImpulse;
use leafwing_input_manager::prelude::*;
use crate::disabling::Disabled;

use super::movement::*;

const CONTROLLER_PITCH_MIN: f32 = -0.8;
const CONTROLLER_PITCH_MAX: f32 = 0.8;

#[derive(Component)]
pub struct PlayerController {
    pub rotation_yaw: f32,

    pub head_entity: Option<Entity>,
}

#[derive(Component)]
pub struct PlayerControllerCamera {
    pub rotation_pitch: f32,
}

pub(super) fn grounded_rotation_system(
    mut bodies: Query<(&mut PlayerController, &mut Transform, &ActionState<GroundedHumanMovements>), (Without<PlayerControllerCamera>, Without<Disabled>)>,
    mut heads: Query<(&mut PlayerControllerCamera, &mut Transform), Without<PlayerController>>,
) {
    for (mut body_data, mut body_transform, body_actions) in bodies.iter_mut() {
        // Try to read any rotation inputs
        let axis_input = match body_actions.axis_pair(&GroundedHumanMovements::Turn) {
            Some(val) => {
                let mut vx = val.xy();
                vx.x *= 0.0070; // left and right
                vx.y *= 0.0040; // up and down
                vx
            },
            None => { continue },
        };

        // Update the body's rotation
        body_data.rotation_yaw += axis_input.x;
        body_transform.rotation = Quat::from_axis_angle(Vec3::Y, -body_data.rotation_yaw);

        // Get the head component
        let (mut head_data, mut head_transform) = match heads.get_mut(match body_data.head_entity {
            Some(v) => v,
            None => { continue },
        }) {
            Ok(v) => v,
            Err(_) => { continue },
        };

        // Update the head's rotation
        head_data.rotation_pitch += axis_input.y;
        head_data.rotation_pitch = head_data.rotation_pitch.clamp(CONTROLLER_PITCH_MIN, CONTROLLER_PITCH_MAX);
        head_transform.rotation = Quat::from_axis_angle(Vec3::X, -head_data.rotation_pitch);
    }
}

pub(super) fn grounded_movement_system(
    mut bodies: Query<(&Transform, &mut ExternalImpulse, &ActionState<GroundedHumanMovements>), (With<PlayerController>, Without<Disabled>)>,
) {
    for (&body_transform, mut body_impulse, body_actions) in bodies.iter_mut() {
        let mut move_intent = Vec2::ZERO;

        let lz = body_transform.local_z();
        let fwd = -Vec2::new(lz.x, lz.z);
        let rgt = Vec2::new(lz.z, -lz.x);

        // Movement inputs
        if body_actions.pressed(&GroundedHumanMovements::Forward     ) { move_intent += fwd; }
        if body_actions.pressed(&GroundedHumanMovements::Backward    ) { move_intent -= fwd; }
        if body_actions.pressed(&GroundedHumanMovements::StrafeRight ) { move_intent += rgt; }
        if body_actions.pressed(&GroundedHumanMovements::StrafeLeft  ) { move_intent -= rgt; }

        // Overall value for movement
        let move_intent = move_intent.normalize_or_zero() * 1.0;

        // Update velocity
        body_impulse.impulse.x += move_intent.x;
        body_impulse.impulse.z += move_intent.y;
    }
}