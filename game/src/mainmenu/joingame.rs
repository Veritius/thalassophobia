use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContexts};
use super::MainMenuPage;

pub(super) fn join_game_menu_system(
    mut contexts: EguiContexts,
    mut menu: ResMut<MainMenuPage>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Join game")
    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::splat(0.0))
    .default_size(egui::Vec2::splat(0.0))
    .resizable(false)
    .collapsible(false)
    .movable(false)
    .show(ctx, |ui| {
        ui.heading("WIP :)");

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Cancel").clicked() {
                *menu = MainMenuPage::FrontPage;
            }
            if ui.button("Confirm").clicked() {
                todo!()
            }
        });
    });
}