//! Health data, specifically for living creatures.

mod vitality;

use bevy::prelude::*;

pub(super) fn setup_health(app: &mut App) {
    app.register_type::<vitality::VitalityAggregate>();
}