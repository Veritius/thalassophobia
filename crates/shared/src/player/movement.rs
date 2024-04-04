use bevy::prelude::*;
use crate::input::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum GroundedHumanMovements {
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

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum FloatingHumanMovements {
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