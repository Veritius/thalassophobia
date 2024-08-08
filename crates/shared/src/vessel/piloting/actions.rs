use bevy::prelude::*;
use leafwing_input_manager::Actionlike;

/// Movements that can be made to pilot a vessel.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum VesselMovements {
    /// Single-axis input for forward engine thrust.
    ForwardThrust,

    /// Single-axis input for sideways engine thrust.
    SideThrust,

    /// Single-axis input for vertical movement.
    VerticalThrust,

    /// Single-axis input for pitching.
    Pitch,

    /// Single-axis input for yawing.
    Yaw,

    /// Single-axis input for rolling.
    Roll,

    /// Button input for halting all translation and rotation.
    Brake,

    /// Button input for changing the [`VesselMoveStyle`].
    ChangeStyle,
}