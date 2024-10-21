use std::{f32::consts::FRAC_PI_2, time::{Duration, Instant}};
use bevy::{prelude::*, ecs::query::QueryData};
use leafwing_input_manager::plugin::InputManagerPlugin;
use crate::math::transform::AxisSet3D;
use super::CharacterMovements;

/// The furthest downward the controller can turn.
/// Prevents the camera from doing frontflips.
const CONTROLLER_PITCH_MIN: f32 = -FRAC_PI_2;

/// The furthest upward the controller can turn.
/// Prevents the camera from doing backflips.
const CONTROLLER_PITCH_MAX: f32 = FRAC_PI_2;

pub(crate) struct PlayerControllerPlugin {
    pub mode: SetupMode,
}

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        match self.mode {
            SetupMode::Full => {
                app.add_plugins(InputManagerPlugin::<CharacterMovements>::default());
            },
            SetupMode::Headless => {
                app.add_plugins(InputManagerPlugin::<CharacterMovements>::server());
            },
        }

        app.register_type::<CharacterMovements>();
        app.register_type::<PlayerController>();
        app.register_type::<PlayerControllerHead>();
        app.register_type::<PlayerControllerState>();

        app.add_systems(SimulationUpdate, (
            character_ground_system,
            character_controller_system,
        ).chain());
    }
}

/// The state of a player controller.
#[derive(Debug, Clone, Copy, Component, PartialEq, Eq, Reflect)]
pub enum PlayerControllerState {
    Grounded,
    Floating,
}

/// A player character controller.
#[derive(Debug, Clone, Component, Reflect)]
#[reflect(from_reflect = false, Component)]
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
    pub base_swim_force: AxisSet3D<f32>,
    /// Coefficient applied to swim speed while sprinting.
    pub swim_sprint_coefficient: AxisSet3D<f32>,

    /// If jumping is allowed.
    pub allow_jumping: bool,
    /// The force applied when jumping.
    pub jump_impulse: f32,
    /// The delay between jumps.
    pub jump_delay: Duration,
    /// The last time the character jumped.
    pub last_jumped: Option<Instant>,

    /// The length of the raycast used to check if the controller is touching the ground.
    pub ground_raycast_len: f32,
    /// Set to `true` when touching an object that it can collide with.
    pub is_touching_ground: bool,
}

impl PlayerController {
    pub fn new(
        head: Entity,
        state: PlayerControllerState,
    ) -> Self {
        Self {
            head,

            state,

            rotation_yaw: 0.0,

            base_walk_force: Vec2::splat(1.0),
            walk_sprint_coefficient: Vec2::splat(1.5),
            walk_crouch_coefficient: Vec2::splat(0.8),

            base_swim_force: AxisSet3D::splat(0.6),
            swim_sprint_coefficient: AxisSet3D::splat(1.3),

            allow_jumping: true,
            jump_impulse: 20.0,
            jump_delay: Duration::from_millis(200),
            last_jumped: None,

            ground_raycast_len: 1.0,
            is_touching_ground: false,
        }
    }

    pub fn reset_jump_timer(&mut self) {
        self.last_jumped = Some(Instant::now());
    }
}

/// The head entity of a [`PlayerController`].
#[derive(Debug, Clone, Component, Reflect)]
#[reflect(from_reflect = false, Component)]
pub struct PlayerControllerHead {
    /// Current rotation (pitch).
    pub rotation_pitch: f32,
}

impl Default for PlayerControllerHead {
    fn default() -> Self {
        Self {
            rotation_pitch: 0.0,
        }
    }
}

pub(super) fn character_ground_system(
    // rapier_context: Res<RapierContext>,
    // // We can't filter out entities that haven't changed positions, since we can't account for other entities changing position.
    // mut controllers: Query<(Entity, &mut PlayerController, &GlobalTransform, Option<&CollisionGroups>), Without<Disabled>>,
) {
    // for (entity, mut controller, transform, groups) in controllers.iter_mut() {
    //     // Get membership data from a component if present, otherwise use a default
    //     let group = if let Some(groups) = groups {
    //         groups.clone()
    //     } else {
    //         // Default collision group
    //         CollisionGroups {
    //             memberships: PHYS_GROUP_CHARACTER,
    //             filters: PHYS_GROUP_TERRAIN | PHYS_GROUP_STRUCTURE
    //         }
    //     };

    //     // Cast a ray to see if there's anything below us to jump off of
    //     let raycast = rapier_context.cast_ray(
    //         transform.translation(),
    //         Vec3::NEG_Y,
    //         controller.ground_raycast_len,
    //         false,
    //         QueryFilter::default().exclude_collider(entity).groups(group),
    //     );

    //     // Apply the raycast result to the controller
    //     match raycast {
    //         Some(_) => { controller.is_touching_ground = true; },
    //         None => { controller.is_touching_ground = false; },
    //     }
    // }
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

fn character_controller_system(
    mut shared: Query<ControllerSharedQueryData, RootAndOrHead>,
    mut roots: Query<ControllerRootQueryData, Without<Disabled>>,
    mut heads: Query<ControllerHeadQueryData>,
) {
    for mut root in roots.iter_mut() {
        let actions = root.action_state;

        // Handle rotation first, so that forces are applied correctly
        'rotation: {
            // Store whether or not the root is the head, we use this later
            let root_is_head = root.entity == root.body_controller.head;

            // Intent vector to sum up inputs
            let rotate_intent = actions.axis_pair(&CharacterMovements::Turn) * Vec2::splat(0.01);
            if rotate_intent == Vec2::splat(0.0) { break 'rotation } // Don't waste our time

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
        'position: {
            // Base horizontal intent constructed from movement values
            let horizontal_inputs = actions.clamped_axis_pair(&CharacterMovements::MoveHorizontally);

            // Constrain the magnitude to 1.0 to prevent a movement exploit
            // that gives greater horizontal speed when moving diagonally
            let input_magnitude = horizontal_inputs.length().min(1.0);

            // Calculate the angle the player is trying to move in
            let input_angle = f32::atan2(
                horizontal_inputs.y * -1.0, // Forward is -Z in Bevy
                horizontal_inputs.x,
            );

            // Calculate overall direction of movement, this lets us
            let intent_angle = input_angle + root.body_controller.rotation_yaw;

            // Create the horizontal intent vector from the angle and magnitude
            let horizontal_intent = Vec2 {
                x: intent_angle.cos() * input_magnitude,
                y: intent_angle.sin() * input_magnitude,
            };

            // Value is automatically brought within bounds by Leafwing
            let vertical_intent = actions.clamped_value(&CharacterMovements::MoveVertically);

            // If there's no input, early return to avoid wasting our time
            if horizontal_intent == Vec2::ZERO && vertical_intent == 0.0 { break 'position; }

            // TODO: Make sprinting and crouching toggleable
            let sprinting = actions.pressed(&CharacterMovements::Sprint);

            match root.body_controller.state {
                PlayerControllerState::Grounded => {
                    // Basic movement force calculation
                    let mut move_force = horizontal_intent * root.body_controller.base_walk_force;

                    // Apply additional force for sprinting
                    move_force *= match sprinting {
                        true => root.body_controller.walk_sprint_coefficient,
                        false => Vec2::splat(1.0),
                    };

                    // Extend the move force to 3D
                    let mut move_impulse = move_force.extend(0.0).xzy();

                    // Handle jumping calculations
                    'jump: {
                        // Character must be allowed to jump and be touching the ground
                        if !root.body_controller.allow_jumping { break 'jump }
                        if !root.body_controller.is_touching_ground { break 'jump }

                        // The player must want to jump
                        if vertical_intent < 0.5 { break 'position }

                        // The character can't jump until the delay is passed
                        if let Some(last_jumped) = root.body_controller.last_jumped {
                            if last_jumped.elapsed() < root.body_controller.jump_delay { break 'jump }
                        }

                        // Reset the jump timer
                        root.body_controller.reset_jump_timer();

                        // Add the movement force to the vector
                        move_impulse.y += root.body_controller.jump_impulse;
                    }

                    // Apply the physics impulse
                    if let Some(mut impulse) = root.impulse {
                        impulse.apply_impulse(move_impulse);
                    }
                },

                PlayerControllerState::Floating => {
                    // Movement speed coefficient
                    let coefficient = match sprinting {
                        true => root.body_controller.swim_sprint_coefficient,
                        false => AxisSet3D::splat(1.0), // no effect
                    };

                    // Calculate the force for the movement
                    let move_impulse = horizontal_intent.extend(vertical_intent).xzy() * root.body_controller.base_swim_force * coefficient;

                    // Apply the physics impulse
                    if let Some(mut impulse) = root.impulse {
                        impulse.apply_impulse(move_impulse);
                    }
                },
            }
        }
    }
}