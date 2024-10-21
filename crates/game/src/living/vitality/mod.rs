pub mod wounds;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CalculatedVitality {
    pub value: f32,
}