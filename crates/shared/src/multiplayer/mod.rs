pub mod permissions;

use crate::prelude::*;
use crate::stardust::plugin::StardustPlugin;

pub(crate) struct MultiplayerPlugin;

impl Plugin for MultiplayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StardustPlugin);
    }
}