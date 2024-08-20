pub mod permissions;
pub mod player;

use bevy::prelude::*;

#[cfg(feature="multiplayer")]
pub use bevy_stardust as stardust;

pub(crate) struct MultiplayerPlugin;

impl Plugin for MultiplayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<player::Player>();
    }
}