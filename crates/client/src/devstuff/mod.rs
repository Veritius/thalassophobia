mod controllers;
mod input;

use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use shared::{bevy::prelude::*, bevy_ecs, bevy_reflect};
use controllers::*;

pub(crate) struct DevStuffPlugin;

impl Plugin for DevStuffPlugin {
    fn build(&self, app: &mut App) {
        // Add bevy-inspector-egui
        app.add_plugins(WorldInspectorPlugin::new()
            .run_if(resource_exists_and_equals(WorldInspectorVisibility::Show)));

        // Add egui (if it isn't added already)
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        app.add_systems(Update, input::show_hide_toggles_system);
        app.add_systems(Update, (controllers::config_window, controllers::draw_gizmos));

        app.register_type::<WorldInspectorVisibility>();
        app.init_resource::<WorldInspectorVisibility>();
        app.register_type::<PlayerControllerGizmos>();
        app.init_resource::<PlayerControllerGizmos>();
    }
}

#[derive(Resource, Default, Reflect, PartialEq, Eq)]
enum WorldInspectorVisibility {
    Show,
    #[default]
    Hide,
}