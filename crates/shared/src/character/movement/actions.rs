use leafwing_input_manager::InputControlKind;

use crate::bevy::prelude::*;
use crate::input::Actionlike;

/// Movements that can be made by a player character.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
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

impl Actionlike for CharacterMovements {
    fn input_control_kind(&self) -> leafwing_input_manager::InputControlKind {
        match self {
            CharacterMovements::MoveHorizontally => InputControlKind::Axis,
            CharacterMovements::MoveVertically => InputControlKind::Axis,
            CharacterMovements::Turn => InputControlKind::Axis,
            CharacterMovements::Lean => InputControlKind::Axis,
            CharacterMovements::Vault => InputControlKind::Button,
            CharacterMovements::Sprint => InputControlKind::Button,
        }
    }
}