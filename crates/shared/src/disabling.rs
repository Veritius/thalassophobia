use bevy::prelude::*;

pub(crate) struct DisablingPlugin;

impl Plugin for DisablingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Disabled>();
    }
}

/// Disabled entities are frozen in time and will not be ticked by most game systems.
#[derive(Debug, Clone, Component, PartialEq, Eq, PartialOrd, Ord, Reflect, Hash)]
pub struct Disabled;