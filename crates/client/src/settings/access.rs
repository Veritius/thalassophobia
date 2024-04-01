use shared::bevy::prelude::*;
use shared::bevy_ecs;
use shared::bevy_reflect;

#[derive(Resource, Reflect)]
pub struct AccessibilitySettings {
    pub colorblindness: ColorblindMode,

    /*
        Blood and gore
    */

    /// Doesn't actually disable dismemberment, since it affects gameplay.
    /// Instead, it just tones it down a significant amount.
    pub disable_dismemberment: bool,
    pub disable_gibbing: bool,
    /// Fully disables blood particle effects.
    pub disable_blood: bool,
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self {
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