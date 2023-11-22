use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Resource, Reflect, Serialize, Deserialize)]
#[reflect(Default, Resource, Serialize, Deserialize)]
pub struct AudioSettings {
    pub level_master: f32,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            level_master: 0.5,
        }
    }
}