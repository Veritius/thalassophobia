use bevy_egui::*;
use shared::{bevy::diagnostic::DiagnosticsStore, initial::Initialisation, prelude::*};

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

    initialisation: Res<State<Initialisation>>,
    sim_loaded: Option<Res<State<SimulationLoaded>>>,
    sim_running: Option<Res<State<SimulationRunning>>>,

    #[cfg(feature="hosting")]
    lobby_state: Option<Res<State<server::lobbies::LobbyState>>>,

    diagnostics: Res<DiagnosticsStore>,
) {
    if *visibility == InfodumpWindowVisibility::Hide { return; }

    egui::Window::new("Statistics").show(ctx.ctx_mut(), |ui| {
        // Game state panel
        egui::CollapsingHeader::new("State")
        .show(ui, |ui| {
            egui::Grid::new("infodump_statistics")
            .striped(true)
            .show(ui, |ui| {
                ui.label("Initialisation");
                ui.label(match *initialisation.get() {
                    Initialisation::Loading => "Loading",
                    Initialisation::Finished => "Finished",
                });
                ui.end_row();

                ui.label("Simulation");
                ui.label(match *initialisation.get() {
                    Initialisation::Loading => "Waiting for init",
                    Initialisation::Finished => match sim_loaded.unwrap().get() {
                        SimulationLoaded::Unloaded => "Unloaded",
                        SimulationLoaded::Loaded => match sim_running.unwrap().get() {
                            SimulationRunning::Paused => "Loaded",
                            SimulationRunning::Running => "Running",
                        },
                    },
                });
                ui.end_row();

                #[cfg(feature="hosting")] {
                    ui.label("Server state");
                    ui.label(match *initialisation.get() {
                        Initialisation::Loading => "Waiting for init",
                        Initialisation::Finished => match *lobby_state.unwrap().get() {
                            server::lobbies::LobbyState::Offline => "Offline",
                            server::lobbies::LobbyState::Active => "Active",
                        },
                    });
                    ui.end_row();
                }
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