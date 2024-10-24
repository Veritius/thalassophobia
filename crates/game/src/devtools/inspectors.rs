use super::*;

#[inline(always)]
pub(super) fn setup(app: &mut App) {
    app.init_resource::<OpenInspectorsState>();

    app.observe(observer_system);
}

fn observer_system(
    mut trigger: Trigger<DevtoolLayout>,
    mut state: ResMut<OpenInspectorsState>,
) {
    let ui = &mut trigger.event_mut().ui;

    ui.collapsing("Inspectors", |ui| {
        todo!()
    });
}

#[derive(Resource)]
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