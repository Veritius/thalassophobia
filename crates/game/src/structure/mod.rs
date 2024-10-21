pub mod compartments;

use bevy::prelude::*;

/// A marker component for structures.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Component, Reflect)]
#[reflect(Default, PartialEq, Component)]
pub struct Structure;

pub(crate) struct StructuresPlugin;

impl Plugin for StructuresPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Structure>();

        app.register_type::<compartments::Compartment>();
        app.register_type::<compartments::CompartmentWater>();
        app.register_type::<compartments::CompartmentPortal>();
    }
}