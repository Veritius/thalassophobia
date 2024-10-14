use std::any::Any;
use bevy_egui::{egui, EguiContexts};
use shared::{bevy::utils::HashMap, prelude::*};

pub(crate) fn register_overlay<T: Any>(app: &mut App, name: &'static str) {
    app.world_mut().resource_mut::<OverlayRegistry>().enabled.insert(
        TypeId::of::<T>(),
        false,
    );
}

pub(crate) fn if_overlay_enabled<T: Any>() -> impl FnMut(Res<'_, OverlayRegistry>) -> bool + Clone {
    |reg| *(reg.enabled.get(&TypeId::of::<T>()).unwrap())
}

#[derive(Default, Resource)]
pub(crate) struct OverlayRegistry {
    names: HashMap<&'static str, TypeId>,
    enabled: BTreeMap<TypeId, bool>,
}

impl OverlayRegistry {
    fn split_borrow(&mut self) -> (
        &mut HashMap<&'static str, TypeId>,
        &mut BTreeMap<TypeId, bool>,
    ) {
        (
            &mut self.names,
            &mut self.enabled,
        )
    }
}

#[derive(Resource, Default, Reflect, PartialEq, Eq)]
#[reflect(Resource)]
pub(super) enum OverlayWindowVisibility {
    Show,
    #[default]
    Hide,
}

pub(super) fn overlay_window(
    mut ctx: EguiContexts,
    visibility: Res<OverlayWindowVisibility>,
    mut overlays: ResMut<OverlayRegistry>,
) {
    if *visibility == OverlayWindowVisibility::Hide { return; }

    egui::Window::new("Overlays").show(ctx.ctx_mut(), |ui| {
        egui::Grid::new("infodump_statistics")
        .striped(true)
        .show(ui, |ui| {
            let (names, enabled) = overlays.split_borrow();

            for (name, id) in names.iter() {
                let value = enabled.get_mut(id).unwrap();

                ui.label(*name);
                ui.checkbox(value, "");
                ui.end_row();
            }
        });
    });
}