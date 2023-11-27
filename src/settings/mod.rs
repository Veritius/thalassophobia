pub mod controls;
pub mod graphics;
pub mod audio;
pub mod network;

use bevy::prelude::*;
use self::graphics::*;

pub use self::graphics::GraphicsSettings;
pub use self::controls::ControlsSettings;
pub use self::audio::AudioSettings;
pub use self::network::NetworkSettings;

pub(super) fn setup_settings(app: &mut App) {
    app.register_type::<GraphicsLevel>();

    app.register_type::<GraphicsSettings>();
    app.register_type::<ControlsSettings>();
    app.register_type::<AudioSettings>();
    app.register_type::<NetworkSettings>();

    app.init_resource::<GraphicsSettings>();
    app.init_resource::<ControlsSettings>();
    app.init_resource::<AudioSettings>();
    app.init_resource::<NetworkSettings>();
}