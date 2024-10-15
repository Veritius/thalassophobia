use std::any::Any;
use bevy_egui::{egui, EguiContexts};
use shared::{bevy::utils::HashMap, prelude::*};

#[derive(Default, Resource)]
pub(super) struct OverlayRegistry {
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


pub(crate) trait DevOverlay: Any {
    const NAME: &'static str;
}

pub(crate) trait OverlayAppExt: sealed::Sealed {
    fn register_overlay<T: DevOverlay, M, P>(&mut self, system: P)
    where
        M: 'static,
        P: IntoSystemConfigs<M>;
}

impl OverlayAppExt for App {
    fn register_overlay<T: DevOverlay, M, P>(&mut self, system: P)
    where
        M: 'static,
        P: IntoSystemConfigs<M>,
    {
        // Register the overlay
        let mut overlays = self.world_mut().resource_mut::<OverlayRegistry>();
        let type_id = TypeId::of::<T>();
        overlays.names.insert(T::NAME, type_id);
        overlays.enabled.insert(type_id, false);

        // Add the system that displays it
        self.add_systems(PostUpdate, system
            .run_if(|overlays: Res<OverlayRegistry>| *overlays.enabled.get(&TypeId::of::<T>()).unwrap()));
    }
}

mod sealed {
    pub trait Sealed {}
    impl Sealed for super::App {}
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