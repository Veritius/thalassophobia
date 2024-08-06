pub use bevy;
pub use bevy::ecs as bevy_ecs;
pub use bevy::state as bevy_state;
pub use bevy::reflect as bevy_reflect;
pub use bevy_rapier3d as rapier;
pub use leafwing_input_manager as input;
pub use bevy_mod_progress as progress;

pub use smallvec;
pub use serde;
pub use chrono;
pub use semver;

mod setup;
pub use setup::*;

pub mod campaign;
pub mod controller;
pub mod disabling;
pub mod living;
pub mod math;
pub mod package;
pub mod physics;
pub mod state;
pub mod structure;
pub mod vessel;

#[cfg(feature="multiplayer")]
pub mod multiplayer;