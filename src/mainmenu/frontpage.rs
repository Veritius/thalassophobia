use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use super::MainMenuPage;

pub(super) fn front_page_system(
    mut contexts: EguiContexts,
    mut page: ResMut<MainMenuPage>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Thalassophobia").show(ctx, |ui| {
        if ui.button("Settings").clicked() {
            *page = MainMenuPage::Settings;
        }
    });
}