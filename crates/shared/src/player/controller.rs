use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerController {
    pub rotation_yaw: f32,

    pub head_entity: Option<Entity>,
}

#[derive(Component)]
pub struct PlayerControllerCamera {
    pub rotation_pitch: f32,
}