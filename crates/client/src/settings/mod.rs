use shared::bevy::prelude::*;
use shared::player::movement::*;

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
        // Accessibility settings
        app.init_resource::<AccessibilitySettings>();

        // Audio settings
        app.init_resource::<AudioSettings>();

        // Controls settings
        app.init_resource::<ControlSettings<GroundedHumanMovements>>();
        app.init_resource::<ControlSettings<FloatingHumanMovements>>();

        // Graphics settings
        app.init_resource::<GraphicsSettings>();
    }
}