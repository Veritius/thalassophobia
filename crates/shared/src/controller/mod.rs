mod actions;
mod systems;
mod queries;

pub use actions::*;
pub use queries::*;

use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use crate::{state::simulation_running, SetupMode};

pub(crate) struct PlayerCharacterPlugin;

impl Plugin for PlayerCharacterPlugin {
    fn build(&self, app: &mut App) {
        let setup_mode = app.world.resource::<SetupMode>();

        match setup_mode {
            SetupMode::Headless => {
                app.add_plugins(InputManagerPlugin::<actions::GroundedHumanMovements>::server());
                app.add_plugins(InputManagerPlugin::<actions::FloatingHumanMovements>::server());
            },
            SetupMode::Full => {
                app.add_plugins(InputManagerPlugin::<actions::GroundedHumanMovements>::default());
                app.add_plugins(InputManagerPlugin::<actions::FloatingHumanMovements>::default());
            },
        }

        app.add_systems(Update, (
            systems::touching_ground_system,
            systems::grounded_rotation_system,
            systems::grounded_movement_system,
        ).chain()
            .run_if(simulation_running())
            .in_set(PlayerCharacterSystemSet::Controller));
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub enum PlayerCharacterSystemSet {
    Controller,
}

const PI_FRAC_2: f32 = std::f32::consts::PI / 2.0;

pub const CONTROLLER_PITCH_MIN: f32 = -PI_FRAC_2;
pub const CONTROLLER_PITCH_MAX: f32 = PI_FRAC_2;

#[derive(Debug, Component)]
pub struct PlayerController {
    /// How fast walking is.
    pub walk_speed_mod: f32,
    /// How fast sprinting is.
    pub sprint_speed_mod: f32,
    /// How much force is applied when the character jumps.
    pub jump_impulse: f32,

    /// The current left-right rotation of the controller, in radians.
    /// Overrides `Transform` and `GlobalTransform`.
    pub rotation_yaw: f32,
    /// The current up-down rotation of the controller, in radians.
    /// Overrides `Transform` and `GlobalTransform`.
    /// Constrained by [`CONTROLLER_PITCH_MIN`] and [`CONTROLLER_PITCH_MAX`].
    pub rotation_pitch: f32,

    /// The length of the raycast used to check if the controller is touching the ground.
    pub ground_raycast_len: f32,
    /// Set to `true` when touching an object that it can collide with.
    pub is_touching_ground: bool,

    /// The head entity used for pitch rotation.
    pub head_entity: Entity,
}

impl PlayerController {
    /// Returns a quaternion of the controller's yaw (left/right)
    #[inline]
    pub fn yaw_quat(&self) -> Quat {
        Quat::from_axis_angle(Vec3::Y, -self.rotation_yaw)
    }

    /// Returns a quaternion of the controller's pitch (up/down)
    #[inline]
    pub fn pitch_quat(&self) -> Quat {
        Quat::from_axis_angle(Vec3::X, -self.rotation_pitch)
    }

    /// Returns a quaternion of the controller's pitch and yaw
    #[inline]
    pub fn look_quat(&self) -> Quat {
        self.yaw_quat() * self.pitch_quat()
    }
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            walk_speed_mod: 1.0,
            sprint_speed_mod: 1.5,
            jump_impulse: 20.0,

            rotation_yaw: 0.0,
            rotation_pitch: 0.0,

            ground_raycast_len: 1.0,
            is_touching_ground: false,

            head_entity: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Debug, Component, Default)]
pub struct PlayerControllerHead;