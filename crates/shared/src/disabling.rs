use bevy::prelude::*;

/// Disabled entities are frozen in time.
#[derive(Debug, Clone, Component, PartialEq, Eq, PartialOrd, Ord, Reflect, Hash)]
pub struct Disabled;