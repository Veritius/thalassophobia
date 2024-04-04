use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use bevy::{asset::AssetLoader, prelude::*};
use rhai::*;

#[derive(Asset, TypePath)]
pub struct Script(pub AST);

impl Deref for Script {
    type Target = AST;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Script {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub(super) struct ScriptAssetLoader {
    pub(super) engine: Arc<Engine>,
}

impl AssetLoader for ScriptAssetLoader {
    type Asset = Script;
    type Settings = ();
    type Error = ParseError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        todo!()
    }

    fn extensions(&self) -> &[&str] {
        &["rhai"]
    }
}