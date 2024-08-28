use bevy::prelude::*;
use chrono::{DateTime, Utc};
use semver::{Version, VersionReq};
use serde::{Serialize, Deserialize};
use crate::package::metadata::Dependencies;

pub(crate) struct CampaignPlugin;

impl Plugin for CampaignPlugin {
    fn build(&self, app: &mut App) {

    }
}

#[derive(Debug, Clone, Resource, Serialize, Deserialize)]
pub struct CampaignMeta {
    pub name: String,
    pub version: Version,

    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub playtime: u64,

    pub random_seed: u64,

    pub game_version: VersionReq,
    pub dependencies: Dependencies,
}