use bevy::prelude::*;
use crate::input::prelude::*;

/// Movements that can be made when standing on the ground.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum GroundedMovements {
    MoveAxis,
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
    Turn,
}

/// Movements that can be made when floating in water.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum FloatingMovements {
    MoveAxis,
    Forward,
    Backward,
    StrafeLeft,
    StrafeRight,
    Ascend,
    Descend,
    Sprint,
    Turn,
}