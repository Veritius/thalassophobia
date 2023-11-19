use std::fmt::Display;
use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Resource, Reflect, Serialize, Deserialize)]
#[reflect(Default, Resource, Serialize, Deserialize)]
pub struct GraphicsSettings {
    pub model_detail: GraphicsLevel,
    pub texture_quality: GraphicsLevel,
    pub lighting_quality: GraphicsLevel,
    pub particle_quality: GraphicsLevel,
    pub shader_quality: GraphicsLevel,
    pub lod_aggression: f32,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(Default, Serialize, Deserialize)]
pub enum GraphicsLevel {
    Low,
    #[default]
    Medium,
    High,
}

impl Display for GraphicsLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use GraphicsLevel::*;
        f.write_str(match self {
            Low => "Low",
            Medium => "Medium",
            High => "High",
        })
    }
}