use bevy::prelude::*;

// TODO: Allow changing the controls in the settings menu

/// Controls for the game.
#[derive(Debug, Resource, Reflect)]
#[reflect(Resource)]
pub struct Controls {
    #[cfg(feature="dev")]
    pub toggle_dev_menu: KeyCode,

    pub walk_forward: KeyCode,
    pub walk_backward: KeyCode,
    pub strafe_left: KeyCode,
    pub strafe_right: KeyCode,
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
            strafe_left: KeyCode::A,
            strafe_right: KeyCode::D,
            mod_sprint: KeyCode::ShiftLeft,
            mod_crouch: KeyCode::ControlLeft,

            action_primary: MouseButton::Left,
            action_secondary: MouseButton::Right,
        }
    }
}