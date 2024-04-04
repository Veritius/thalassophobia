mod asset;

pub use asset::Script;

use std::{ops::Deref, sync::Arc};
use bevy::prelude::*;
use rhai::*;

pub(crate) struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut App) {
        // Add the primary script engine
        let engine = Engine::new();
        let engine = Arc::new(engine);
        app.insert_resource(ScriptEngine(engine.clone()));

        // Add the asset loader
        let asset_server = app.world.resource_mut::<AssetServer>();
        asset_server.register_loader(asset::ScriptAssetLoader { engine });
    }
}

#[derive(Resource, Clone)]
pub struct ScriptEngine(Arc<Engine>);

impl Deref for ScriptEngine {
    type Target = Engine;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}