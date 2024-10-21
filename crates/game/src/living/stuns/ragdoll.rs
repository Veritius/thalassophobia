use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct RagdollStun {
    #[reflect(@0.0 ..= f32::INFINITY)]
    pub seconds: f32,

    pub resilience: f32,
}

impl Default for RagdollStun {
    fn default() -> Self {
        Self {
            seconds: 0.0,

            resilience: 1.0,
        }
    }
}