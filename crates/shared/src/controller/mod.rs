mod actions;
mod systems;
mod queries;

pub use actions::*;
pub use queries::*;

use std::f32::consts::FRAC_PI_2;
use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use crate::{state::simulation_running, SetupMode};

pub(crate) struct PlayerCharacterPlugin;

impl Plugin for PlayerCharacterPlugin {
    fn build(&self, app: &mut App) {
        let setup_mode = app.world.resource::<SetupMode>();

        match setup_mode {
            SetupMode::Full => {
                app.add_plugins(InputManagerPlugin::<actions::RotationMovements>::default());
                app.add_plugins(InputManagerPlugin::<actions::GroundedMovements>::default());
                app.add_plugins(InputManagerPlugin::<actions::FloatingMovements>::default());
            },
            SetupMode::Headless => {
                app.add_plugins(InputManagerPlugin::<actions::RotationMovements>::server());
                app.add_plugins(InputManagerPlugin::<actions::GroundedMovements>::server());
                app.add_plugins(InputManagerPlugin::<actions::FloatingMovements>::server());
            },
        }

        app.configure_sets(Update, PlayerControllerSystemSet::Rotation
            .after(PlayerControllerSystemSet::PhysicsTests));

        app.configure_sets(Update, PlayerControllerSystemSet::Movements
            .after(PlayerControllerSystemSet::Rotation));

        app.add_systems(Update, systems::touching_ground_system
            .run_if(simulation_running())
            .in_set(PlayerControllerSystemSet::PhysicsTests));

        app.add_systems(Update, systems::controller_rotation_system
            .run_if(simulation_running())
            .in_set(PlayerControllerSystemSet::Rotation));

        app.add_systems(Update, systems::grounded_movement_system
            .run_if(simulation_running())
            .in_set(PlayerControllerSystemSet::Movements));

        app.add_systems(Update, systems::floating_movement_system
            .run_if(simulation_running())
            .in_set(PlayerControllerSystemSet::Movements));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub enum PlayerControllerSystemSet {
    PhysicsTests,
    Rotation,
    Movements,
}

/// The furthest downward the controller can turn.
/// Prevents the camera from doing frontflips.
pub const CONTROLLER_PITCH_MIN: f32 = -FRAC_PI_2;
/// The furthest upward the controller can turn.
/// Prevents the camera from doing backflips.
pub const CONTROLLER_PITCH_MAX: f32 = FRAC_PI_2;

/// A simple FPS controller for player controlled characters.
#[derive(Debug, Component)]
pub struct PlayerController {
    /// How fast walking is.
    pub base_walk_speed: f32,
    /// How fast sprinting is.
    pub sprint_walk_speed: f32,
    /// How much force is applied when the character jumps.
    pub jump_impulse: f32,

    /// How fast swimming is.
    pub base_swim_speed: f32,
    /// How fast sprinting underwater is.
    pub sprint_swim_speed: f32,

    /// The current left-right rotation of the controller, in radians.
    /// Overrides `Transform` and `GlobalTransform`.
    pub rotation_yaw: f32,
    /// The current up-down rotation of the controller, in radians.
    /// Overrides `Transform` and `GlobalTransform`.
    /// 
    /// If [`head_entity`](Self::head_entity) is `None`, this value is meaningless.
    /// Constrained by [`CONTROLLER_PITCH_MIN`] and [`CONTROLLER_PITCH_MAX`].
    pub rotation_pitch: f32,

    /// The length of the raycast used to check if the controller is touching the ground.
    pub ground_raycast_len: f32,
    /// Set to `true` when touching an object that it can collide with.
    pub is_touching_ground: bool,

    /// The head entity used for pitch rotation.
    /// If `None`, pitch will not be applied.
    pub head_entity: Option<Entity>,
}

impl PlayerController {
    /// Returns a quaternion of the controller's yaw (left/right)
    #[inline]
    pub fn yaw_quat(&self) -> Quat {
        Quat::from_axis_angle(Vec3::Y, -self.rotation_yaw)
    }

    /// Returns a quaternion of the controller's pitch (up/down)
    /// 
    /// If [`head_entity`](Self::head_entity) is `None`, the returned value is meaningless.
    #[inline]
    pub fn pitch_quat(&self) -> Quat {
        Quat::from_axis_angle(Vec3::X, -self.rotation_pitch)
    }

    /// Returns a quaternion of the controller's pitch and yaw.
    #[inline]
    pub fn look_quat(&self) -> Quat {
        self.yaw_quat() * self.pitch_quat()
    }
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            base_walk_speed: 1.0,
            sprint_walk_speed: 1.5,
            jump_impulse: 20.0,

            base_swim_speed: 0.2,
            sprint_swim_speed: 0.3,

            rotation_yaw: 0.0,
            rotation_pitch: 0.0,

            ground_raycast_len: 1.0,
            is_touching_ground: false,

            head_entity: None,
        }
    }
}

#[derive(Debug, Component, Default)]
pub struct PlayerControllerHead;