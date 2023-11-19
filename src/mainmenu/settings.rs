use bevy::{prelude::*, reflect::TypeRegistry};
use bevy_egui::{egui, EguiContexts};
use crate::settings::*;
use super::MainMenuPage;

// TODO: The settings menu is dependent on bevy_inspector_egui
// which means it doesn't work when the 'dev' feature is disabled

pub(super) fn settings_menu_system(
    mut contexts: EguiContexts,
    type_registry: Res<AppTypeRegistry>,
    mut menu: ResMut<MainMenuPage>,

    mut graphics: ResMut<GraphicsSettings>,
    mut controls: ResMut<ControlsSettings>,
    mut audio: ResMut<AudioSettings>,
) {
    let ctx = contexts.ctx_mut();
    let type_registry = &*type_registry.read();

    egui::Window::new("Settings")
    .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::splat(0.0))
    .default_size(egui::Vec2::splat(0.0))
    .resizable(false)
    .collapsible(false)
    .movable(false)
    .show(ctx, |ui| {
        #[cfg(not(feature="dev"))] {
            ui.label("The settings menu is not functional on non-dev builds right now.");
            return;
        }

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading("Graphics");
                egui::Grid::new("settings_grid_graphics")
                .show(ui, |ui| {
                    setting_reflect(ui, type_registry, "Model quality", &mut graphics.model_detail);
                    setting_reflect(ui, type_registry, "Texture quality", &mut graphics.texture_quality);
                    setting_reflect(ui, type_registry, "Lighting quality", &mut graphics.lighting_quality);
                    setting_reflect(ui, type_registry, "Particle quality", &mut graphics.particle_quality);
                    setting_reflect(ui, type_registry, "Shader quality", &mut graphics.shader_quality);
                    setting_reflect(ui, type_registry, "LOD aggression", &mut graphics.lod_aggression);
                });
            });

            ui.vertical(|ui| {
                ui.heading("Controls");
                egui::Grid::new("settings_grid_controls")
                .show(ui, |ui| {
                    #[cfg(feature="dev")] {
                        setting_reflect(ui, type_registry, "Developer menu", &mut controls.toggle_dev_menu);
                    }

                    setting_reflect(ui, type_registry, "Walk forward", &mut controls.walk_forward);
                    setting_reflect(ui, type_registry, "Walk backward", &mut controls.walk_backward);
                    setting_reflect(ui, type_registry, "Walk left", &mut controls.walk_left);
                    setting_reflect(ui, type_registry, "Walk right", &mut controls.walk_right);

                    setting_reflect(ui, type_registry, "Roll left", &mut controls.roll_left);
                    setting_reflect(ui, type_registry, "Roll right", &mut controls.roll_right);

                    setting_reflect(ui, type_registry, "Sprint", &mut controls.mod_sprint);
                    setting_reflect(ui, type_registry, "Crouch", &mut controls.mod_crouch);

                    setting_reflect(ui, type_registry, "Primary interact", &mut controls.action_primary);
                    setting_reflect(ui, type_registry, "Secondary interact", &mut controls.action_secondary);
                });
            });

            ui.vertical(|ui| {
                ui.heading("Audio");
                egui::Grid::new("settings_grid_audio")
                .show(ui, |ui| {
                    setting_reflect(ui, type_registry, "Master", &mut audio.level_master);
                });
            });
        });

        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Done").clicked() {
                *menu = MainMenuPage::FrontPage;
            }
            if ui.button("Reset").clicked() {
                *graphics = GraphicsSettings::default();
                *controls = ControlsSettings::default();
                *audio = AudioSettings::default();
            }
        });
    });
}

fn setting_reflect(
    ui: &mut egui::Ui,
    type_registry: &TypeRegistry,
    label: &'static str,
    value: &mut dyn Reflect,
) {
    ui.label(label);
    ui.push_id(format!("settings_{label}"), |ui| {
        #[cfg(feature="dev")]
        bevy_inspector_egui::reflect_inspector::ui_for_value(value, ui, type_registry);
        #[cfg(not(feature="dev"))]
        unimplemented!()
    });
    ui.end_row();
}