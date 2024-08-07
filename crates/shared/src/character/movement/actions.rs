use crate::bevy::prelude::*;
use crate::input::Actionlike;

/// Movements that can be made by a player character.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum CharacterMovements {
    /// Axis input for pitching and yawing.
    LookAxis,

    /// Button input to pitch up.
    LookUp,

    /// Button input to pitch down.
    LookDown,

    /// Button input to turn left.
    LookLeft,

    /// Button input to turn right.
    LookRight,

    /// Axis input for horizontal movement (X and Z)
    MoveAxis,

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

    /// Button input to:
    /// - Jump if standing
    /// - Swim up if floating
    Ascend,

    /// Button input to:
    /// - Crouch if standing
    /// - Swim down if floating
    Descend,

    /// Button input to vault, attempting to climb nearby objects.
    /// 
    /// If `Vault` is available and bound to the same value as `Jump`,
    /// `Vault` takes precedence and the `Jump` input is ignored.
    Vault,

    /// Button input (toggle or hold) to move faster.
    Sprint,
}