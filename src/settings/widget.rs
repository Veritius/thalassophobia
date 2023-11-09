use bevy_egui::egui;
use super::{Settings, options::GraphicsLevel};

impl Settings {
    /// (Egui) Lays out a menu for editing the game's settings.
    pub fn draw(&mut self, ui: &mut egui::Ui) {
        egui::Grid::new("menu_settings_grid")
        .show(ui, |ui| {
            ui.label("Model detail");
            self.model_detail.draw("menu_settings_model_detail", ui)
            .on_hover_text("The detail of on-screen models and geometry. Significantly affects FPS.");
            ui.end_row();

            ui.label("Texture quality");
            self.texture_quality.draw("menu_settings_texture_quality", ui)
            .on_hover_text("The detail of textures. Affects VRAM usage significantly.");
            ui.end_row();

            ui.label("Main volume");
            ui.add(egui::Slider::new(&mut self.main_volume, 0.0..=1.0)
            .show_value(false))
            .on_hover_text("Overall audio levels for the entire game.");
            ui.end_row();
        });

        ui.collapsing("Advanced", |ui| {
            egui::Grid::new("menu_advanced_settings_grid")
            .show(ui, |ui| {
                ui.label("Particle quality");
                self.particle_quality.draw("menu_settings_particle_quality", ui)
                .on_hover_text("The quality and density of particle effects. Slightly affects FPS.");
                ui.end_row();

                ui.label("Shader quality");
                self.shader_quality.draw("menu_settings_shader_quality", ui)
                .on_hover_text("The quality of shader effects. Dramatically affects FPS.");
                ui.end_row();

                ui.label("LOD aggression");
                ui.add(egui::Slider::new(&mut self.lod_aggression, -1.0..=1.0)
                .show_value(false))
                .on_hover_text("Modifies the system that reduces model and texture quality on objects distant from the camera.");
                ui.end_row();
            });
        });
    }
}

impl GraphicsLevel {
    fn draw(&mut self, id_source: impl std::hash::Hash, ui: &mut egui::Ui) -> egui::Response {
        egui::ComboBox::new(id_source, "")
        .selected_text(format!("{}", self))
        .show_ui(ui, |ui| {
            ui.selectable_value(self, GraphicsLevel::Low, "Low");
            ui.selectable_value(self, GraphicsLevel::Medium, "Medium");
            ui.selectable_value(self, GraphicsLevel::High, "High");        
        }).response
    }
}