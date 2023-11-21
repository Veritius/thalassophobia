use bevy::prelude::*;

/// Shoots [`AmmoCartridge`](super::ammo::AmmoCartridge)s  from [`AmmoProvider`](super::ammo::AmmoProvider)s.
#[derive(Debug, Component, Reflect)]
pub struct Gun {
    /// Specifies an `AmmoProvider` to use.
    pub provider: Option<Entity>,

    /// Offset of where the bullets spawn, relative to the [`Transform`] of the entity.
    pub offset: Vec3,

    /// How fast the weapon can fire.
    pub fire_rate: f32,

    /// The inaccuracy of shots from the weapon.
    pub spread: f32,
}