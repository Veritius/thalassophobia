use bevy::prelude::*;
use bevy_egui::{egui::{self, Color32, RichText, WidgetText}, EguiContexts};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
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
    type Tab = Entity;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        // Try and use the `Name` component as the tab title
        if let Some(ent) = self.world.get_entity(*tab) {
            if let Some(name) = ent.get::<Name>() {
                return WidgetText::from(name.as_str());
            }
        }

        // We couldn't get the name of the tab normally
        // Instead, use the entity id's Display implementation
        return WidgetText::from(format!("{}", *tab));
    }

    fn ui(&mut self, ui: &mut egui::Ui, tab: &mut Self::Tab) {
        // Try to get the function from the tab entity
        let mut state = match self.world.get_entity_mut(*tab)
            .map(|mut e| e.take::<DevtoolsTabState>())
        {
            Some(Some(v)) => v,
            Some(None)  | None => {
                // If the entity doesn't exist, display a warning and return
                ui.label(RichText::new("Tab entity did not exist").color(Color32::RED));
                return;
            },
        };

        // Run the inner function
        state.state.ui(self.world, ui);

        // Put the component back into the entity.
        self.world.entity_mut(*tab).insert(state);
    }
}

#[derive(Component)]
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
    fn ui(
        &mut self,
        world: &mut World,
        ui: &mut egui::Ui,
    );
}