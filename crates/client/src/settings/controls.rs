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

        // Mouse turning
        map.insert(CharacterMovements::Turn, DualAxis::mouse_motion());

        // Controller turning
        map.insert(CharacterMovements::Turn, DualAxis::right_stick());

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