use bevy::prelude::*;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>();

    }
}

/// An individual player that is playing the game.
/// 
/// In multiplayer, this entity may be 'owned' by a network peer.
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Player {

}