use crate::{math::{curve::FloatCurve, transform::TranslateSet}, prelude::*};

/// Drag applied to the vessel as it moves.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct VesselDrag {
    pub curves: TranslateSet<FloatCurve>,
}