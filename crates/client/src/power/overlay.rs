use shared::prelude::*;
use shared::power::*;
use shared::relations::edges::{Edges, EdgeInfo};
use crate::devstuff::overlays::*;

pub(super) fn setup_overlays(app: &mut App) {
    app.register_overlay::<PowerLinksOverlay, _, _>(power_links_system);
}

pub(super) struct PowerLinksOverlay;

impl DevOverlay for PowerLinksOverlay {
    const NAME: &'static str = "power_links";
}

fn power_links_system(
    mut gizmos: Gizmos,

    all: Query<(&GlobalTransform, Edges<SuppliesEnergy>)>,
    // sources: Query<((&PowerSource, &Transform), Relations<SuppliesEnergy>)>,
    // sinks: Query<((&PowerSink, &Transform), Relations<SuppliesEnergy>)>,
    // storage: Query<((&PowerStorage, &Transform), Relations<SuppliesEnergy>)>,
) {
    for (origin, relations) in all.iter() {
        for target in relations.targets() {
            let (target, _) = match all.get(*target) {
                Ok(v) => v,
                Err(_) => continue, // Next item
            };

            gizmos.line(
                origin.translation(),
                target.translation(),
                Hsva::hsv(1.0 / 3.0, 1.0, 1.0),
            );
        }
    }
}