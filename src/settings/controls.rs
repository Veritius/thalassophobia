use bevy::prelude::*;
use serde::{Serialize, Deserialize};

/// Controls for the game.
#[derive(Debug, Resource, Reflect, Serialize, Deserialize)]
#[reflect(Default, Resource, Serialize, Deserialize)]
pub struct ControlsSettings {
    #[cfg(feature="dev")]
    pub toggle_dev_menu: KeyCode,

    pub walk_forward: KeyCode,
    pub walk_backward: KeyCode,
    pub walk_left: KeyCode,
    pub walk_right: KeyCode,

    pub roll_left: KeyCode,
    pub roll_right: KeyCode,

    pub sprint: KeyCode,
    pub ascend: KeyCode,
    pub descend: KeyCode,

    pub action_primary: MouseButton,
    pub action_secondary: MouseButton,
}

impl Default for ControlsSettings {
    fn default() -> Self {
        Self {
            #[cfg(feature="dev")]
            toggle_dev_menu: KeyCode::Grave,

            walk_forward: KeyCode::W,
            walk_backward: KeyCode::S,
            walk_left: KeyCode::A,
            walk_right: KeyCode::D,

            roll_left: KeyCode::Q,
            roll_right: KeyCode::R,

            sprint: KeyCode::ShiftLeft,
            ascend: KeyCode::Space,
            descend: KeyCode::ControlLeft,

            action_primary: MouseButton::Left,
            action_secondary: MouseButton::Right,
        }
    }
}