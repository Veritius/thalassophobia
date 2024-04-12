use shared::bevy::prelude::*;
use shared::{bevy_ecs, bevy_reflect};

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct GraphicsSettings {
    pub camera_fov: f32,
    pub model_detail: GraphicsLevel,
    pub texture_detail: GraphicsLevel,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            camera_fov: 80.0,
            model_detail: default(),
            texture_detail: default(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum GraphicsLevel {
    Low,
    #[default]
    Medium,
    High,
}