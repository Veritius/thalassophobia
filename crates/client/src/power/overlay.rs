use shared::prelude::*;
use crate::devstuff::{register_overlay, if_overlay_enabled};

pub(super) fn setup_overlays(
    app: &mut App,
) {
    register_overlay::<PowerLinksOverlay>(app, "power_links");

    app.add_systems(PostUpdate, overlay_system
        .run_if(if_overlay_enabled::<PowerLinksOverlay>()));
}

struct PowerLinksOverlay;

fn overlay_system(
    gizmos: Gizmos,
) {

}