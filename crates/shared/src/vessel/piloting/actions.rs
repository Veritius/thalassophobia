use leafwing_input_manager::InputControlKind;

use crate::bevy::prelude::*;
use crate::input::Actionlike;

/// Movements that can be made to pilot a vessel.
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
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
}

impl Actionlike for VesselMovements {
    fn input_control_kind(&self) -> InputControlKind {
        match self {
            VesselMovements::ForwardThrust => InputControlKind::Axis,
            VesselMovements::SideThrust => InputControlKind::Axis,
            VesselMovements::VerticalThrust => InputControlKind::Axis,
            VesselMovements::Pitch => InputControlKind::Axis,
            VesselMovements::Yaw => InputControlKind::Axis,
            VesselMovements::Roll => InputControlKind::Axis,
            VesselMovements::Brake => InputControlKind::Button,
        }
    }
}