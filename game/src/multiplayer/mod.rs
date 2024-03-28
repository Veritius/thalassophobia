use bevy::prelude::*;
use bevy_stardust::prelude::*;
use bevy_stardust_udp::*;

const APP_NET_VERSION: ApplicationNetworkVersion = ApplicationNetworkVersion {
    ident: 0x3E92B4A1CBD963DD,
    major: 0,
    minor: 0,
    banlist: &[],
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, States)]
pub enum MultiplayerState {
    #[default]
    Disabled,
    Standby,
    Running,
}

pub(crate) struct MultiplayerPlugin;

impl Plugin for MultiplayerPlugin {
    fn build(&self, app: &mut App) {
        // Core networking plugin
        app.add_plugins(StardustPlugin);

        // Transport layer plugins
        UdpTransportPlugin::balanced(APP_NET_VERSION);

        app.register_type::<MultiplayerState>();
        app.init_state::<MultiplayerState>();
    }
}