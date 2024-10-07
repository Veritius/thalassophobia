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

        // Horizontal keyboard movement
        map.insert_dual_axis(CharacterMovements::MoveHorizontally, KeyboardVirtualDPad::new(
            KeyCode::KeyW,
            KeyCode::KeyS,
            KeyCode::KeyA,
            KeyCode::KeyD,
        ).with_circle_bounds(1.0));

        // Horizontal controller movement
        map.insert_dual_axis(CharacterMovements::MoveHorizontally, GamepadStick::LEFT);

        // Vertical keyboard movement
        map.insert_axis(CharacterMovements::MoveVertically, KeyboardVirtualAxis::new(
            KeyCode::ControlLeft,
            KeyCode::Space,
        ));

        // Vertical controller movement
        map.insert_axis(CharacterMovements::MoveVertically, GamepadVirtualAxis::new(
            GamepadButtonType::East,
            GamepadButtonType::South,
        ));

        // Mouse turning
        map.insert_dual_axis(CharacterMovements::Turn, MouseMove::default());

        // Controller turning
        map.insert_dual_axis(CharacterMovements::Turn, GamepadStick::RIGHT);

        // Keyboard leaning
        map.insert_axis(CharacterMovements::Lean, KeyboardVirtualAxis::new(
            KeyCode::KeyQ,
            KeyCode::KeyE,
        ));

        // Controller leaning
        map.insert_axis(CharacterMovements::Lean, GamepadVirtualAxis::new(
            GamepadButtonType::LeftTrigger,
            GamepadButtonType::RightTrigger,
        ));

        return Self(map);
    }
}

impl Default for ControlSettings<VesselMovements> {
    fn default() -> Self {
        let mut map = InputMap::default();

        // Keyboard forward thrust
        map.insert_axis(VesselMovements::ForwardThrust, KeyboardVirtualAxis::new(
            KeyCode::KeyS,
            KeyCode::KeyW,
        ));

        // Controller forward thrust
        map.insert_axis(VesselMovements::ForwardThrust, GamepadControlAxis::LEFT_Y);

        // Keyboard sideways thrust
        map.insert_axis(VesselMovements::SideThrust, KeyboardVirtualAxis::new(
            KeyCode::KeyA,
            KeyCode::KeyD,
        ));

        // Controller sideways thrust
        map.insert_axis(VesselMovements::SideThrust, GamepadControlAxis::LEFT_Y);

        // Keyboard vertical thrust
        map.insert_axis(VesselMovements::VerticalThrust, KeyboardVirtualAxis::new(
            KeyCode::KeyQ,
            KeyCode::KeyE,
        ));

        // Controller vertical thrust
        map.insert_axis(VesselMovements::VerticalThrust, GamepadVirtualAxis::new(
            GamepadButtonType::South,
            GamepadButtonType::North,
        ));

        // Keyboard pitching
        map.insert_axis(VesselMovements::Pitch, KeyboardVirtualAxis::new(
            KeyCode::KeyI,
            KeyCode::KeyK,
        ));

        // Controller pitching
        map.insert_axis(VesselMovements::Pitch, GamepadControlAxis::RIGHT_Y);

        // Keyboard yawing
        map.insert_axis(VesselMovements::Yaw, KeyboardVirtualAxis::new(
            KeyCode::KeyJ,
            KeyCode::KeyL,
        ));

        // Controller yawing
        map.insert_axis(VesselMovements::Yaw, GamepadControlAxis::RIGHT_X);

        // Keyboard rolling
        map.insert_axis(VesselMovements::Roll, KeyboardVirtualAxis::new(
            KeyCode::KeyO,
            KeyCode::KeyU,
        ));

        // Controller rolling
        map.insert_axis(VesselMovements::Roll, GamepadVirtualAxis::new(
            GamepadButtonType::LeftTrigger2,
            GamepadButtonType::RightTrigger2,
        ));

        map.insert(VesselMovements::Brake, KeyCode::Space);
        map.insert(VesselMovements::Brake, GamepadButtonType::South);

        return Self(map);
    }
}