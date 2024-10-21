mod plugin;

use bevy::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, States)]
pub enum Initialisation {
    #[default]
    Loading,
    Finished,
}