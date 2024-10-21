use std::ops::Not;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::initialisation::Initialisation;

pub struct DevtoolsPlugin;

impl Plugin for DevtoolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egui::EguiPlugin);
        app.add_plugins(WorldInspectorPlugin::new());

        app.register_type::<Visibility>();
        app.register_type::<SidebarSettings>();
        app.init_resource::<SidebarSettings>();

        app.add_systems(PostUpdate, sidebar_system
            .run_if(in_state(Initialisation::Finished)));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Resource, Reflect)]
#[reflect(Resource)]
struct SidebarSettings {
    visible: Visibility,
}

impl Default for SidebarSettings {
    fn default() -> Self {
        Self {
            visible: Visibility::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Reflect)]
enum Visibility {
    #[default]
    Hidden,
    Visible,
}

impl Not for Visibility {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Visibility::Hidden => Visibility::Visible,
            Visibility::Visible => Visibility::Hidden,
        }
    }
}

fn sidebar_system(
    mut ctx: EguiContexts,
    settings: Res<SidebarSettings>,
) {
    // Quick check to see if we should actually show this
    if settings.visible == Visibility::Hidden { return }

    // show the sidebar
    egui::SidePanel::left("devtools_sidebar")
    .show(ctx.ctx_mut(), |ui| {
        ui.heading("Dev tools");

        ui.separator();

        egui::ScrollArea::vertical()
        .show(ui, |ui| {
            egui::CollapsingHeader::new("Entities")
            .show(ui, |ui| {

            });

            egui::CollapsingHeader::new("Resources")
            .show(ui, |ui| {

            });

            egui::CollapsingHeader::new("States")
            .show(ui, |ui| {

            });

            egui::CollapsingHeader::new("Assets")
            .show(ui, |ui| {

            });

            #[cfg(feature="multiplayer")]
            egui::CollapsingHeader::new("Players")
            .show(ui, |ui| {

            });
        });
    });
}