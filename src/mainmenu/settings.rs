use bevy::prelude::*;
use bevy_egui::{egui::{self, Align2}, EguiContexts};
use crate::settings::Settings;

use super::MainMenuPage;

pub(super) fn settings_menu_system(
    mut contexts: EguiContexts,
    mut settings: ResMut<Settings>,
    mut menu: ResMut<MainMenuPage>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Settings")
    .anchor(Align2::CENTER_CENTER, egui::Vec2::splat(0.0))
    .resizable(false)
    .collapsible(false)
    .movable(false)
    .show(ctx, |ui| {
        settings.draw(ui);

        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Done").clicked() {
                *menu = MainMenuPage::FrontPage;
            }
        });
    });
}