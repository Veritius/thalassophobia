use bevy_egui::*;
use shared::{bevy::{diagnostic::DiagnosticsStore, prelude::*}, bevy_ecs, bevy_reflect, state::GameState};
use crate::state::ClientState;

#[derive(Resource, Default, Reflect, PartialEq, Eq)]
#[reflect(Resource)]
pub(super) enum InfodumpWindowVisibility {
    Show,
    #[default]
    Hide,
}

pub(super) fn infodump_window(
    mut ctx: EguiContexts,
    shared_state: Res<State<GameState>>,
    client_state: Res<State<ClientState>>,
    visibility: Res<InfodumpWindowVisibility>,
    diagnostics: Res<DiagnosticsStore>,
) {
    if *visibility == InfodumpWindowVisibility::Hide { return; }
    egui::Window::new("Information Dump").show(ctx.ctx_mut(), |ui| {
        ui.label(format!("State: {} / {}", client_state.get(), shared_state.get()));

        // Diagnostics panel
        egui::CollapsingHeader::new("Diagnostics").show(ui, |ui| {
            egui::Grid::new("infodump_diagnostics").show(ui, |ui| {
                for diagnostic in diagnostics.iter() {
                    ui.label(diagnostic.path().as_str());
                    if let Some(value) = diagnostic.value() {
                        ui.label(format!("{value:.3}"));
                    }
                    ui.end_row();
                }
            });
        });
    });
}