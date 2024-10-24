use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContext};
use crate::initialisation::Initialisation;

pub struct DevtoolsPlugin;

impl Plugin for DevtoolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egui::EguiPlugin);
    }
}