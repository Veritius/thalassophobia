//! [Structure] entities and their child entities.

use bevy::prelude::*;

/// A structure entity. This is the root entity of something like a station or submarine.
#[derive(Component, Reflect)]
pub struct Structure;

/// A bundle of components, with everything you need to have a functional [Structure] entity.
#[derive(Bundle)]
pub struct StructureBundle {
    pub structure: Structure,
    pub name: Name,
    pub transform: TransformBundle,
    pub visibility: VisibilityBundle,
}

pub(super) fn setup_structures(app: &mut App) {
    app.register_type::<Structure>();
}