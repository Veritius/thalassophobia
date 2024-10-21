use std::net::SocketAddr;
use shared::{prelude::*, bevy_ecs::system::SystemParam};

#[derive(SystemParam)]
pub struct JoinAttempts {

}

impl JoinAttempts {
    pub fn join(
        &mut self,
        remote: SocketAddr,
    ) {
        todo!()
    }
}