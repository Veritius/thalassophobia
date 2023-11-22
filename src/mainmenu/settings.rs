use std::ops::RangeInclusive;

use bevy::{prelude::*, reflect::TypeRegistry};
use bevy_egui::{egui::{self, emath::Numeric}, EguiContexts};
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
                    reflect_combobox(ui, type_registry, "Model quality", &mut graphics.model_detail);
                    reflect_combobox(ui, type_registry, "Texture quality", &mut graphics.texture_quality);
                    reflect_combobox(ui, type_registry, "Lighting quality", &mut graphics.lighting_quality);
                    reflect_combobox(ui, type_registry, "Particle quality", &mut graphics.particle_quality);
                    reflect_combobox(ui, type_registry, "Shader quality", &mut graphics.shader_quality);

                    graphics_slider(ui, "LOD aggression", &mut graphics.lod_aggression, -10.0..=10.0)
                });
            });

            ui.vertical(|ui| {
                ui.heading("Controls");
                egui::Grid::new("settings_grid_controls")
                .show(ui, |ui| {
                    #[cfg(feature="dev")] {
                        reflect_combobox(ui, type_registry, "Developer menu", &mut controls.toggle_dev_menu);
                    }

                    reflect_combobox(ui, type_registry, "Walk forward", &mut controls.walk_forward);
                    reflect_combobox(ui, type_registry, "Walk backward", &mut controls.walk_backward);
                    reflect_combobox(ui, type_registry, "Walk left", &mut controls.walk_left);
                    reflect_combobox(ui, type_registry, "Walk right", &mut controls.walk_right);

                    reflect_combobox(ui, type_registry, "Roll/lean left", &mut controls.roll_left);
                    reflect_combobox(ui, type_registry, "Roll/lean right", &mut controls.roll_right);

                    reflect_combobox(ui, type_registry, "Sprint", &mut controls.sprint);
                    reflect_combobox(ui, type_registry, "Ascend / Jump", &mut controls.ascend);
                    reflect_combobox(ui, type_registry, "Descend / Crouch", &mut controls.descend);

                    reflect_combobox(ui, type_registry, "Primary interact", &mut controls.action_primary);
                    reflect_combobox(ui, type_registry, "Secondary interact", &mut controls.action_secondary);
                });
            });

            ui.vertical(|ui| {
                ui.heading("Audio");
                egui::Grid::new("settings_grid_audio")
                .show(ui, |ui| {
                    audio_slider(ui, "Master volume", &mut audio.level_master);
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

fn reflect_combobox(
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

fn graphics_slider<T: Numeric>(
    ui: &mut egui::Ui,
    label: &'static str,
    value: &mut T,
    range: RangeInclusive<T>,
) {
    ui.label(label);
    ui.add(egui::Slider::new(value, range).show_value(false));
    ui.end_row();
}

fn audio_slider(
    ui: &mut egui::Ui,
    label: &'static str,
    value: &mut f32,
) {
    ui.label(label);
    ui.add(egui::Slider::new(value, 0.0..=1.0));
    ui.end_row();
}