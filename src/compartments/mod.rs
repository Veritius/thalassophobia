//! Compartments of water.

use bevy::prelude::*;
use serde::{Serialize, Deserialize};

pub(super) fn setup_compartments(app: &mut App) {
    app.register_type::<Compartment>();
}

/// A sealed compartment for human habitation, as well as data used to show water.
#[derive(Debug, Component, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct Compartment {
    /// The volume in of the compartment, in liters.
    pub volume: f32,
    /// The amount of water in the compartment, in liters.
    pub water: f32,
    /// How breathable the air in the compartment is, from `0.0` to `1.0`.
    pub oxygen: f32,
}

impl Compartment {
    /// Returns how much of the compartment is water, from `0.0` to `1.0`.
    /// Returns `0.0` if the compartment has volume `0.0`.
    pub fn water_volume(&self) -> f32 {
        if self.water == 0.0 { return 0.0 }
        (self.water / self.volume).clamp(0.0, 1.0)
    }
}