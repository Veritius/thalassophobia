use shared::prelude::*;

/// A gamepad associated with a [`Player`].
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct PlayerGamepad {
    pub id: Gamepad,
}