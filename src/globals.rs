//! Unchanging global variables, such as physics masks.

/// Terrain, such as rock formations.
pub const PHYSICS_TERRAIN: u32 = 1 << 0;
/// Structures, such as submarines, stations, and wrecks.
pub const PHYSICS_STRUCTURE: u32 = 1 << 1;
/// Creatures, such as humans or mobile alien wildlife.
pub const PHYSICS_CREATURE: u32 = 1 << 2;
/// Bullets, including hitscan and projectile.
pub const PHYSICS_BULLET: u32 = 1 << 3;
/// Items, such as those dropped by players.
pub const PHYSICS_ITEM: u32 = 1 << 4;