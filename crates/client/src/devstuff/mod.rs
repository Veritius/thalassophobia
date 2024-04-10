use bevy_egui::EguiPlugin;
use shared::bevy::prelude::*;

pub(crate) struct DevStuffPlugin;

impl Plugin for DevStuffPlugin {
    fn build(&self, app: &mut App) {
        // Add egui.
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }
    }
}