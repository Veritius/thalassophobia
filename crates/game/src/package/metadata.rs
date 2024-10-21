use semver::{Version, VersionReq};
use serde::{Serialize, Deserialize};
use bevy::prelude::*;

#[derive(Debug, Clone, Deserialize)]
pub struct PackageMeta {
    pub name: InlinedString,
    pub shortcode: InlinedString,
    pub package_version: Version,

    pub game_version: VersionReq,
    pub dependencies: Dependencies,

    pub can_insert_mid_save: bool,
    pub can_remove_mid_save: bool,
}

/// A version requirement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub shortcode: InlinedString,
    pub version: VersionReq,
    pub required: bool,
}

/// A set of dependencies for a package.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependencies {
    packages: Vec<Dependency>,
}