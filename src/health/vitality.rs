use bevy::prelude::*;

/// Sum of all vitality adjustments for an entity and all of its children.
#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Debug, Component)]
pub struct VitalityAggregate(f32);

impl std::ops::Deref for VitalityAggregate {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}