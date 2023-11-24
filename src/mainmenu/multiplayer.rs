use bevy::prelude::*;
use bevy_egui::egui::*;

pub(super) fn toggle_multiplayer_button(
    ui: &mut Ui,
    commands: &mut Commands,
    multiplayer: &mut Option<ResMut<MultiplayerHostConfig>>,
) {
    let enabled = multiplayer.is_some();
    if ui.button(match enabled {
        true => "Disable multiplayer",
        false => "Enable multiplayer",
    }).clicked() {
        match enabled {
            true => commands.remove_resource::<MultiplayerHostConfig>(),
            false => commands.init_resource::<MultiplayerHostConfig>(),
        }
    }
}

#[derive(Debug, Resource)]
pub(super) struct MultiplayerHostConfig {
    pub ports: Vec<u16>,
}

impl Default for MultiplayerHostConfig {
    fn default() -> Self {
        Self {
            ports: vec![24434]
        }
    }
}

impl Widget for &mut MultiplayerHostConfig {
    fn ui(self, ui: &mut Ui) -> Response {
        Frame::default().show(ui, |ui| {
            ui.label(RichText::new("Multiplayer").size(16.0));
            ui.add_space(3.0);

            Grid::new("multiplayer_cfg_grid")
            .show(ui, |ui| {
                ui.label("Ports");
                ui.horizontal(|ui| {
                    for port in self.ports.iter_mut() {
                        ui.add(DragValue::new(port).clamp_range(1024..=u16::MAX));
                    }
                    if ui.button("+").clicked() {
                        self.ports.push(24434);
                    }
                });
                ui.end_row();
            });
        }).response
    }
}