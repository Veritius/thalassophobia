mod markers;

use std::marker::PhantomData;
use shared::{bevy::prelude::*, bevy_ecs, smallvec::SmallVec};
use self::markers::*;

pub type MeshDetailLevels = DetailLevels<Mesh>;
pub type BaseColorDetailLevels = DetailLevels<Image, BaseColor>;
pub type MetallicRoughnessDetailLevels = DetailLevels<Image, MetallicRoughness>;
pub type NormalMapDetailLevels = DetailLevels<Image, NormalMap>;

pub(crate) struct LevelOfDetailPlugin;

impl Plugin for LevelOfDetailPlugin {
    fn build(&self, app: &mut App) {

    }
}

#[derive(Debug, Default, Component)]
pub struct DetailLevels<A: Asset, M: ?Sized = ()> {
    pub inner: SmallVec<[Handle<A>; 3]>,
    phantom: PhantomData<M>,
}

impl<A: Asset, M: ?Sized> DetailLevels<A, M> {
    pub fn highest(&self) -> Option<Handle<A>> {
        self.inner.first().map(|v| v.clone())
    }

    pub fn lowest(&self) -> Option<Handle<A>> {
        self.inner.last().map(|v| v.clone())
    }
}

impl<A: Asset, M: ?Sized> FromIterator<Handle<A>> for DetailLevels<A, M> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = Handle<A>>>(iter: T) -> Self {
        Self {
            inner: SmallVec::from_iter(iter),
            phantom: PhantomData,
        }
    }
}