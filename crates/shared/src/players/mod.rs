#[cfg(feature="multiplayer")]
pub mod multiplayer;

use crate::prelude::*;

/// An individual player that is playing the game.
/// 
/// In multiplayer, this entity may be 'owned' by a network peer.
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Player {

}

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(feature="multiplayer")]
        app.add_plugins(multiplayer::MultiplayerPlugin);
    }
}