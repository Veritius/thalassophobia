use crate::bevy::prelude::*;
use crate::input::Actionlike;

/// Movements that can be made by a player character.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum CharacterMovements {
    /// Axis input for pitching and yawing.
    Turn,

    /// Axis input for horizontal movement (X and Z)
    MoveHorizontally,

    /// Button input to:
    /// - Jump if standing
    /// - Swim up if floating
    MoveVertically,

    /// Button input to lean to the left.
    Lean,

    /// Button input to vault, attempting to climb nearby objects.
    /// 
    /// If `Vault` is available and bound to the same value as `Jump`,
    /// `Vault` takes precedence and the `Jump` input is ignored.
    Vault,

    /// Button input (toggle or hold) to move faster.
    Sprint,
}