use shared::bevy::prelude::*;
use shared::vessel::piloting::VesselMovements;
use shared::{bevy_ecs, bevy_reflect};
use shared::input::prelude::*;
use shared::controller::*;

#[derive(Resource, Reflect)]
pub struct ControlSettings<T: Actionlike>(pub InputMap<T>);

impl Default for ControlSettings<RotationMovements> {
    fn default() -> Self {
        let mut map = InputMap::default();

        map.insert(RotationMovements::Axis, DualAxis::mouse_motion());
        map.insert(RotationMovements::Axis, DualAxis::right_stick());

        return Self(map);
    }
}

impl Default for ControlSettings<GroundedMovements> {
    fn default() -> Self {
        let mut map = InputMap::default();

        map.insert(GroundedMovements::Axis, DualAxis::left_stick());

        map.insert(GroundedMovements::Forward, KeyCode::KeyW);
        map.insert(GroundedMovements::Backward, KeyCode::KeyS);

        map.insert(GroundedMovements::StrafeLeft, KeyCode::KeyA);
        map.insert(GroundedMovements::StrafeRight, KeyCode::KeyD);

        map.insert(GroundedMovements::LeanLeft, KeyCode::KeyQ);
        map.insert(GroundedMovements::LeanLeft, GamepadButtonType::LeftTrigger);
        map.insert(GroundedMovements::LeanRight, KeyCode::KeyE);
        map.insert(GroundedMovements::LeanRight, GamepadButtonType::RightTrigger);

        map.insert(GroundedMovements::Jump, KeyCode::Space);
        map.insert(GroundedMovements::Jump, GamepadButtonType::West);

        map.insert(GroundedMovements::Crouch, KeyCode::ControlLeft);
        map.insert(GroundedMovements::Crouch, GamepadButtonType::South);

        map.insert(GroundedMovements::Sprint, KeyCode::ShiftLeft);
        map.insert(GroundedMovements::Sprint, GamepadButtonType::LeftThumb);

        return Self(map);
    }
}

impl Default for ControlSettings<SwimmingMovements> {
    fn default() -> Self {
        let mut map = InputMap::default();

        map.insert(SwimmingMovements::Axis, DualAxis::left_stick());

        map.insert(SwimmingMovements::Forward, KeyCode::KeyW);
        map.insert(SwimmingMovements::Backward, KeyCode::KeyS);

        map.insert(SwimmingMovements::StrafeLeft, KeyCode::KeyA);
        map.insert(SwimmingMovements::StrafeRight, KeyCode::KeyD);

        map.insert(SwimmingMovements::Ascend, KeyCode::Space);
        map.insert(SwimmingMovements::Ascend, GamepadButtonType::West);

        map.insert(SwimmingMovements::Descend, KeyCode::ControlLeft);
        map.insert(SwimmingMovements::Descend, GamepadButtonType::South);

        map.insert(SwimmingMovements::Sprint, KeyCode::ShiftLeft);
        map.insert(SwimmingMovements::Sprint, GamepadButtonType::LeftThumb);

        return Self(map);
    }
}

impl Default for ControlSettings<VesselMovements> {
    fn default() -> Self {
        let mut map = InputMap::default();

        map.insert(VesselMovements::MoveUp, KeyCode::KeyZ);
        map.insert(VesselMovements::MoveUp, GamepadButtonType::North);
        map.insert(VesselMovements::MoveDown, KeyCode::KeyC);
        map.insert(VesselMovements::MoveDown, GamepadButtonType::West);

        map.insert(VesselMovements::MoveLeft, KeyCode::KeyA);
        map.insert(VesselMovements::MoveRight, KeyCode::KeyD);
        map.insert(VesselMovements::MoveFwd, KeyCode::KeyW);
        map.insert(VesselMovements::MoveBack, KeyCode::KeyS);

        map.insert(VesselMovements::FwdSide, DualAxis::right_stick());
        map.insert(VesselMovements::PitchYaw, DualAxis::mouse_motion());
        map.insert(VesselMovements::PitchYaw, DualAxis::left_stick());

        map.insert(VesselMovements::RollLeft, KeyCode::KeyQ);
        map.insert(VesselMovements::RollRight, KeyCode::KeyE);

        map.insert(VesselMovements::Brake, KeyCode::Space);
        map.insert(VesselMovements::Brake, GamepadButtonType::South);
        map.insert(VesselMovements::ChangeStyle, KeyCode::KeyX);
        map.insert(VesselMovements::ChangeStyle, GamepadButtonType::LeftTrigger);

        return Self(map);
    }
}