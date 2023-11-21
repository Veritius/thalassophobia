//! Things that shoot, and the bits that let that happen.

mod ammo;
mod firing;

use bevy::prelude::*;

pub(super) fn setup_guns(app: &mut App) {
    app.register_type::<ammo::AmmoCartridge>();
    app.register_type::<ammo::AmmoProvider>();
    app.register_type::<firing::FiringPiece>();
}