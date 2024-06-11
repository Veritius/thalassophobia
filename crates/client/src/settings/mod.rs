use std::{any::Any, fmt::Debug, ops::{Deref, DerefMut}};
use shared::{bevy::prelude::*, bevy_ecs, bevy_reflect::{self, GetTypeRegistration}};
use shared::controller::*;

mod access;
mod audio;
mod controls;
mod graphics;

pub use access::*;
pub use audio::*;
pub use controls::*;
pub use graphics::*;

pub(crate) struct UserSettingsPlugin;

impl Plugin for UserSettingsPlugin {
    fn build(&self, app: &mut App) {
        // Accessibility
        register::<Blood>(app);
        register::<Dismemberment>(app);
        register::<GameSpeed>(app);
        register::<Gibbing>(app);
        register::<SensoryShock>(app);

        // Audio settings
        register::<MasterVolume>(app);
        register::<AmbientVolume>(app);
        register::<AudioSonar>(app);
        register::<Subtitles>(app);

        // Graphics settings
        register::<CameraFov>(app);
        register::<ModelQuality>(app);
        register::<TextureQuality>(app);
        register::<ShaderQuality>(app);
        register::<Colorblindness>(app);
        register::<Contrast>(app);
        register::<Flickering>(app);

        // Controls settings
        app.register_type::<ControlSettings<RotationMovements>>();
        app.init_resource::<ControlSettings<RotationMovements>>();
        app.register_type::<ControlSettings<GroundedMovements>>();
        app.init_resource::<ControlSettings<GroundedMovements>>();
        app.register_type::<ControlSettings<FloatingMovements>>();
        app.init_resource::<ControlSettings<FloatingMovements>>();
    }
}

fn register<T>(app: &mut App) where T: SettingsValue {
    app.register_type::<T>();
    app.register_type::<Setting<T>>();
    app.init_resource::<Setting<T>>();
}

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct Setting<T: SettingsValue> {
    inner: T,
}

impl<T> Deref for Setting<T>
where
    T: SettingsValue,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Setting<T>
where
    T: SettingsValue
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

pub trait SettingsValue
where 
    Self: Any + Send + Sync + Debug + Default,
    Self: TypePath + GetTypeRegistration + FromReflect + Reflect,
{}

impl<T> SettingsValue for T
where
    T: Any + Send + Sync + Debug + Default,
    T: TypePath + GetTypeRegistration + FromReflect + Reflect,
{}