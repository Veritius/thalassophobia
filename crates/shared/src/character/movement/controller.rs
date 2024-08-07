use std::f32::consts::FRAC_PI_2;
use bevy::ecs::query::QueryData;
use bevy_rapier3d::{plugin::RapierContext, prelude::*};
use leafwing_input_manager::prelude::ActionState;
use crate::{bevy::prelude::*, disabling::Disabled, math::transform::TranslateSet, physics::*};
use super::CharacterMovements;

/// The furthest downward the controller can turn.
/// Prevents the camera from doing frontflips.
const CONTROLLER_PITCH_MIN: f32 = -FRAC_PI_2;

/// The furthest upward the controller can turn.
/// Prevents the camera from doing backflips.
const CONTROLLER_PITCH_MAX: f32 = FRAC_PI_2;

/// The state of a player controller.
#[derive(Debug, Component, Reflect)]
pub enum PlayerControllerState {
    Grounded,
    Floating,
}

/// A player character controller.
#[derive(Debug, Component, Reflect)]
pub struct PlayerController {
    /// The head entity.
    /// 
    /// This may be the same entity that the `PlayerController` component is attached to.
    pub head: Entity,

    /// The state of the player controller.
    pub state: PlayerControllerState,

    /// Current rotation (yaw).
    pub rotation_yaw: f32,

    /// Base ground movement speed.
    pub base_walk_force: Vec2,
    /// Coefficient applied to walk speed while sprinting.
    pub walk_sprint_coefficient: Vec2,
    /// Coefficient applied to walk speed while crouching.
    pub walk_crouch_coefficient: Vec2,

    /// Base swim speed.
    pub base_swim_force: TranslateSet<f32>,
    /// Coefficient applied to swim speed while sprinting.
    pub swim_sprint_coefficient: TranslateSet<f32>,

    /// The length of the raycast used to check if the controller is touching the ground.
    pub ground_raycast_len: f32,
    /// Set to `true` when touching an object that it can collide with.
    pub is_touching_ground: bool,
}

/// The head entity of a [`PlayerController`].
#[derive(Debug, Component, Reflect)]
pub struct PlayerControllerHead {
    /// Current rotation (pitch).
    pub rotation_pitch: f32,
}

pub(super) fn controller_grounding_system(
    rapier_context: Res<RapierContext>,
    // We can't filter out entities that haven't changed positions, since we can't account for other entities changing position.
    mut controllers: Query<(Entity, &mut PlayerController, &GlobalTransform, Option<&CollisionGroups>), Without<Disabled>>,
) {
    for (entity, mut controller, transform, groups) in controllers.iter_mut() {
        // Get membership data from a component if present, otherwise use a default
        let group = if let Some(groups) = groups {
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
            transform.translation(),
            Vec3::NEG_Y,
            controller.ground_raycast_len,
            false,
            QueryFilter::default().exclude_collider(entity).groups(group),
        );

        // Apply the raycast result to the controller
        match raycast {
            Some(_) => { controller.is_touching_ground = true; },
            None => { controller.is_touching_ground = false; },
        }
    }
}

#[derive(QueryData)]
#[query_data(mutable)]
struct ControllerSharedQueryData<'w> {
    transform: &'w mut Transform,
}

#[derive(QueryData)]
#[query_data(mutable)]
struct ControllerRootQueryData<'w> {
    entity: Entity,
    body_controller: &'w mut PlayerController,
    action_state: &'w ActionState<CharacterMovements>,

    impulse: Option<&'w mut ExternalImpulse>,
}

#[derive(QueryData)]
#[query_data(mutable)]
struct ControllerHeadQueryData<'w> {
    head_controller: &'w mut PlayerControllerHead,
}

type RootAndOrHead = Or<(
    With<PlayerController>,
    With<PlayerControllerHead>,
)>;

#[allow(private_interfaces)]
pub(super) fn controller_movement_system(
    mut shared: Query<ControllerSharedQueryData, RootAndOrHead>,
    mut roots: Query<ControllerRootQueryData>,
    mut heads: Query<ControllerHeadQueryData>,
) {
    for mut root in roots.iter_mut() {
        let actions = root.action_state;

        // Handle rotation first, so that forces are applied correctly
        {
            // Store whether or not the root is the head, we use this later
            let root_is_head = root.entity == root.body_controller.head;

            // Intent vector to sum up inputs
            let mut rotate_intent = Vec2::ZERO;

            // Keyboard rotation inputs
            if actions.pressed(&CharacterMovements::LookUp   ) { rotate_intent.y -= 1.0; }
            if actions.pressed(&CharacterMovements::LookDown ) { rotate_intent.y += 1.0; }
            if actions.pressed(&CharacterMovements::LookLeft ) { rotate_intent.x -= 1.0; }
            if actions.pressed(&CharacterMovements::LookRight) { rotate_intent.x += 1.0; }

            // Controller/mouse rotation inputs
            if let Some(axis_pair) = actions.axis_pair(&CharacterMovements::LookAxis) {
                rotate_intent += axis_pair.xy();
            };

            // Get the head components
            let mut head = match heads.get_mut(root.body_controller.head) {
                Ok(head) => head,
                Err(_) => todo!(),
            };

            // Store the new rotation state in the controllers
            root.body_controller.rotation_yaw += rotate_intent.x;
            head.head_controller.rotation_pitch += rotate_intent.y;

            // Clamp the rotation of the head to a valid range
            // This stops the player character from bending over backwards at inhuman angles
            head.head_controller.rotation_pitch = head.head_controller.rotation_pitch.clamp(
                CONTROLLER_PITCH_MIN,
                CONTROLLER_PITCH_MAX,
            );

            let yaw_quat = Quat::from_axis_angle(
                Vec3::Y,
                -root.body_controller.rotation_yaw,
            );

            let pitch_quat = Quat::from_axis_angle(
                Vec3::X,
                -head.head_controller.rotation_pitch,
            );

            if root_is_head {
                // The body and head are the same, apply both axes of rotation
                if let Ok(mut shared) = shared.get_mut(root.entity) {
                    shared.transform.rotation = yaw_quat + pitch_quat;
                }
            } else {
                // Apply the yaw to the body independent of the head
                if let Ok(mut shared) = shared.get_mut(root.entity) {
                    shared.transform.rotation = yaw_quat;
                }

                // Apply the pitch to the head independent of the body
                if let Ok(mut shared) = shared.get_mut(root.body_controller.head) {
                    shared.transform.rotation = pitch_quat;
                }
            }
        }

        // Handle horizontal movement inputs
        {
            // Intent vectors to sum up inputs
            let mut horizontal_intent = Vec2::default();
            let mut vertical_intent = 0.0;

            // TODO: Make sprinting and crouching toggleable
            let sprinting = actions.pressed(&CharacterMovements::Sprint);

            // Keyboard movement inputs
            if actions.pressed(&CharacterMovements::Forward     ) { horizontal_intent.x -= 1.0; }
            if actions.pressed(&CharacterMovements::Backward    ) { horizontal_intent.x += 1.0; }
            if actions.pressed(&CharacterMovements::StrafeRight ) { horizontal_intent.y += 1.0; }
            if actions.pressed(&CharacterMovements::StrafeLeft  ) { horizontal_intent.y -= 1.0; }
            if actions.pressed(&CharacterMovements::Ascend      ) { vertical_intent += 1.0;     }
            if actions.pressed(&CharacterMovements::Descend     ) { vertical_intent -= 1.0;     }

            // Controller movement inputs
            if let Some(axis_pair) = actions.axis_pair(&CharacterMovements::MoveAxis) {
                horizontal_intent += axis_pair.xy();
            }

            // Get the entity's translation value
            let shared = match shared.get(root.entity) {
                Ok(shared) => shared,
                Err(_) => todo!(),
            };

            match root.body_controller.state {
                PlayerControllerState::Grounded => {
                    
                },

                PlayerControllerState::Floating => {
                    // Put the two intents together to get the movement direction
                    let mut move_direction = horizontal_intent.normalize_or_zero().extend(0.0) * shared.transform.forward().as_vec3();
                    move_direction.y = (move_direction.y + vertical_intent).clamp(-1.0, 1.0);

                    // Movement speed coefficient
                    let coefficient = match sprinting {
                        true => root.body_controller.swim_sprint_coefficient,
                        false => TranslateSet::splat(1.0), // no effect
                    };

                    // Calculate the force for the movement
                    let move_force = move_direction * root.body_controller.base_swim_force * coefficient;

                    // Apply the physics impulse
                    if let Some(mut impulse) = root.impulse {
                        impulse.impulse += move_force;
                    }
                },
            }
        }
    }
}