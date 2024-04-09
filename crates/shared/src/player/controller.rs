use bevy::prelude::*;
use bevy_rapier3d::{dynamics::ExternalImpulse, geometry::CollisionGroups, pipeline::QueryFilter, plugin::RapierContext};
use leafwing_input_manager::prelude::*;
use crate::{disabling::Disabled, physics::{PHYS_GROUP_CHARACTER, PHYS_GROUP_STRUCTURE, PHYS_GROUP_TERRAIN}};

use super::movement::*;

const CONTROLLER_PITCH_MIN: f32 = -1.7;
const CONTROLLER_PITCH_MAX: f32 = 1.7;

#[derive(Component)]
pub struct PlayerController {
    pub walk_speed_mod: f32,
    pub sprint_speed_mod: f32,

    pub rotation_yaw: f32,

    pub raycast_len: f32,
    pub head_entity: Option<Entity>,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            walk_speed_mod: 1.0,
            sprint_speed_mod: 1.5,

            rotation_yaw: 0.0,

            raycast_len: 1.0,
            head_entity: None,
        }
    }
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
                vx.x *= 0.0030; // left and right
                vx.y *= 0.0020; // up and down
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
    rapier_context: Res<RapierContext>,
    mut bodies: Query<(
        Entity,
        &GlobalTransform,
        &Transform,
        &PlayerController,
        &mut ExternalImpulse,
        &ActionState<GroundedHumanMovements>,
        Option<&CollisionGroups>,
    ), Without<Disabled>>,
) {
    for (body_entity, &body_global_transform, &body_transform, &ref body_controller, mut body_impulse, body_actions, body_groups) in bodies.iter_mut() {
        let mut move_intent = Vec2::ZERO;

        let lz = body_transform.local_z();
        let fwd = -Vec2::new(lz.x, lz.z);
        let rgt = Vec2::new(lz.z, -lz.x);

        // Keyboard movement inputs
        if body_actions.pressed(&GroundedHumanMovements::Forward     ) { move_intent += fwd; }
        if body_actions.pressed(&GroundedHumanMovements::Backward    ) { move_intent -= fwd; }
        if body_actions.pressed(&GroundedHumanMovements::StrafeRight ) { move_intent += rgt; }
        if body_actions.pressed(&GroundedHumanMovements::StrafeLeft  ) { move_intent -= rgt; }

        // Controller movement inputs
        if let Some(axis_pair) = body_actions.axis_pair(&GroundedHumanMovements::MoveAxis) {
            let vect = axis_pair.xy() * fwd;
            move_intent += vect;
        }

        let speed_mult = match body_actions.pressed(&GroundedHumanMovements::Sprint) {
            false => body_controller.walk_speed_mod,
            true => body_controller.sprint_speed_mod,
        };

        // Overall value for movement
        let move_intent = move_intent.normalize_or_zero() * speed_mult;

        // Update velocity for movement vector
        body_impulse.impulse.x += move_intent.x;
        body_impulse.impulse.z += move_intent.y;

        // Jumping
        if body_actions.just_pressed(&GroundedHumanMovements::Jump) {
            // Get membership data from a component if present, otherwise use a default
            let group = if let Some(groups) = body_groups {
                groups.clone()
            } else {
                // Default collision group
                CollisionGroups {
                    memberships: PHYS_GROUP_CHARACTER,
                    filters: PHYS_GROUP_TERRAIN | PHYS_GROUP_STRUCTURE
                }
            };

            // Cast a ray to see if there's anything below us to jump off of
            if let Some((_, _)) = rapier_context.cast_ray(
                body_global_transform.translation(),
                -Vec3::Y,
                body_controller.raycast_len,
                false,
                QueryFilter::default().exclude_collider(body_entity).groups(group),
            ) {
                // Apply jump impulse
                body_impulse.impulse.y += 20.0;
            }
        }
    }
}