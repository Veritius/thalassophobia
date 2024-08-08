use bevy::prelude::*;
use super::permissions::Permissions;

/// An individual player.
#[derive(Component)]
pub struct Player {

}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub player: Player,
    pub permissions: Permissions,
}