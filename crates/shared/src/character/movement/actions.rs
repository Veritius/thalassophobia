use crate::bevy::prelude::*;
use crate::input::Actionlike;

/// Movements that can be made to turn around.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum RotationMovements {
    /// Axis input for pitching and yawing.
    Axis,

    /// Button input to pitch up.
    Up,

    /// Button input to pitch down.
    Down,

    /// Button input to turn left.
    Left,

    /// Button input to turn right.
    Right,
}

/// Movements that can be made when standing on the ground.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum GroundedMovements {
    /// Axis input for horizontal movement (X and Z)
    Axis,

    /// Button input to move forward.
    Forward,

    /// Button input to move backward.
    Backward,

    /// Button input to move to the left.
    StrafeLeft,

    /// Button input to move to the right.
    StrafeRight,

    /// Button input to lean to the left.
    LeanLeft,

    /// Button input to lean to the right.
    LeanRight,

    /// Button input to jump, giving a burst of vertical speed.
    Jump,

    /// Button input to vault, attempting to climb nearby objects.
    /// 
    /// If `Vault` is available and bound to the same value as `Jump`,
    /// `Vault` takes precedence and the `Jump` input is ignored.
    Vault,

    /// Button input to toggle crouching, or held to crouch.
    Crouch,

    /// Button input (toggle or hold) to move faster.
    Sprint,
}

/// Movements that can be made when floating in water.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum SwimmingMovements {
    /// Axis input for horizontal movement relative to the character's orientation.
    Axis,

    /// Button input to swim forwards.
    Forward,

    /// Button input to swim backwards.
    Backward,

    /// Button input to swim to the left.
    StrafeLeft,

    /// Button input to swim to the right.
    StrafeRight,

    /// Button input to swim upwards.
    Ascend,

    /// Button input to swim downwards.
    Descend,

    /// Button input (toggle or hold) to swim rapidly.
    Sprint,
}