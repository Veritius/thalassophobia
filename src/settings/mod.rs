pub mod controls;
pub mod graphics;

use bevy::prelude::*;
use self::graphics::*;

pub use self::graphics::Graphics;
pub use self::controls::Controls;

pub(super) fn setup_settings(app: &mut App) {
    app.register_type::<GraphicsLevel>();

    app.register_type::<Graphics>();
    app.register_type::<Controls>();

    app.init_resource::<Graphics>();
    app.init_resource::<Controls>();
}