//! Things that are global across the entire program, and decided at compile time.

/// Physics-related globals, like masks.
pub mod physics {
    /// Static or dynamic terrain segments.
    pub const PHYSICS_TERRAIN: u32 = 1 << 0;

    /// Submarines, stations, and wrecks.
    pub const PHYSICS_STRUCTURE: u32 = 1 << 1;

    /// Humans or alien wildlife.
    pub const PHYSICS_CREATURE: u32 = 1 << 2;

    /// A weapon hurtbox.
    pub const PHYSICS_WEAPON: u32 = 1 << 3;

    /// An item on the floor.
    pub const PHYSICS_ITEM: u32 = 1 << 4;
}