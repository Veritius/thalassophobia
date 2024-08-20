use crate::prelude::*;

pub(crate) struct DisablingPlugin;

impl Plugin for DisablingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Disabled>();
    }
}

/// Disabled entities are frozen in time and will not be ticked by most game systems.
#[derive(Debug, Default, Clone, Copy, Component, PartialEq, Eq, PartialOrd, Ord, Reflect, Hash)]
#[reflect(Default, Component, PartialEq, Hash)]
pub struct Disabled;