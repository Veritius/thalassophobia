#[cfg(feature="multiplayer_quic")]
mod quic;

pub use bevy_stardust as stardust;

pub mod permissions;

use crate::prelude::*;
use stardust::prelude::*;

pub(crate) struct MultiplayerPlugin;

impl Plugin for MultiplayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StardustPlugin);

        app.register_type::<Protocols>();
        app.register_relation::<Manages>();

        #[cfg(feature="multiplayer_quic")]
        app.add_plugins(quic::NetTransportQuic);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
#[reflect(PartialEq)]
pub enum Protocols {
    #[cfg(feature="multiplayer_quic")]
    Quic,
}

/// A relation from a [peer entity](stardust::connections) to a [player](crate::players::Player).
/// If a player is 'managed' it means it is controlled by a remote peer, and inputs originate from the peer.
#[derive(Relation)]
pub struct Manages;