use bevy::prelude::*;
use leafwing_input_manager::Actionlike;

/// Movements that can be made to pilot a vessel.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum VesselMovements {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveFwd,
    MoveBack,

    PitchLeft,
    PitchRight,
    RollLeft,
    RollRight,
    YawLeft,
    YawRight,
}