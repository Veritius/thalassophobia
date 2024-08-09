pub use crate::avian::prelude::*;

pub const PHYS_LAYER_TERRAIN   : LayerMask = LayerMask(1);
pub const PHYS_LAYER_STRUCTURE : LayerMask = LayerMask(2);
pub const PHYS_LAYER_VESSEL    : LayerMask = LayerMask(4);
pub const PHYS_LAYER_CHARACTER : LayerMask = LayerMask(8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum DominancePreset {
    Terrain = 126,
    Structure = 110,
    Vessel = 94,
}

impl From<DominancePreset> for Dominance {
    #[inline]
    fn from(value: DominancePreset) -> Self {
        Dominance(value as i8)
    }
}