use bevy::prelude::*;

/// Item storage.
#[derive(Debug, Component, Reflect)]
pub struct Inventory {
    pub size: UVec2,
}