use bevy::prelude::*;
use bevy_egui::{egui::{self, Align2}, EguiContexts};
use crate::settings::*;
use super::MainMenuPage;

pub(super) fn settings_menu_system(
    mut contexts: EguiContexts,
    mut graphics: ResMut<Graphics>,
    mut menu: ResMut<MainMenuPage>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Settings")
    .anchor(Align2::CENTER_CENTER, egui::Vec2::splat(0.0))
    .default_size(egui::Vec2::splat(0.0))
    .resizable(false)
    .collapsible(false)
    .movable(false)
    .show(ctx, |ui| {
        todo!()
    });
}