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
    visibility: Res<InfodumpWindowVisibility>,

    shared_state: Res<State<GameState>>,
    client_state: Res<State<ClientState>>,

    diagnostics: Res<DiagnosticsStore>,
) {
    if *visibility == InfodumpWindowVisibility::Hide { return; }
    egui::Window::new("Statistics").show(ctx.ctx_mut(), |ui| {
        // Game state panel
        egui::CollapsingHeader::new("State")
        .show(ui, |ui| {
            egui::Grid::new("infodump_diagnostics")
            .striped(true)
            .show(ui, |ui| {
                ui.label("Client state");
                ui.label(format!("{}", client_state.get()));
                ui.end_row();

                ui.label("Shared state");
                ui.label(format!("{}", shared_state.get()));
                ui.end_row();
            });
        });

        // Diagnostics panel
        egui::CollapsingHeader::new("Diagnostics")
        .show(ui, |ui| {
            egui::ScrollArea::vertical()
            .show(ui, |ui| {
                egui::Grid::new("infodump_diagnostics")
                .striped(true)
                .show(ui, |ui| {
                    let count = diagnostics.iter().count();
                    let mut vec = Vec::with_capacity(count);

                    for diagnostic in diagnostics.iter() {
                        vec.push((diagnostic.path(), diagnostic.value()));
                    }

                    // Make the set ordered instead of weird and unstable
                    vec.sort_unstable_by(|(a,_),(b,_)| a.as_str().cmp(b.as_str()));

                    for (name, value) in vec.drain(..) {
                        ui.label(name.as_str());
                        if let Some(value) = value {
                            const EPSILON_A: f64 = 0.001;
                            const EPSILON_B: f64 = 0.999;

                            let frac = value % 1.0;

                            if frac < EPSILON_A || frac > EPSILON_B {
                                ui.label(format!("{value:.0}"));
                            } else {
                                ui.label(format!("{value:.3}"));
                            }
                        }
                        ui.end_row();
                    }
                });
            });
        });
    });
}