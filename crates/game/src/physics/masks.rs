use std::ops::BitOr;
use avian3d::prelude::*;

/// Physics layers for objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectLayer {
    Terrain,
    Structure,
    Vessel,
    Character,
}

impl BitOr for ObjectLayer {
    type Output = LayerMask;

    fn bitor(self, rhs: Self) -> Self::Output {
        LayerMask::from(self.to_bits() | rhs.to_bits())
    }
}

impl BitOr<LayerMask> for ObjectLayer {
    type Output = LayerMask;

    fn bitor(self, rhs: LayerMask) -> Self::Output {
        LayerMask::from(self.to_bits() | *rhs)
    }
}

impl PhysicsLayer for ObjectLayer {
    fn to_bits(&self) -> u32 {
        let offset = match self {
            ObjectLayer::Terrain => 0,
            ObjectLayer::Structure => 1,
            ObjectLayer::Vessel => 2,
            ObjectLayer::Character => 3,
        };

        return 1 << offset;
    }

    fn all_bits() -> u32 {
        u32::MAX
    }
}

/// Dominance values for objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum ObjectDominance {
    Terrain = 126,
    Structure = 110,
    Vessel = 94,
}

impl From<ObjectDominance> for Dominance {
    #[inline]
    fn from(value: ObjectDominance) -> Self {
        Dominance(value as i8)
    }
}