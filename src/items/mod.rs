mod inventory;

use bevy::prelude::*;

pub(super) fn setup_items(app: &mut App) {
    app.register_type::<Item>();
    app.register_type::<inventory::Inventory>();
}

/// An item that can be stored in an inventory.
#[derive(Debug, Component, Reflect)]
pub struct Item {
    /// The slots the item takes up in an inventory.
    pub size: UVec2,
}