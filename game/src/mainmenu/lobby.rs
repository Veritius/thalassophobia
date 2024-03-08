use bevy_egui::{EguiContexts, egui::*};

pub(super) fn lobby_menu_system(
    mut contexts: EguiContexts,
) {
    CentralPanel::default()
    .show(contexts.ctx_mut(), |ui| {
        ui.heading("Work in progress");
    });
}