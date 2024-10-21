pub mod permissions;

use crate::prelude::*;
use crate::stardust::plugin::StardustPlugin;

pub(crate) struct MultiplayerPlugin;

impl Plugin for MultiplayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StardustPlugin);

        app.register_relation::<Manages>();
    }
}

/// A relation from a [peer entity](crate::stardust::connections) to a [player](crate::players::Player).
/// If a player is 'managed' it means it is controlled by a remote peer, and inputs originate from the peer.
#[derive(Relation)]
pub struct Manages;