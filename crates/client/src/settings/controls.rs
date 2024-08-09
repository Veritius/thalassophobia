use shared::bevy::prelude::*;
use shared::character::movement::CharacterMovements;
use shared::vessel::piloting::VesselMovements;
use shared::{bevy_ecs, bevy_reflect};
use shared::input::prelude::*;

/// Mouse sensitivity configuration.
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub struct MouseSensitivity {
    /// Horizontal (left-right) sensitivity coefficient.
    pub horizontal: f32,

    /// Vertical (up-down) sensitivity coefficient.
    pub vertical: f32,
}

#[derive(Resource, Reflect)]
pub struct ControlSettings<T: Actionlike>(pub InputMap<T>);

impl Default for ControlSettings<CharacterMovements> {
    fn default() -> Self {
        let mut map = InputMap::default();

        // Horizontal keyboard movement
        map.insert(CharacterMovements::MoveHorizontally, UserInput::VirtualDPad(VirtualDPad {
            up: InputKind::PhysicalKey(KeyCode::KeyW),
            down: InputKind::PhysicalKey(KeyCode::KeyS),
            left: InputKind::PhysicalKey(KeyCode::KeyA),
            right: InputKind::PhysicalKey(KeyCode::KeyD),
        }));

        // Horizontal controller movement
        map.insert(CharacterMovements::MoveHorizontally, DualAxis::left_stick());

        // Vertical keyboard movement
        map.insert(CharacterMovements::MoveVertically, UserInput::VirtualAxis(VirtualAxis {
            negative: InputKind::PhysicalKey(KeyCode::ControlLeft),
            positive: InputKind::PhysicalKey(KeyCode::Space),
        }));

        // Vertical controller movement
        map.insert(CharacterMovements::MoveVertically, UserInput::VirtualAxis(VirtualAxis {
            negative: InputKind::GamepadButton(GamepadButtonType::East),
            positive: InputKind::GamepadButton(GamepadButtonType::South),
        }));

        // Mouse turning
        map.insert(CharacterMovements::Turn, DualAxis::mouse_motion());

        // Controller turning
        map.insert(CharacterMovements::Turn, DualAxis::right_stick());

        // Keyboard leaning
        map.insert(CharacterMovements::Lean, UserInput::VirtualAxis(VirtualAxis {
            negative: InputKind::PhysicalKey(KeyCode::KeyQ),
            positive: InputKind::PhysicalKey(KeyCode::KeyE),
        }));

        // Controller leaning
        map.insert(CharacterMovements::Lean, UserInput::VirtualAxis(VirtualAxis {
            negative: InputKind::GamepadButton(GamepadButtonType::LeftTrigger),
            positive: InputKind::GamepadButton(GamepadButtonType::RightTrigger),
        }));

        return Self(map);
    }
}

impl Default for ControlSettings<VesselMovements> {
    fn default() -> Self {
        let mut map = InputMap::default();

        // Keyboard forward thrust
        map.insert(VesselMovements::ForwardThrust, UserInput::VirtualAxis(VirtualAxis {
            negative: InputKind::PhysicalKey(KeyCode::KeyS),
            positive: InputKind::PhysicalKey(KeyCode::KeyW),
        }));

        // Controller forward thrust
        map.insert(VesselMovements::ForwardThrust, DualAxis::left_stick().x);

        // Keyboard sideways thrust
        map.insert(VesselMovements::SideThrust, UserInput::VirtualAxis(VirtualAxis {
            negative: InputKind::PhysicalKey(KeyCode::KeyA),
            positive: InputKind::PhysicalKey(KeyCode::KeyD),
        }));

        // Controller sideways thrust
        map.insert(VesselMovements::SideThrust, DualAxis::left_stick().y);

        // Keyboard vertical thrust
        map.insert(VesselMovements::SideThrust, UserInput::VirtualAxis(VirtualAxis {
            negative: InputKind::PhysicalKey(KeyCode::KeyQ),
            positive: InputKind::PhysicalKey(KeyCode::KeyE),
        }));

        // Controller vertical thrust
        map.insert(VesselMovements::SideThrust, UserInput::VirtualAxis(VirtualAxis {
            negative: InputKind::GamepadButton(GamepadButtonType::South),
            positive: InputKind::GamepadButton(GamepadButtonType::East),
        }));

        // Keyboard pitching
        map.insert(VesselMovements::Pitch, UserInput::VirtualAxis(VirtualAxis {
            negative: InputKind::PhysicalKey(KeyCode::KeyJ),
            positive: InputKind::PhysicalKey(KeyCode::KeyU),
        }));

        // Controller pitching
        map.insert(VesselMovements::Pitch, DualAxis::right_stick().x);

        // Keyboard yawing
        map.insert(VesselMovements::Yaw, UserInput::VirtualAxis(VirtualAxis {
            negative: InputKind::PhysicalKey(KeyCode::KeyH),
            positive: InputKind::PhysicalKey(KeyCode::KeyK),
        }));

        // Controller yawing
        map.insert(VesselMovements::Yaw, DualAxis::right_stick().y);

        map.insert(VesselMovements::Brake, KeyCode::Space);
        map.insert(VesselMovements::Brake, GamepadButtonType::South);

        return Self(map);
    }
}