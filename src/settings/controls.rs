use bevy::prelude::*;
use serde::{Serialize, Deserialize};

// TODO: Allow changing the controls in the settings menu

/// Controls for the game.
#[derive(Debug, Resource, Reflect, Serialize, Deserialize)]
#[reflect(Default, Resource, Serialize, Deserialize)]
pub struct Controls {
    #[cfg(feature="dev")]
    pub toggle_dev_menu: KeyCode,

    pub walk_forward: KeyCode,
    pub walk_backward: KeyCode,
    pub walk_left: KeyCode,
    pub walk_right: KeyCode,

    pub roll_left: KeyCode,
    pub roll_right: KeyCode,

    pub mod_sprint: KeyCode,
    pub mod_crouch: KeyCode,

    pub action_primary: MouseButton,
    pub action_secondary: MouseButton,
}

impl Default for Controls {
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

            mod_sprint: KeyCode::ShiftLeft,
            mod_crouch: KeyCode::ControlLeft,

            action_primary: MouseButton::Left,
            action_secondary: MouseButton::Right,
        }
    }
}