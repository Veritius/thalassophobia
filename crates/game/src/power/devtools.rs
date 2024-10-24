use aery::edges::{EdgeInfo, Edges};
use bevy::prelude::*;
use bevy_egui::egui;
use crate::devtools::DevtoolLayout;
use super::SuppliesEnergy;

pub(super) fn setup(app: &mut App) {
    app.observe(link_overlay);
}

struct PowerToolsMemory {
    show_links: bool,
    show_link_arrows: bool,
}

impl Default for PowerToolsMemory {
    fn default() -> Self {
        Self {
            show_links: false,
            show_link_arrows: true,
        }
    }
}

fn link_overlay(
    mut trigger: Trigger<DevtoolLayout>,
    mut state: Local<PowerToolsMemory>,

    hosts: Query<(&GlobalTransform, Edges<SuppliesEnergy>)>,
    mut gizmos: Gizmos,
) {
    let ui = &mut trigger.event_mut().ui;

    ui.collapsing("Electricity", |ui| {
        ui.add(egui::Checkbox::new(&mut state.show_links, "Show links between devices"));
        ui.add_enabled(state.show_links, egui::Checkbox::new(&mut state.show_link_arrows, "Show arrow heads on links"));
    });

    if state.show_links {
        for (host, relations) in hosts.iter() {
            for target in relations.targets() {
                let (target, _) = match hosts.get(*target) {
                    Ok(v) => v,
                    Err(_) => continue, // Next item
                };

                if state.show_link_arrows {
                    gizmos.arrow(
                        host.translation(),
                        target.translation(),
                        Hsva::hsv(1.0 / 3.0, 1.0, 1.0),
                    );
                } else {
                    gizmos.line(
                        host.translation(),
                        target.translation(),
                        Hsva::hsv(1.0 / 3.0, 1.0, 1.0),
                    );
                }
            }
        }
    }
}