use shared::prelude::*;

/// Control settings stored on a per-player basis.
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct PlayerControls<T: Actionlike> {
    /// The player's controls.
    pub map: InputMap<T>
}