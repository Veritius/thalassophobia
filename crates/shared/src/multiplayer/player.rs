use bevy::prelude::*;
use super::permissions::Permissions;

/// An individual player.
#[derive(Debug, Component, Reflect)]
#[reflect(Debug, Component)]
pub struct Player {
    /// The player's display name.
    pub display_name: String,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub player: Player,
    pub permissions: Permissions,
}