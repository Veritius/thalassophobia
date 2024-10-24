use std::any::TypeId;
use aery::edges::{EdgeInfo, Edges};
use bevy::prelude::*;
use bevy_egui::egui;
use crate::devtools::DevtoolLayout;
use super::SuppliesEnergy;

pub(super) fn setup(app: &mut App) {
    app.observe(link_overlay);
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum LinkOverlay {
    #[default]
    Disabled,
    Enabled,
}

fn link_overlay(
    mut trigger: Trigger<DevtoolLayout>,
    mut state: Local<LinkOverlay>,

    hosts: Query<(&GlobalTransform, Edges<SuppliesEnergy>)>,
    mut gizmos: Gizmos,
) {
    // Combo box for changing the overlay visibility
    egui::ComboBox::new(TypeId::of::<LinkOverlay>(), "Show power links")
    .wrap_mode(egui::TextWrapMode::Extend)
    .selected_text(match *state {
        LinkOverlay::Disabled => "Disabled",
        LinkOverlay::Enabled => "Enabled",
    })
    .show_ui(&mut trigger.event_mut().ui, |ui| {
        ui.selectable_value(&mut *state, LinkOverlay::Disabled, "Disabled");
        ui.selectable_value(&mut *state, LinkOverlay::Enabled, "Enabled");
    });

    match *state {
        // Do nothing, overlay is disabled
        LinkOverlay::Disabled => { return },

        // Draw lines between connected devices
        LinkOverlay::Enabled => {
            for (host, relations) in hosts.iter() {
                for target in relations.targets() {
                    let (target, _) = match hosts.get(*target) {
                        Ok(v) => v,
                        Err(_) => continue, // Next item
                    };
        
                    gizmos.line(
                        host.translation(),
                        target.translation(),
                        Hsva::hsv(1.0 / 3.0, 1.0, 1.0),
                    );
                }
            }
        },
    }
}