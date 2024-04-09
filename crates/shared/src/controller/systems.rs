use bevy::prelude::*;
use bevy_rapier3d::{dynamics::ExternalImpulse, geometry::CollisionGroups, pipeline::QueryFilter, plugin::RapierContext};
use leafwing_input_manager::prelude::*;
use crate::{disabling::Disabled, physics::{PHYS_GROUP_CHARACTER, PHYS_GROUP_STRUCTURE, PHYS_GROUP_TERRAIN}};

use super::*;

pub(super) fn touching_ground_system(
    rapier_context: Res<RapierContext>,
    // We can't filter out entities that haven't changed positions, since we can't account for other entities changing position.
    mut bodies: Query<(Entity, &mut PlayerController, &GlobalTransform, Option<&CollisionGroups>), Without<Disabled>>,
) {
    for (body_entity, mut body_data, global_transform, collision_groups) in bodies.iter_mut() {
        // Get membership data from a component if present, otherwise use a default
        let group = if let Some(groups) = collision_groups {
            groups.clone()
        } else {
            // Default collision group
            CollisionGroups {
                memberships: PHYS_GROUP_CHARACTER,
                filters: PHYS_GROUP_TERRAIN | PHYS_GROUP_STRUCTURE
            }
        };

        // Cast a ray to see if there's anything below us to jump off of
        let raycast = rapier_context.cast_ray(
            global_transform.translation(),
            -Vec3::Y,
            body_data.ground_raycast_len,
            false,
            QueryFilter::default().exclude_collider(body_entity).groups(group),
        );

        // Apply the raycast result to the controller
        match raycast {
            Some(_) => { body_data.is_touching_ground = true; },
            None => { body_data.is_touching_ground = false; },
        }
    }
}

pub(super) fn grounded_rotation_system(
    mut bodies: Query<(&mut PlayerController, &mut Transform, &ActionState<GroundedMovements>), (Without<PlayerControllerHead>, Without<Disabled>)>,
    mut heads: Query<&mut Transform, (With<PlayerControllerHead>, Without<PlayerController>)>,
) {
    for (mut body_data, mut body_transform, body_actions) in bodies.iter_mut() {
        // Try to read any rotation inputs
        let axis_input = match body_actions.axis_pair(&GroundedMovements::Turn) {
            Some(val) => {
                let mut vx = val.xy();
                vx.x *= 0.0030; // left and right
                vx.y *= 0.0020; // up and down
                vx
            },
            None => { continue },
        };

        // Update the body's rotation
        body_data.rotation_yaw += axis_input.x;
        body_transform.rotation = body_data.yaw_quat();

        // Get the head component
        let mut head_transform = match heads.get_mut(match body_data.head_entity {
            Some(v) => v,
            None => { continue },
        }) {
            Ok(v) => v,
            Err(_) => { continue },
        };

        // Update the head's rotation
        body_data.rotation_pitch += axis_input.y;
        body_data.rotation_pitch = body_data.rotation_pitch.clamp(CONTROLLER_PITCH_MIN, CONTROLLER_PITCH_MAX);
        head_transform.rotation = body_data.pitch_quat();
    }
}

pub(super) fn grounded_movement_system(
    mut bodies: Query<(&PlayerController, &Transform, &mut ExternalImpulse, &ActionState<GroundedMovements>), Without<Disabled>>,
) {
    for (&ref body_controller, &body_transform, mut body_impulse, body_actions) in bodies.iter_mut() {
        let mut move_intent = Vec2::ZERO;

        let lz = body_transform.local_z();
        let fwd = -Vec2::new(lz.x, lz.z);
        let rgt = Vec2::new(lz.z, -lz.x);

        // Keyboard movement inputs
        if body_actions.pressed(&GroundedMovements::Forward     ) { move_intent += fwd; }
        if body_actions.pressed(&GroundedMovements::Backward    ) { move_intent -= fwd; }
        if body_actions.pressed(&GroundedMovements::StrafeRight ) { move_intent += rgt; }
        if body_actions.pressed(&GroundedMovements::StrafeLeft  ) { move_intent -= rgt; }

        // Controller movement inputs
        if let Some(axis_pair) = body_actions.axis_pair(&GroundedMovements::MoveAxis) {
            let vect = axis_pair.xy() * fwd;
            move_intent += vect;
        }

        let speed_mult = match body_actions.pressed(&GroundedMovements::Sprint) {
            false => body_controller.walk_speed_mod,
            true => body_controller.sprint_speed_mod,
        };

        // Overall value for movement
        let move_intent = move_intent.normalize_or_zero() * speed_mult;

        // Update velocity for movement vector
        body_impulse.impulse.x += move_intent.x;
        body_impulse.impulse.z += move_intent.y;

        // Jump if need be
        if body_actions.just_pressed(&GroundedMovements::Jump) && body_controller.is_touching_ground {
            body_impulse.impulse.y += body_controller.jump_impulse;
        }
    }
}