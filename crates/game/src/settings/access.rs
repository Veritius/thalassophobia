use std::fmt::Debug;
use std::ops::RangeInclusive;
use bevy::prelude::*;

/// Alters or disables blood effects.
/// 
/// https://gameaccessibilityguidelines.com/provide-an-option-to-disable-blood-and-gore/
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub enum Blood {
    #[default]
    Standard,
    Recolor(Color),
    Disabled,
}

/// Reduces dismemberment effects.
/// Since dismemberment is a game mechanic, this cannot be done fully.
/// Instead, the destroyed limb is just shown as partially transparent.
/// 
/// https://gameaccessibilityguidelines.com/provide-an-option-to-disable-blood-and-gore/
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub enum Dismemberment {
    #[default]
    Standard,
    Reduced,
}

/// Speed multiplier for the game. Affects [`Virtual`] time.
/// 
/// https://gameaccessibilityguidelines.com/include-an-option-to-adjust-the-game-speed/
#[derive(Debug, Clone, Copy, Reflect)]
pub struct GameSpeed {
    #[reflect(@Self::LIMIT)]
    pub speed: f32,
}

impl GameSpeed {
    pub const LIMIT: RangeInclusive<f32> = 0.1 ..= 1.3;
}

impl Default for GameSpeed {
    fn default() -> Self {
        Self { speed: 1.0 }
    }
}

/// Disables giblets and viscera effects.
/// 
/// https://gameaccessibilityguidelines.com/provide-an-option-to-disable-blood-and-gore/
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub enum Giblets {
    #[default]
    Standard,
    Disabled,
}

/// Reduction of sudden sensory shocks, like explosions and gunshots.
/// 
/// https://gameaccessibilityguidelines.com/avoid-any-sudden-unexpected-movement-or-events/
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub enum SensoryShock {
    #[default]
    Standard,
    Reduced,
}