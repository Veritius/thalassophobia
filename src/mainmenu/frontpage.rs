use bevy::prelude::*;
use bevy_egui::{egui::{self, Align2}, EguiContexts};
use super::MainMenuPage;

pub(super) fn front_page_system(
    mut contexts: EguiContexts,
    mut page: ResMut<MainMenuPage>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Thalassophobia")
    .anchor(Align2::CENTER_CENTER, egui::Vec2::splat(0.0))
    .resizable(false)
    .collapsible(false)
    .movable(false)
    .show(ctx, |ui| {
        if ui.button("Settings").clicked() {
            *page = MainMenuPage::Settings;
        }
    });
}