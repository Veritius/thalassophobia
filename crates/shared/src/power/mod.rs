mod sink;
mod source;
mod transfer;

pub use sink::PowerSink;
pub use source::PowerSource;
pub use transfer::Joule;

use bevy::prelude::*;

pub(crate) struct ElectricityPlugin;

impl Plugin for ElectricityPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<sink::PowerSink>();
        app.register_type::<source::PowerSource>();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum ElectricitySystems {
    Update,
}