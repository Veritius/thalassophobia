use shared::bevy::prelude::*;
use shared::bevy_ecs;
use shared::bevy_reflect;

#[derive(Resource, Reflect)]
pub struct GraphicsSettings {
    pub model_detail: GraphicsLevel,
    pub texture_detail: GraphicsLevel,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
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