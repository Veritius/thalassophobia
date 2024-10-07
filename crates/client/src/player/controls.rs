use shared::prelude::*;

/// Control settings associated with a [`Player`].
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct PlayerControls<T: Actionlike> {
    /// The player's controls.
    pub map: InputMap<T>
}