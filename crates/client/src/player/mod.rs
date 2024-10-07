pub mod controls;
pub mod gamepads;

use shared::prelude::*;

/// An individual player on this client.
/// 
/// If one `Player` entity is present, it plays like any other game.
/// If multiple entities are present, the game enters split-screen mode.
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Player {

}