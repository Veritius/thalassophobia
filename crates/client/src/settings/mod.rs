use shared::bevy::prelude::*;
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
        // Accessibility settings
        app.register_type::<AccessibilitySettings>();
        app.init_resource::<AccessibilitySettings>();

        // Audio settings
        app.register_type::<AudioSettings>();
        app.init_resource::<AudioSettings>();

        // Controls settings
        app.register_type::<ControlSettings<GroundedHumanMovements>>();
        app.init_resource::<ControlSettings<GroundedHumanMovements>>();
        app.register_type::<ControlSettings<FloatingHumanMovements>>();
        app.init_resource::<ControlSettings<FloatingHumanMovements>>();

        // Graphics settings
        app.register_type::<GraphicsSettings>();
        app.init_resource::<GraphicsSettings>();
    }
}