use bevy::prelude::*;
use leafwing_input_manager::Actionlike;

/// Movements that can be made to pilot a vessel.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum VesselMovements {
    /// Axis movement for forward and sideways movement.
    FwdSide,

    /// Button input for translating upwards.
    MoveUp,

    /// Button input for translating downwards.
    MoveDown,

    /// Button input for translating left.
    MoveLeft,

    /// Button input for translating right.
    MoveRight,

    /// Button input for translating forward.
    MoveFwd,

    /// Button input for translating backward.
    MoveBack,

    /// Button input for halting all translation.
    Brake,

    /// Axis input for pitch and yaw rotation.
    PitchYaw,

    /// Axis input for pitching upwards.
    PitchUp,

    /// Axis input for pitching downwards.
    PitchDown,

    /// Axis input for yawing to the left.
    YawLeft,

    /// Axis input for yawing to the right.
    YawRight,

    /// Axis input for rolling to the left.
    RollLeft,

    /// Axis input for rolling to the right.
    RollRight,
}