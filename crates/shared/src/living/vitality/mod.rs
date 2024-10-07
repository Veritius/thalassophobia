pub mod wounds;

use crate::prelude::*;

#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CalculatedVitality {
    pub value: f32,
}