use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::settings::Settings;

pub(super) fn settings_menu_system(
    mut contexts: EguiContexts,
    mut settings: ResMut<Settings>,
) {
    let ctx = contexts.ctx_mut();

    egui::Window::new("Settings").show(ctx, |ui| {
        settings.draw(ui);
    });
}