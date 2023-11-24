use bevy::prelude::*;
use bevy_egui::{egui::{self, Widget}, EguiContexts};
use super::{MainMenuPage, multiplayer::{MultiplayerHostConfig, toggle_multiplayer_button}};

pub(super) fn new_game_menu_system(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut menu: ResMut<MainMenuPage>,
    mut multiplayer: Option<ResMut<MultiplayerHostConfig>>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("New game")
    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::splat(0.0))
    .default_size(egui::Vec2::splat(0.0))
    .resizable(false)
    .collapsible(false)
    .movable(false)
    .show(ctx, |ui| {
        ui.heading("WIP :)");

        ui.separator();

        if let Some(ref mut multiplayer) = multiplayer {
            multiplayer.ui(ui);
            ui.separator();
        }

        ui.horizontal(|ui| {
            if ui.button("Cancel").clicked() {
                *menu = MainMenuPage::FrontPage;
                commands.remove_resource::<MultiplayerHostConfig>();
            }
            if ui.button("Confirm").clicked() {
                todo!()
            }

            toggle_multiplayer_button(ui, &mut commands, &mut multiplayer);
        });
    });
}

pub(super) fn load_game_menu_system(
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut menu: ResMut<MainMenuPage>,
    mut multiplayer: Option<ResMut<MultiplayerHostConfig>>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Load game")
    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::splat(0.0))
    .default_size(egui::Vec2::splat(0.0))
    .resizable(false)
    .collapsible(false)
    .movable(false)
    .show(ctx, |ui| {
        ui.heading("WIP :)");

        ui.separator();

        if let Some(ref mut multiplayer) = multiplayer {
            multiplayer.ui(ui);
            ui.separator();
        }

        ui.horizontal(|ui| {
            if ui.button("Cancel").clicked() {
                *menu = MainMenuPage::FrontPage;
                commands.remove_resource::<MultiplayerHostConfig>();
            }
            if ui.button("Confirm").clicked() {
                todo!()
            }

            toggle_multiplayer_button(ui, &mut commands, &mut multiplayer);
        });
    });
}