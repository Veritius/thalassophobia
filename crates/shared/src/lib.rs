pub use bevy as bevy;
pub use bevy::ecs as bevy_ecs;
pub use bevy::reflect as bevy_reflect;
pub use bevy_rapier3d as rapier;
pub use leafwing_input_manager as input;

#[cfg(feature="multiplayer")]
pub use bevy_stardust as stardust;

mod setup;
pub use setup::setup;

pub mod movement;
pub mod state;