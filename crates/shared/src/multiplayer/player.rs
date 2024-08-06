use bevy::prelude::*;
use super::permissions::Permission;

/// An individual player.
#[derive(Component)]
pub struct Player {

}

#[derive(Component)]
pub struct PlayerBundle {
    pub name: Name,
    pub player: Player,
    pub permissions: Permission,
}