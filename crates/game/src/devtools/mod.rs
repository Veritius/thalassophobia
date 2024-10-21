use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::initialisation::Initialisation;

pub struct DevtoolsPlugin;

impl Plugin for DevtoolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egui::EguiPlugin);
        app.add_plugins(WorldInspectorPlugin::new());

        app.register_type::<SidebarVisibility>();
        app.init_resource::<SidebarVisibility>();

        app.add_systems(PostUpdate, sidebar_system
            .run_if(in_state(Initialisation::Finished)));
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Resource, Reflect)]
#[reflect(Resource)]
enum SidebarVisibility {
    #[default]
    Hidden,
    Visible,
}

fn sidebar_system(
    mut ctx: EguiContexts,
    // show: Res<SidebarVisibility>,
) {
    // Quick check to see if we should actually show this
    // if *show == SidebarVisibility::Hidden { return }

    // show the sidebar
    egui::SidePanel::left("devtools_sidebar")
    .show(ctx.ctx_mut(), |ui| {
        ui.heading("Dev tools");
    });
}