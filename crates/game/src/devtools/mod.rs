use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui_dock::TabViewer;
use crate::initialisation::Initialisation;

pub struct DevtoolsPlugin;

impl Plugin for DevtoolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_egui::EguiPlugin);

        // app.add_systems(Update, sidebar_system
        //     .run_if(in_state(Initialisation::Finished)));
    }
}

struct DevtoolsTabViewer<'a> {
    world: &'a mut World,
}

impl<'a> TabViewer for DevtoolsTabViewer<'a> {
    type Tab = DevtoolsTabState;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.state.title()
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        tab.state.ui(&mut self.world, ui);
    }
}

struct DevtoolsTabState {
    state: Box<dyn DevtoolsWidget>,
}

impl<S> From<S> for DevtoolsTabState
where
    S: DevtoolsWidget,
{
    fn from(value: S) -> Self {
        Self {
            state: Box::from(value),
        }
    }
}

trait DevtoolsWidget
where
    Self: Send + Sync + 'static,
{
    fn title(
        &mut self,
    ) -> egui::WidgetText;

    fn ui(
        &mut self,
        world: &mut World,
        ui: &mut egui::Ui,
    );
}