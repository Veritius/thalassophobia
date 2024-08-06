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
            Vec3::NEG_Y,
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

pub(super) fn controller_rotation_system(
    mut bodies: Query<(&mut PlayerController, &mut Transform, &ActionState<RotationMovements>), (Without<PlayerControllerHead>, Without<Disabled>)>,
    mut heads: Query<&mut Transform, (With<PlayerControllerHead>, Without<PlayerController>)>,
) {
    for (mut body_data, mut body_transform, body_actions) in bodies.iter_mut() {
        let mut turn_intent = Vec2::ZERO;

        // Try to read any rotation inputs
        if let Some(axis_pair) = body_actions.axis_pair(&RotationMovements::Axis) {
            turn_intent += axis_pair.xy();
        };

        // Keyboard rotation inputs
        if body_actions.pressed(&RotationMovements::Up   ) { turn_intent.y -= 1.0; }
        if body_actions.pressed(&RotationMovements::Down ) { turn_intent.y += 1.0; }
        if body_actions.pressed(&RotationMovements::Left ) { turn_intent.x -= 4.0; }
        if body_actions.pressed(&RotationMovements::Right) { turn_intent.x += 4.0; }

        // Make turn intent a lot easier to use for math
        let turn_intent = turn_intent * Vec2::new(0.002, 0.003);

        // Update the body's rotation
        body_data.rotation_yaw += turn_intent.x;
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
        body_data.rotation_pitch += turn_intent.y;
        body_data.rotation_pitch = body_data.rotation_pitch.clamp(CONTROLLER_PITCH_MIN, CONTROLLER_PITCH_MAX);
        head_transform.rotation = body_data.pitch_quat();
    }
}

pub(super) fn grounded_movement_system(
    mut bodies: Query<(&PlayerController, &Transform, &mut ExternalImpulse, &ActionState<GroundedMovements>), Without<Disabled>>,
) {
    for (controller, body_transform, mut body_impulse, body_actions) in bodies.iter_mut() {
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
        if let Some(axis_pair) = body_actions.axis_pair(&GroundedMovements::Axis) {
            let vect = axis_pair.xy() * fwd;
            move_intent += vect;
        }

        // Get movement speed value
        let speed_mult = match body_actions.pressed(&GroundedMovements::Sprint) {
            false => controller.base_walk_speed,
            true => controller.sprint_walk_speed,
        };

        // Overall value for movement
        let move_intent = move_intent.normalize_or_zero() * speed_mult;

        // Update velocity for movement vector
        body_impulse.impulse.x += move_intent.x;
        body_impulse.impulse.z += move_intent.y;

        // Jump if need be
        if body_actions.just_pressed(&GroundedMovements::Jump) && controller.is_touching_ground {
            body_impulse.impulse.y += controller.jump_impulse;
        }
    }
}

pub(super) fn floating_movement_system(
    mut bodies: Query<(&PlayerController, &Transform, &mut ExternalImpulse, &ActionState<SwimmingMovements>), Without<Disabled>>,
) {
    for (controller, body_transform, mut body_impulse, body_actions) in bodies.iter_mut() {
        let mut move_intent = Vec3::ZERO;

        let up = *body_transform.up();
        let fwd = *body_transform.forward();
        let rgt = *body_transform.right();

        // Keyboard movement inputs
        if body_actions.pressed(&SwimmingMovements::Ascend      ) { move_intent += up;  }
        if body_actions.pressed(&SwimmingMovements::Descend     ) { move_intent -= up;  }
        if body_actions.pressed(&SwimmingMovements::Forward     ) { move_intent += fwd; }
        if body_actions.pressed(&SwimmingMovements::Backward    ) { move_intent -= fwd; }
        if body_actions.pressed(&SwimmingMovements::StrafeRight ) { move_intent += rgt; }
        if body_actions.pressed(&SwimmingMovements::StrafeLeft  ) { move_intent -= rgt; }

        // Controller movement inputs
        if let Some(axis_pair) = body_actions.axis_pair(&SwimmingMovements::Axis) {
            let vect = Vec3::new(
                axis_pair.x() * fwd.x,
                axis_pair.y() * fwd.y,
                0.0,
            );
            move_intent += vect;
        }

        // Get movement speed value
        let speed_mult = match body_actions.pressed(&SwimmingMovements::Sprint) {
            false => controller.base_swim_speed,
            true => controller.sprint_swim_speed,
        };

        // Overall value for movement
        let move_intent = move_intent.normalize_or_zero() * speed_mult;

        // Apply movement impulse
        body_impulse.impulse += move_intent;
    }
}