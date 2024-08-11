mod battery;
mod sink;
mod source;

pub use battery::Battery;
pub use sink::{PowerSink, CalculatedPowerSink, PowerSinkBundle};
pub use source::{PowerSource, CalculatedPowerSource, PowerSourceBundle};
pub use crate::math::units::Joule;

use bevy::prelude::*;

pub(crate) struct ElectricityPlugin;

impl Plugin for ElectricityPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Joule>();
        app.register_type::<Battery>();
        app.register_type::<PowerSink>();
        app.register_type::<CalculatedPowerSink>();
        app.register_type::<PowerSource>();
        app.register_type::<CalculatedPowerSource>();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum ElectricitySystems {
    Update,
}