use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::reflect_inspector::ui_for_value;
use crate::settings::*;
use super::MainMenuPage;

// TODO: This sucks, make it better.

pub(super) fn settings_menu_system(
    mut contexts: EguiContexts,
    type_registry: Res<AppTypeRegistry>,
    mut menu: ResMut<MainMenuPage>,

    mut graphics: ResMut<Graphics>,
    mut controls: ResMut<Controls>,
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
        ui.vertical(|ui| {
            egui::Grid::new("settings_grid")
            .show(ui, |ui| {
                ui.label("Model detail");
                graphics_level_combobox(ui, "settings_model_detail", &mut graphics.model_detail);
                ui.end_row();

                ui.label("Texture quality");
                graphics_level_combobox(ui, "settings_texture_quality", &mut graphics.texture_quality);
                ui.end_row();

                ui.label("Lighting quality");
                graphics_level_combobox(ui, "settings_lighting_quality", &mut graphics.lighting_quality);
                ui.end_row();

                ui.label("Particle quality");
                graphics_level_combobox(ui, "settings_particle_quality", &mut graphics.particle_quality);
                ui.end_row();

                ui.label("Shader quality");
                graphics_level_combobox(ui, "settings_shader_quality", &mut graphics.shader_quality);
                ui.end_row();

                ui.label("LOD aggression");
                ui.add(egui::Slider::new(&mut graphics.lod_aggression, -1.0..=1.0).show_value(false));
                ui.end_row();

                #[cfg(feature="dev")] {
                    ui.label("Developer menu");
                    ui.push_id("settings_toggle_dev_menu", |ui| { ui_for_value(&mut controls.toggle_dev_menu, ui, type_registry) });
                    ui.end_row();
                }

                ui.label("Walk forward");
                ui.push_id("settings_walk_forward", |ui| { ui_for_value(&mut controls.walk_forward, ui, type_registry) });
                ui.end_row();

                ui.label("Walk backward");
                ui.push_id("settings_walk_backward", |ui| { ui_for_value(&mut controls.walk_backward, ui, type_registry) });
                ui.end_row();

                ui.label("Strafe left");
                ui.push_id("settings_strafe_left", |ui| { ui_for_value(&mut controls.strafe_left, ui, type_registry) });
                ui.end_row();

                ui.label("Strafe right");
                ui.push_id("settings_strafe_right", |ui| { ui_for_value(&mut controls.strafe_right, ui, type_registry) });
                ui.end_row();

                ui.label("Sprint");
                ui.push_id("settings_sprint", |ui| { ui_for_value(&mut controls.mod_sprint, ui, type_registry) });
                ui.end_row();

                ui.label("Crouch");
                ui.push_id("settings_crouch", |ui| { ui_for_value(&mut controls.mod_crouch, ui, type_registry) });
                ui.end_row();

                ui.label("Primary action");
                ui.push_id("settings_action_primary", |ui| { ui_for_value(&mut controls.action_primary, ui, type_registry) });
                ui.end_row();

                ui.label("Secondary action");
                ui.push_id("settings_action_secondary", |ui| { ui_for_value(&mut controls.action_secondary, ui, type_registry) });
                ui.end_row();
            });
        });
        ui.separator();
        ui.horizontal(|ui| {
            if ui.button("Done").clicked() {
                *menu = MainMenuPage::FrontPage;
            }
            if ui.button("Reset").clicked() {
                *graphics = Graphics::default();
                *controls = Controls::default();
            }
        });
    });
}

fn graphics_level_combobox(
    ui: &mut egui::Ui,
    id: impl std::hash::Hash,
    value: &mut graphics::GraphicsLevel
) {
    use graphics::GraphicsLevel::*;
    egui::ComboBox::new(id, "")
    .selected_text(format!("{value}"))
    .show_ui(ui, |ui| {
        ui.selectable_value(value, High, "High");
        ui.selectable_value(value, Medium, "Medium");
        ui.selectable_value(value, Low, "Low");
    });
}