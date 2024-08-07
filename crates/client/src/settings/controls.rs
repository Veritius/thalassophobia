use shared::bevy::prelude::*;
use shared::character::movement::CharacterMovements;
use shared::vessel::piloting::VesselMovements;
use shared::{bevy_ecs, bevy_reflect};
use shared::input::prelude::*;

#[derive(Resource, Reflect)]
pub struct ControlSettings<T: Actionlike>(pub InputMap<T>);

impl Default for ControlSettings<CharacterMovements> {
    fn default() -> Self {
        let mut map = InputMap::default();

        map.insert(CharacterMovements::MoveAxis, DualAxis::left_stick());

        map.insert(CharacterMovements::Forward, KeyCode::KeyW);
        map.insert(CharacterMovements::Backward, KeyCode::KeyS);

        map.insert(CharacterMovements::StrafeLeft, KeyCode::KeyA);
        map.insert(CharacterMovements::StrafeRight, KeyCode::KeyD);

        map.insert(CharacterMovements::LeanLeft, KeyCode::KeyQ);
        map.insert(CharacterMovements::LeanLeft, GamepadButtonType::LeftTrigger);
        map.insert(CharacterMovements::LeanRight, KeyCode::KeyE);
        map.insert(CharacterMovements::LeanRight, GamepadButtonType::RightTrigger);

        map.insert(CharacterMovements::Ascend, KeyCode::Space);
        map.insert(CharacterMovements::Ascend, GamepadButtonType::West);

        map.insert(CharacterMovements::Descend, KeyCode::ControlLeft);
        map.insert(CharacterMovements::Descend, GamepadButtonType::South);

        map.insert(CharacterMovements::Sprint, KeyCode::ShiftLeft);
        map.insert(CharacterMovements::Sprint, GamepadButtonType::LeftThumb);

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