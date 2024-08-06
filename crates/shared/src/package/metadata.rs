use semver::{Version, VersionReq};
use serde::Deserialize;

// Since the strings will not change 
type BoxStr = Box<str>;

#[derive(Debug, Deserialize)]
pub struct ContentPackageMetadata {
    pub name: BoxStr,
    pub version: Version,
    pub shortcode: BoxStr,
    pub authors: BoxStr,

    pub game_version: VersionReq,
    pub dependencies: Box<[PackageVersionReq]>,
}

#[derive(Debug, Deserialize)]
pub struct PackageVersionReq {
    pub name: Option<BoxStr>,
    pub shortcode: BoxStr,
    pub version: VersionReq,
}