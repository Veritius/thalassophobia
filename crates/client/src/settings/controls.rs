use shared::bevy::prelude::*;
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

impl Default for ControlSettings<FloatingMovements> {
    fn default() -> Self {
        let mut map = InputMap::default();

        map.insert(FloatingMovements::Axis, DualAxis::left_stick());

        map.insert(FloatingMovements::Forward, KeyCode::KeyW);
        map.insert(FloatingMovements::Backward, KeyCode::KeyS);

        map.insert(FloatingMovements::StrafeLeft, KeyCode::KeyA);
        map.insert(FloatingMovements::StrafeRight, KeyCode::KeyD);

        map.insert(FloatingMovements::Ascend, KeyCode::Space);
        map.insert(FloatingMovements::Ascend, GamepadButtonType::West);

        map.insert(FloatingMovements::Descend, KeyCode::ControlLeft);
        map.insert(FloatingMovements::Descend, GamepadButtonType::South);

        map.insert(FloatingMovements::Sprint, KeyCode::ShiftLeft);
        map.insert(FloatingMovements::Sprint, GamepadButtonType::LeftThumb);

        return Self(map);
    }
}