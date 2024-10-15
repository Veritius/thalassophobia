use shared::prelude::*;
use crate::devstuff::overlays::*;

pub(super) fn setup_overlays(app: &mut App) {
    // app.register_overlay::<PowerLinksOverlay>();
}

// pub(super) struct PowerLinksOverlay;

// impl DevOverlay for PowerLinksOverlay {
//     const NAME: &'static str = "power_links";

//     fn system<M, S: IntoSystemConfigs<M>>() -> S {
//         power_links_system
//     }
// }

// fn power_links_system(
//     mut gizmos: Gizmos,
// ) {

// }