pub mod controls;
pub mod graphics;
pub mod audio;

use bevy::prelude::*;
use self::graphics::*;

pub use self::graphics::GraphicsSettings;
pub use self::controls::ControlsSettings;
pub use self::audio::AudioSettings;

pub(super) fn setup_settings(app: &mut App) {
    app.register_type::<GraphicsLevel>();

    app.register_type::<GraphicsSettings>();
    app.register_type::<ControlsSettings>();
    app.register_type::<AudioSettings>();

    app.init_resource::<GraphicsSettings>();
    app.init_resource::<ControlsSettings>();
    app.init_resource::<AudioSettings>();
}