pub use bevy_stardust::prelude::*;
pub use bevy_stardust_udp as udp;

use udp::ApplicationNetworkVersion;

pub(crate) const UDP_APPLICATION_VERSION: ApplicationNetworkVersion = ApplicationNetworkVersion {
    ident: 0x9AA9A2EAC72E1CBF,
    major: 0,
    minor: 0,
    banlist: &[],
};