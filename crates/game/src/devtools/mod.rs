use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::initialisation::Initialisation;

pub struct DevtoolsPlugin;

impl Plugin for DevtoolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egui::EguiPlugin);

        // app.add_systems(Update, sidebar_system
        //     .run_if(in_state(Initialisation::Finished)));
    }
}