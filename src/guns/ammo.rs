use bevy::prelude::*;

/// Can be fired from a gun as a projectile! Not recommended to add to players.
#[derive(Debug, Component, Reflect)]
pub struct AmmoCartridge {
    
}

/// Provides a 'source' of ammunition for a `FiringPiece`, shooting [AmmoCartridge] entities.
#[derive(Debug, Component, Reflect)]
pub struct AmmoProvider {

}