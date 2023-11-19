use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Resource, Reflect, Serialize, Deserialize)]
#[reflect(Default, Resource, Serialize, Deserialize)]
pub struct AudioSettings {
    pub level_master: f32,
}