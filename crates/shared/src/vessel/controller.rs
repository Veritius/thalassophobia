use bevy::prelude::*;
use super::{RotationSet, TranslateSet};

/// The 'style' of movement for vessels.
#[derive(Default, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum VesselMoveStyle {
    /// The vessel will not attempt to correct any movements.
    #[default]
    Manual,

    /// The vessel will maintain speed and course.
    Maintain,
}

/// A controller for a submarine.
#[derive(Debug, Component, Reflect)]
pub struct VesselController {
    /// The current movement style of the vessel.
    pub move_style: VesselMoveStyle,

    /// The maximum amount of force that can be applied to translate the vessel.
    pub translate_force: TranslateSet<f32>,

    /// The maximum amount of force that can be applied to rotate the vessel.
    pub rotation_force: RotationSet<f32>,
}