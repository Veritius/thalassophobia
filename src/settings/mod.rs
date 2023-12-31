mod options;
mod widget;

use bevy::prelude::*;
use self::options::GraphicsLevel;

pub(super) fn setup_settings(app: &mut App) {
    app.register_type::<Settings>();
    app.register_type::<GraphicsLevel>();

    app.init_resource::<Settings>();
}

#[derive(Debug, Resource, Reflect)]
pub struct Settings {
    // Graphics
    pub model_detail: GraphicsLevel,
    pub texture_quality: GraphicsLevel,
    pub particle_quality: GraphicsLevel,
    pub shader_quality: GraphicsLevel,
    pub lod_aggression: f32,

    // Audio
    pub main_volume: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            model_detail: Default::default(),
            texture_quality: Default::default(),
            particle_quality: Default::default(),
            shader_quality: Default::default(),
            lod_aggression: 0.0,

            main_volume: 0.5,
        }
    }
}