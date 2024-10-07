mod infodump;
mod input;

use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use shared::{bevy::{prelude::*, diagnostic::*}, bevy_ecs, bevy_reflect};
use infodump::*;

pub(crate) struct DevStuffPlugin;

impl Plugin for DevStuffPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin);
        app.add_plugins(EntityCountDiagnosticsPlugin);

        // Add bevy-inspector-egui
        app.add_plugins(WorldInspectorPlugin::new()
            .run_if(resource_exists_and_equals(WorldInspectorVisibility::Show)));

        // Add egui (if it isn't added already)
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        #[cfg(feature="multiplayer")] {
            use shared::stardust::diagnostics::*;

            app.add_plugins((
                PeerDiagnosticPlugin,
                MessageCountDiagnosticsPlugin,
            ));
        }

        app.add_systems(Update, input::show_hide_toggles_system);
        app.add_systems(Update, infodump::infodump_window);

        app.register_type::<WorldInspectorVisibility>();
        app.init_resource::<WorldInspectorVisibility>();
        app.register_type::<InfodumpWindowVisibility>();
        app.init_resource::<InfodumpWindowVisibility>();
    }
}

#[derive(Resource, Default, Reflect, PartialEq, Eq)]
#[reflect(Resource)]
enum WorldInspectorVisibility {
    Show,
    #[default]
    Hide,
}