use bevy::{prelude::*, app::AppExit};
use bevy_egui::{egui::{self, Align2}, EguiContexts};
use crate::gamestate::AppState;
use super::MainMenuPage;

static VERSION_STRING: &'static str = if cfg!(debug_assertions = "true") { "debug" } else { "release" };

pub(super) fn front_page_system(
    mut contexts: EguiContexts,
    mut page: ResMut<MainMenuPage>,
    mut exit: EventWriter<AppExit>,
    mut next_app_state: ResMut<NextState<AppState>>,
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
            ui.style_mut().spacing.item_spacing = [0.0, 4.0].into();

            fn main_menu_button(
                ui: &mut egui::Ui,
                label: &'static str,
            ) -> bool {
                ui.add_sized([90.0, 12.0], egui::Button::new(label)).clicked()
            }

            if main_menu_button(ui, "Create lobby") {
                next_app_state.set(AppState::Lobby);
            }

            if main_menu_button(ui, "Join game") {
                *page = MainMenuPage::JoinGame;
            }

            if main_menu_button(ui, "Quit") {
                exit.send(AppExit);
            }
        });

        ui.add_space(5.0);

        ui.horizontal_centered(|ui| {
            ui.small(format!("{} {} ({VERSION_STRING})", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));
        });
    });
}