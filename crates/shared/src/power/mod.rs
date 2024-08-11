mod battery;
mod sink;
mod source;

pub use battery::Battery;
pub use sink::PowerSink;
pub use source::PowerSource;
pub use crate::math::energy::Joule;

use bevy::prelude::*;

pub(crate) struct ElectricityPlugin;

impl Plugin for ElectricityPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Joule>();
        app.register_type::<Battery>();
        app.register_type::<PowerSink>();
        app.register_type::<PowerSource>();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum ElectricitySystems {
    Update,
}