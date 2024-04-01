use shared::bevy::prelude::*;
use shared::movement::*;

mod keybinds;

pub use keybinds::*;

pub(crate) struct UserSettingsPlugin;

impl Plugin for UserSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Bindings<GroundedHumanMovements>>();
        app.init_resource::<Bindings<FloatingHumanMovements>>();
    }
}