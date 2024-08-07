use crate::{bevy::prelude::*, math::transform::TranslateSet};

/// A player character controller.
#[derive(Debug, Component, Reflect)]
pub struct PlayerController {
    /// The head entity.
    /// 
    /// This may be the same entity that the `PlayerController` component is attached to.
    pub head: Entity,

    /// Current rotation (yaw).
    pub rotation_yaw: f32,

    /// Base ground movement speed.
    pub base_walk_speed: TranslateSet<f32>,
    /// Coefficient applied to walk speed while sprinting.
    pub walk_sprint_coefficient: TranslateSet<f32>,

    /// Base swim speed.
    pub base_swim_speed: TranslateSet<f32>,
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