mod battery;
mod sink;
mod source;

pub use battery::Battery;
pub use sink::{PowerSink, CalculatedPowerSink, PowerSinkBundle};
pub use source::{PowerSource, CalculatedPowerSource, PowerSourceBundle};

use crate::prelude::*;

pub(crate) struct ElectricityPlugin;

impl Plugin for ElectricityPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Battery>();
        app.register_type::<PowerSink>();
        app.register_type::<CalculatedPowerSink>();
        app.register_type::<PowerSource>();
        app.register_type::<CalculatedPowerSource>();

        app.register_relation::<SuppliesEnergy>();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum ElectricitySystems {
    Update,
}

#[derive(Relation)]
#[aery(Poly)]
pub struct SuppliesEnergy;