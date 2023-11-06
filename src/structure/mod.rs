//! [Structure] entities and their child entities.

use bevy::prelude::*;

/// A structure entity.
#[derive(Component)]
pub struct Structure {
    pub name: String,
}

/// A bundle of components, with everything you need to have a functional [Structure] entity.
#[derive(Bundle)]
pub struct StructureBundle {
    pub structure: Structure,
    pub transform: TransformBundle,
    pub visibility: VisibilityBundle,
}