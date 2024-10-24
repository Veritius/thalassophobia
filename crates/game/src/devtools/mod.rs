mod diagnostics;
mod inspectors;

use std::marker::PhantomData;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui::{self, UiBuilder}, EguiContext};

pub struct DevtoolsPlugin;

impl Plugin for DevtoolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
        app.add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin);

        app.add_plugins(bevy_egui::EguiPlugin);

        app.add_systems(Update, layout_devtool_ui);

        diagnostics::setup(app);
        inspectors::setup(app);
    }
}

/// Observer event run when painting the devtool UI.
#[derive(Event)]
pub(crate) struct DevtoolLayout<Category = ()> {
    pub ui: egui::Ui,
    _p1: PhantomData<Category>,
}

fn layout_devtool_ui(
    world: &mut World,
) {
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