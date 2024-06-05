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
        // Accessibility
        access::setup(app);

        // Audio settings
        app.register_type::<AudioSettings>();
        app.init_resource::<AudioSettings>();

        // Controls settings
        app.register_type::<ControlSettings<RotationMovements>>();
        app.init_resource::<ControlSettings<RotationMovements>>();
        app.register_type::<ControlSettings<GroundedMovements>>();
        app.init_resource::<ControlSettings<GroundedMovements>>();
        app.register_type::<ControlSettings<FloatingMovements>>();
        app.init_resource::<ControlSettings<FloatingMovements>>();

        // Graphics settings
        app.register_type::<GraphicsSettings>();
        app.init_resource::<GraphicsSettings>();
    }
}