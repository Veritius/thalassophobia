mod sink;
mod source;
mod storage;

#[cfg(feature="devtools")]
mod devtools;

pub use sink::{PowerSink, CalculatedPowerSink, PowerSinkBundle};
pub use source::{PowerSource, CalculatedPowerSource, PowerSourceBundle};
pub use storage::PowerStorage;

use bevy::prelude::*;
use aery::prelude::*;

pub(crate) struct ElectricityPlugin;

impl Plugin for ElectricityPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PowerStorage>();
        app.register_type::<PowerSink>();
        app.register_type::<CalculatedPowerSink>();
        app.register_type::<PowerSource>();
        app.register_type::<CalculatedPowerSource>();

        app.register_relation::<SuppliesEnergy>();

        #[cfg(feature="devtools")]
        devtools::setup(app);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum ElectricitySystems {
    Update,
}

#[derive(Relation)]
#[aery(Poly)]
pub struct SuppliesEnergy;