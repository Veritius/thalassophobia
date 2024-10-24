use std::marker::PhantomData;
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui::{self, UiBuilder}, EguiContext};

pub struct DevtoolsPlugin;

impl Plugin for DevtoolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egui::EguiPlugin);

        app.add_systems(Update, layout_devtool_ui);
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

    egui::SidePanel::left("devtools")
    .show(ctx.get_mut(), |ui| {
        // Some nice decorations
        ui.heading("Dev tools");

        // Let other systems add elements
        world.trigger(DevtoolLayout {
            ui: ui.new_child(UiBuilder::new()),
            _p1: PhantomData::<()>,
        });
    });
}