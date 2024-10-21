use super::*;

pub(super) struct NetTransportQuic;

impl Plugin for NetTransportQuic {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_stardust_quinn::QuinnPlugin);
    }
}