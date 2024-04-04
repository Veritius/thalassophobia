use shared::bevy::prelude::*;
use shared::bevy_ecs;
use shared::input::prelude::*;
use shared::player::movement::*;

#[derive(Resource)]
pub struct ControlSettings<T: Actionlike>(pub InputMap<T>);

impl Default for ControlSettings<GroundedHumanMovements> {
    fn default() -> Self {
        let mut map = InputMap::default();

        map.insert(GroundedHumanMovements::MoveAxis, DualAxis::left_stick());

        map.insert(GroundedHumanMovements::Forward, KeyCode::KeyW);
        map.insert(GroundedHumanMovements::Backward, KeyCode::KeyS);

        map.insert(GroundedHumanMovements::StrafeLeft, KeyCode::KeyA);
        map.insert(GroundedHumanMovements::StrafeRight, KeyCode::KeyD);

        map.insert(GroundedHumanMovements::LeanLeft, KeyCode::KeyQ);
        map.insert(GroundedHumanMovements::LeanLeft, GamepadButtonType::LeftTrigger);
        map.insert(GroundedHumanMovements::LeanRight, KeyCode::KeyE);
        map.insert(GroundedHumanMovements::LeanRight, GamepadButtonType::RightTrigger);

        map.insert(GroundedHumanMovements::Jump, KeyCode::Space);
        map.insert(GroundedHumanMovements::Jump, GamepadButtonType::West);

        map.insert(GroundedHumanMovements::Crouch, KeyCode::ControlLeft);
        map.insert(GroundedHumanMovements::Crouch, GamepadButtonType::South);

        map.insert(GroundedHumanMovements::Sprint, KeyCode::ShiftLeft);
        map.insert(GroundedHumanMovements::Sprint, GamepadButtonType::LeftThumb);

        map.insert(GroundedHumanMovements::Turn, DualAxis::mouse_motion());
        map.insert(GroundedHumanMovements::Turn, DualAxis::right_stick());

        return Self(map);
    }
}

impl Default for ControlSettings<FloatingHumanMovements> {
    fn default() -> Self {
        let mut map = InputMap::default();

        map.insert(FloatingHumanMovements::MoveAxis, DualAxis::left_stick());

        map.insert(FloatingHumanMovements::Forward, KeyCode::KeyW);
        map.insert(FloatingHumanMovements::Backward, KeyCode::KeyS);

        map.insert(FloatingHumanMovements::StrafeLeft, KeyCode::KeyA);
        map.insert(FloatingHumanMovements::StrafeRight, KeyCode::KeyD);

        map.insert(FloatingHumanMovements::Ascend, KeyCode::Space);
        map.insert(FloatingHumanMovements::Ascend, GamepadButtonType::West);

        map.insert(FloatingHumanMovements::Descend, KeyCode::ControlLeft);
        map.insert(FloatingHumanMovements::Descend, GamepadButtonType::South);

        map.insert(FloatingHumanMovements::Sprint, KeyCode::ShiftLeft);
        map.insert(FloatingHumanMovements::Sprint, GamepadButtonType::LeftThumb);

        map.insert(FloatingHumanMovements::Turn, DualAxis::mouse_motion());
        map.insert(FloatingHumanMovements::Turn, DualAxis::right_stick());

        return Self(map);
    }
}