mod diagnostics;
mod inspectors;

use std::marker::PhantomData;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui::{self, UiBuilder}, EguiContext};

pub(crate) use inspectors::Inspectors;

pub struct DevtoolsPlugin;

impl Plugin for DevtoolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
        app.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin);

        app.add_plugins(bevy_egui::EguiPlugin);
        app.add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin);

        app.init_resource::<DevtoolSidebar>();

        app.add_systems(Update, (
            handle_input_system,
            ui_layout_system,
        ).chain());

        diagnostics::setup(app);
        inspectors::setup(app);
    }
}

#[derive(Resource, Clone)]
struct DevtoolSidebar {
    pub visible: bool,

    pub toggle_key: KeyCode,
}

impl Default for DevtoolSidebar {
    fn default() -> Self {
        Self {
            visible: false,

            toggle_key: KeyCode::F3,
        }
    }
}

/// Observer event run when painting the devtool UI.
#[derive(Event)]
pub(crate) struct DevtoolLayout<Category = ()> {
    pub ui: egui::Ui,
    _p1: PhantomData<Category>,
}

fn handle_input_system(
    mut sidebar: ResMut<DevtoolSidebar>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(sidebar.toggle_key) {
        sidebar.visible = !sidebar.visible;
    }
}

fn ui_layout_system(
    world: &mut World,
) {
    // If the sidebar isn't visible, don't continue
    if !world.resource::<DevtoolSidebar>().visible { return };

    // Early return if there's no window that we can use.
    let mut ctx = match world
    .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
    .get_single(world) {
        Ok(v) => v.clone(),
        Err(_) => { return },
    };

    // Side panel :)
    egui::SidePanel::left("devtools")
    .max_width(f32::INFINITY)
    .min_width(0.0)
    .show(ctx.get_mut(), |ui| {
        // Some nice decorations
        ui.heading("Dev tools");
        ui.separator();
        ui.add_space(8.0);

        // Scroll area since it may become long
        egui::ScrollArea::vertical()
        .show(ui, |ui| {
            // Let other systems add elements
            world.trigger(DevtoolLayout {
                ui: ui.new_child(UiBuilder::new()),
                _p1: PhantomData::<()>,
            });
        });
    });
}