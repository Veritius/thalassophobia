use std::ops::RangeInclusive;
use shared::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct CameraFov {
    #[reflect(@Self::LIMIT)]
    pub fov: f32,
}

impl CameraFov {
    pub const LIMIT: RangeInclusive<f32> = 60.0 ..= 120.0;
}

impl Default for CameraFov {
    fn default() -> Self {
        Self { fov: 80.0 }
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
pub struct Contrast {
    #[reflect(@Self::LIMIT)]
    pub contrast: f32,
}

impl Contrast {
    pub const LIMIT: RangeInclusive<f32> = 0.7 ..= 1.3;
}

impl Default for Contrast {
    fn default() -> Self {
        Self { contrast: 1.0 }
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