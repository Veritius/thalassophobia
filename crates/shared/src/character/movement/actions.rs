use crate::bevy::prelude::*;
use crate::input::Actionlike;

/// Movements that can be made by a player character.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum CharacterMovements {
    /// Dual-axis input for horizontal movement.
    MoveHorizontally,

    /// Single-axis input for vertical movement.
    MoveVertically,

    /// Dual-axis input for pitching and yawing.
    Turn,

    /// Single-axis input to lean to the left.
    Lean,

    /// Button input to vault, attempting to climb nearby objects.
    /// 
    /// If `Vault` is available and bound to the same value as `Jump`,
    /// `Vault` takes precedence and the `Jump` input is ignored.
    Vault,

    /// Button input (toggle or hold) to move faster.
    Sprint,
}