use bevy::prelude::*;
use crate::input::prelude::*;

/// Movements that can be made to turn around.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum RotationMovements {
    Axis,
    Up,
    Down,
    Left,
    Right,
}

/// Movements that can be made when standing on the ground.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum GroundedMovements {
    Axis,
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
    LeanLeft,
    LeanRight,
    Jump,
    Vault,
    Crouch,
    Sprint,
}

/// Movements that can be made when floating in water.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum FloatingMovements {
    Axis,
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
    Ascend,
    Descend,
    Sprint,
}