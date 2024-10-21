use bevy::prelude::*;
use bevy_egui::egui;
use bevy_inspector_egui::bevy_inspector;
use super::DevtoolsWidget;


pub(super) struct WorldEntitiesInspector;

impl Default for WorldEntitiesInspector {
    fn default() -> Self {
        Self {}
    }
}

impl DevtoolsWidget for WorldEntitiesInspector {
    fn title(
        &mut self,
        _world: &mut World,
    ) -> egui::WidgetText {
        egui::WidgetText::from("Entities")
    }

    fn ui(
        &mut self,
        world: &mut World,
        ui: &mut egui::Ui,
    ) {
        bevy_inspector::ui_for_world_entities(
            world,
            ui
        );
    }
}

pub(super) struct SingleEntityInspector {
    pub entity: Entity,
}

impl DevtoolsWidget for SingleEntityInspector {
    fn title(
        &mut self,
        world: &mut World,
    ) -> egui::WidgetText {
        bevy_inspector::guess_entity_name(
            world,
            self.entity,
        ).into()
    }

    fn ui(
        &mut self,
        world: &mut World,
        ui: &mut egui::Ui,
    ) {
        bevy_inspector::ui_for_entity(
            world,
            self.entity,
            ui,
        );
    }
}