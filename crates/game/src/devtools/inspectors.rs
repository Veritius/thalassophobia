use std::collections::BTreeSet;
use bevy_inspector_egui::bevy_inspector;
use egui::Id;
use super::*;

#[inline(always)]
pub(super) fn setup(app: &mut App) {
    app.init_resource::<Inspectors>();

    app.add_systems(Update, inspector_system
        .after(layout_devtool_ui));

    app.observe(observer_system);
}

fn observer_system(
    mut trigger: Trigger<DevtoolLayout>,
    mut state: ResMut<Inspectors>,
) {
    let ui = &mut trigger.event_mut().ui;

    egui::CollapsingHeader::new("Inspectors")
    .show(ui, |ui| {
        ui.checkbox(&mut state.show_world_resources, "Resources");
        ui.checkbox(&mut state.show_world_entities, "Entities");
        ui.checkbox(&mut state.show_world_assets, "Assets");

        ui.add_space(4.0);

        egui::CollapsingHeader::new("Individual entities")
        .show(ui, |ui| {
            egui::Grid::new("devtools_inspector_entities")
            .show(ui, |ui| {
                let mut removals = vec![];

                for entity in state.inspect_entities.iter().copied() {
                    ui.label(format!("{entity}"));

                    if ui.small_button("Close").clicked() {
                        removals.push(entity);
                    }

                    ui.end_row();
                }

                for removal in removals {
                    state.inspect_entities.remove(&removal);
                }
            });
        });
    });
}

fn inspector_system(
    world: &mut World,
) {
    // Clone the state out of the World (so we don't have to borrow)
    let mut state = world.resource::<Inspectors>().clone();

    if {
        // Check if we need to return early
        let any_global_inspectors = state.show_world_resources || state.show_world_entities || state.show_world_assets;
        let any_entity_inspectors = state.inspect_entities.len() > 0;
        !any_global_inspectors && !any_entity_inspectors
    } { return }

    // Early return if there's no window that we can use.
    let mut ctx = match world
    .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
    .get_single(world) {
        Ok(v) => v.clone(),
        Err(_) => { return },
    };

    if state.show_world_resources {
        egui::Window::new("Resources")
        .open(&mut state.show_world_resources)
        .show(ctx.get_mut(), |ui| {
            egui::ScrollArea::both()
            .show(ui, |ui| {
                bevy_inspector::ui_for_resources(world, ui);
            });
        });
    }

    if state.show_world_entities {
        egui::Window::new("Entities")
        .open(&mut state.show_world_entities)
        .show(ctx.get_mut(), |ui| {
            egui::ScrollArea::both()
            .show(ui, |ui| {
                bevy_inspector::ui_for_world_entities(world, ui);
            });
        });
    }

    if state.show_world_assets {
        egui::Window::new("Assets")
        .open(&mut state.show_world_assets)
        .show(ctx.get_mut(), |ui| {
            egui::ScrollArea::both()
            .show(ui, |ui| {
                bevy_inspector::ui_for_all_assets(world, ui);
            });
        });
    }

    let mut removals = Vec::new();

    for entity in state.inspect_entities.iter().copied() {
        let mut open = true;

        egui::Window::new(bevy_inspector::guess_entity_name(world, entity))
        .id(Id::new(format!("devtool_inspector_{entity}")))
        .open(&mut open)
        .show(ctx.get_mut(), |ui| {
            egui::ScrollArea::both()
            .show(ui, |ui| {
                bevy_inspector::ui_for_entity(world, entity, ui);
            });
        });

        if !open { removals.push(entity); }
    }

    for removal in removals {
        state.inspect_entities.remove(&removal);
    }

    // Update the World's copy of the state, in case we've made changes
    *world.resource_mut::<Inspectors>() = state;
}

#[derive(Resource, Clone)]
pub(crate) struct Inspectors {
    show_world_resources: bool,
    show_world_entities: bool,
    show_world_assets: bool,

    inspect_entities: BTreeSet<Entity>,
}

impl Default for Inspectors {
    fn default() -> Self {
        Self {
            show_world_resources: false,
            show_world_entities: false,
            show_world_assets: false,

            inspect_entities: BTreeSet::new(),
        }
    }
}

impl Inspectors {
    pub fn inspect_entity(&mut self, entity: Entity) {
        self.inspect_entities.insert(entity);
    }
}