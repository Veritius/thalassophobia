use std::any::Any;
use std::ops::{Deref, DerefMut};
use std::fmt::Debug;
use shared::bevy::prelude::*;
use shared::{bevy_ecs, bevy_reflect::{self, GetTypeRegistration}};

pub(super) fn setup(app: &mut App) {
    register::<GameSpeed>(app);
    register::<Flickering>(app);
    register::<SensoryShock>(app);
    register::<Contrast>(app);
    register::<Colorblindness>(app);
    register::<AudioSonar>(app);
    register::<Dismemberment>(app);
    register::<Gibbing>(app);
    register::<Blood>(app);
}

fn register<T>(app: &mut App) where T: AccessibilitySetting {
    app.register_type::<T>();
    app.register_type::<Accessibility<T>>();
    app.init_resource::<Accessibility<T>>();
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct Accessibility<T: AccessibilitySetting> {
    inner: T,
}

impl<T> Deref for Accessibility<T>
where
    T: AccessibilitySetting,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Accessibility<T>
where
    T: AccessibilitySetting
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub trait AccessibilitySetting
where 
    Self: Any + Send + Sync + Debug + Default,
    Self: TypePath + GetTypeRegistration + FromReflect + Reflect,
{}

impl<T> AccessibilitySetting for T
where
    T: Any + Send + Sync + Debug + Default,
    T: TypePath + GetTypeRegistration + FromReflect + Reflect,
{}

/// Speed multiplier for the game. Affects [`Virtual`] time.
/// 
/// https://gameaccessibilityguidelines.com/include-an-option-to-adjust-the-game-speed/
#[derive(Debug, Clone, Copy, Reflect)]
pub struct GameSpeed(pub f32);

impl Default for GameSpeed {
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

/// Reduction of sudden sensory shocks, like explosions and gunshots.
/// 
/// https://gameaccessibilityguidelines.com/avoid-any-sudden-unexpected-movement-or-events/
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub enum SensoryShock {
    #[default]
    Standard,
    Reduced,
}

/// Improves the contrast of the world.
#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct Contrast(f32);

impl Default for Contrast {
    fn default() -> Self {
        Self(1.0)
    }
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

/// Visually displays sounds for the hearing impaired.
/// 
/// https://gameaccessibilityguidelines.com/provide-a-pingable-sonar-style-audio-map
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub enum AudioSonar {
    #[default]
    Disabled,
    Enabled,
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

/// Disables gibbing effects.
/// 
/// https://gameaccessibilityguidelines.com/provide-an-option-to-disable-blood-and-gore/
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub enum Gibbing {
    #[default]
    Standard,
    Disabled,
}

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