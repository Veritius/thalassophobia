use bevy::prelude::*;
use leafwing_input_manager::Actionlike;

/// The 'style' of movement for vessels.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum VesselMoveStyle {
    /// The vessel will maintain speed and course.
    Maintain,

    /// The vessel will not attempt to correct any movements.
    Manual,
}

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

    /// Axis input for pitch and yaw rotation.
    PitchYaw,

    /// Button input for pitching upwards.
    PitchUp,

    /// Button input for pitching downwards.
    PitchDown,

    /// Button input for yawing to the left.
    YawLeft,

    /// Button input for yawing to the right.
    YawRight,

    /// Button input for rolling to the left.
    RollLeft,

    /// Button input for rolling to the right.
    RollRight,

    /// Button input for halting all translation and rotation.
    Brake,

    /// Button input for changing the [`VesselMoveStyle`].
    ChangeStyle,
}