use shared::bevy::prelude::*;
use shared::{bevy_ecs, bevy_reflect};

#[derive(Resource, Reflect)]
pub struct AccessibilitySettings {
    /*
        Motor
    */

    /// Reduces the speed of the game to be slower.
    /// Only applies to the host of the game.
    /// 
    /// https://gameaccessibilityguidelines.com/include-an-option-to-adjust-the-game-speed/
    pub game_speed: f32,

    /*
        Cognitive
    */

    /// Disables flickering lights and images.
    /// 
    /// https://gameaccessibilityguidelines.com/avoid-flickering-images-and-repetitive-patterns/
    pub disable_flicker: bool,

    /// Reduces sudden sensory shocks, like loud sounds.
    ///
    /// https://gameaccessibilityguidelines.com/avoid-any-sudden-unexpected-movement-or-events/
    pub reduce_shock: bool,

    /*
        Vision
    */

    /// Adjusts brightness of the world.
    pub light_gamma: f32,

    /// Color blindness config.
    pub colorblindness: ColorblindMode,

    /*
        Hearing
    */

    /// Shows sounds as a visual representation at the center of the screen.
    /// 
    /// https://gameaccessibilityguidelines.com/provide-a-pingable-sonar-style-audio-map
    pub audio_sonar: bool,

    /*
        Content
    */

    /// Doesn't actually disable dismemberment, since it affects gameplay.
    /// Instead, limbs are greyed out to mark their disappearance.
    pub disable_dismemberment: bool,

    /// Disables gib effects alongside other gore.
    pub disable_gibbing: bool,

    /// Changes the color of blood to something else.
    pub blood_recolor: Option<Color>,

    /// Fully disables blood particle effects.
    pub disable_blood: bool,
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self {
            game_speed: 1.0,

            disable_flicker: false,
            reduce_shock: false,

            audio_sonar: false,

            light_gamma: 1.0,
            colorblindness: ColorblindMode::default(),

            disable_dismemberment: false,
            disable_gibbing: false,
            blood_recolor: None,
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