use bevy::{prelude::*, app::AppExit};
use bevy_egui::{egui::{self, Align2}, EguiContexts};
use super::MainMenuPage;

static VERSION_STRING: &'static str = if cfg!(debug_assertions = "true") { "debug" } else { "release" };

pub(super) fn front_page_system(
    mut contexts: EguiContexts,
    mut page: ResMut<MainMenuPage>,
    mut exit: EventWriter<AppExit>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Thalassophobia")
    .anchor(Align2::CENTER_CENTER, egui::Vec2::splat(0.0))
    .default_size(egui::Vec2::splat(0.0))
    .resizable(false)
    .collapsible(false)
    .movable(false)
    .show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            if ui.button("New game").clicked() {
                *page = MainMenuPage::NewGame;
            }
            if ui.button("Load game").clicked() {
                *page = MainMenuPage::LoadGame;
            }
            if ui.button("Join game").clicked() {
                *page = MainMenuPage::JoinGame;
            }
            if ui.button("Settings").clicked() {
                *page = MainMenuPage::Settings;
            }
            if ui.button("Quit").clicked() {
                exit.send(AppExit);
            }
        });

        ui.add_space(4.0);

        ui.small(format!("{} {} ({VERSION_STRING})", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
    });
}