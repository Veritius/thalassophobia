use bevy_inspector_egui::bevy_inspector;
use super::*;

#[inline(always)]
pub(super) fn setup(app: &mut App) {
    app.init_resource::<OpenInspectorsState>();

    app.add_systems(Update, inspector_system
        .after(layout_devtool_ui));

    app.observe(observer_system);
}

fn observer_system(
    mut trigger: Trigger<DevtoolLayout>,
    mut state: ResMut<OpenInspectorsState>,
) {
    let ui = &mut trigger.event_mut().ui;

    ui.collapsing("Inspectors", |ui| {
        ui.checkbox(&mut state.resources, "Show resources inspector");
        ui.checkbox(&mut state.entities, "Show entities inspector");
        ui.checkbox(&mut state.assets, "Show assets inspector");
    });
}

fn inspector_system(
    world: &mut World,
) {
    // Clone the state out of the World (so we don't have to borrow)
    // Also check if we need to return early because there's no inspectors to show anyway
    let mut state = world.resource::<OpenInspectorsState>().clone();
    if !state.resources && !state.entities && !state.assets { return }

    // Early return if there's no window that we can use.
    let mut ctx = match world
    .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
    .get_single(world) {
        Ok(v) => v.clone(),
        Err(_) => { return },
    };

    if state.resources {
        egui::Window::new("Resources")
        .open(&mut state.resources)
        .show(ctx.get_mut(), |ui| {
            bevy_inspector::ui_for_resources(world, ui);
        });
    }

    if state.entities {
        egui::Window::new("Entities")
        .open(&mut state.entities)
        .show(ctx.get_mut(), |ui| {
            bevy_inspector::ui_for_world_entities(world, ui);
        });
    }

    if state.assets {
        egui::Window::new("Assets")
        .open(&mut state.assets)
        .show(ctx.get_mut(), |ui| {
            bevy_inspector::ui_for_all_assets(world, ui);
        });
    }

    // Update the World's copy of the state, in case we've made changes
    *world.resource_mut::<OpenInspectorsState>() = state;
}

#[derive(Resource, Clone)]
struct OpenInspectorsState {
    resources: bool,
    entities: bool,
    assets: bool,
}

impl Default for OpenInspectorsState {
    fn default() -> Self {
        Self {
            resources: false,
            entities: false,
            assets: false,
        }
    }
}