use shared::bevy::prelude::*;
use shared::bevy_ecs;
use shared::bevy_reflect;

#[derive(Resource, Reflect)]
pub struct AccessibilitySettings {
    /*
        General accessibility stuff
    */

    /// Only applies to the host of the game.
    pub game_speed: f32,

    /// Adjusts brightness of the world.
    pub light_gamma: f32,

    /// Color blindness config.
    pub colorblindness: ColorblindMode,

    /*
        Blood and gore
    */

    /// Doesn't actually disable dismemberment, since it affects gameplay.
    /// Instead, limbs are greyed out to mark their disappearance.
    pub disable_dismemberment: bool,

    /// Disables gib effects alongside other gore.
    pub disable_gibbing: bool,

    /// Fully disables blood particle effects.
    pub disable_blood: bool,
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self {
            game_speed: 0.0,
            light_gamma: 1.0,
            colorblindness: ColorblindMode::default(),

            disable_dismemberment: false,
            disable_gibbing: false,
            disable_blood: false,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum ColorblindMode {
    /// Normal color vision.
    #[default]
    FullColor,
    /// Red-green color blindness.
    RedGreen,
    /// Blue-yellow color blindness.
    BlueYellow,
    /// Complete color vision deficiency.
    Monochromia,
}