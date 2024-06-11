use shared::bevy::prelude::*;
use shared::bevy_reflect;

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct CameraFov(pub f32);

impl Default for CameraFov {
    fn default() -> Self {
        Self(80.0)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub struct ModelQuality(pub Quality);

#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub struct TextureQuality(pub Quality);

#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub struct ShaderQuality(pub Quality);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum Quality {
    Low,
    #[default]
    Medium,
    High,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub enum Colorblindness {
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

/// Improves the visual contrast of the world.
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct Contrast(f32);

impl Default for Contrast {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Reduces flickering light effects and repetitive patterns.
/// 
/// http://gameaccessibilityguidelines.com/avoid-flickering-images-and-repetitive-patterns
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub enum Flickering {
    #[default]
    Standard,
    Reduced,
}