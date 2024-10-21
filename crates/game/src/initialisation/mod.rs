mod plugin;

use bevy::prelude::*;

pub use plugin::InitialLoadingPlugin;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
pub enum Initialisation {
    #[default]
    Loading,
    Finished,
}