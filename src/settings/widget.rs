use bevy_egui::egui;
use super::{Settings, options::GraphicsLevel};

impl Settings {
    /// (Egui) Lays out a menu for editing the game's settings.
    pub fn draw(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("menu_settings_grid")
        .show(ui, |ui| {
            ui.label("Model detail");
            self.model_detail.draw("menu_settings_model_detail", ui);
            ui.end_row();

            ui.label("Texture quality");
            self.texture_quality.draw("menu_settings_texture_quality", ui);
            ui.end_row();

            ui.label("Particle quality");
            self.particle_quality.draw("menu_settings_particle_quality", ui);
            ui.end_row();

            ui.label("Shader quality");
            self.shader_quality.draw("menu_settings_shader_quality", ui);
            ui.end_row();

            ui.label("Main volume");
            ui.add(egui::Slider::new(&mut self.main_volume, 0.0..=1.0)
            .show_value(false));
            ui.end_row();
        });
    }
}

impl GraphicsLevel {
    fn draw(&mut self, id_source: impl std::hash::Hash, ui: &mut egui::Ui) {
        egui::ComboBox::new(id_source, "")
        .selected_text(format!("{}", self))
        .show_ui(ui, |ui| {
            ui.selectable_value(self, GraphicsLevel::Low, "Low");
            ui.selectable_value(self, GraphicsLevel::Medium, "Medium");
            ui.selectable_value(self, GraphicsLevel::High, "High");        
        });
    }
}