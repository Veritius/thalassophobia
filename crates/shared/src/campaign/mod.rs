use bevy::prelude::*;
use chrono::{DateTime, Utc};
use semver::Version;
use serde::{Serialize, Deserialize};

pub(crate) struct CampaignPlugin;

impl Plugin for CampaignPlugin {
    fn build(&self, app: &mut App) {

    }
}

#[derive(Debug, Clone, Resource, Deserialize, Serialize)]
pub struct CampaignMeta {
    pub name: String,
    pub version: Version,

    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub playtime: u64,

    pub random_seed: u64,
}