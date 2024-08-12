use std::ops::{Add, Div, Mul, Sub};
use crate::prelude::*;

macro_rules! unit {
    {
        name: $name:ident,
        doc: $doc:expr,
        aliases: [$($alias:literal),*],
        conversions: [$(($($in_nm:ident: $in_ty:ty),*) { $expr:expr }),*],
    } => {
        #[doc=$doc]
        #[doc(alias($($alias),*))]
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect, Serialize, Deserialize)]
        #[reflect(Serialize, Deserialize)]
        pub struct $name(u32);

        impl $name {
            #[inline]
            pub const fn new(value: u32) -> Self {
                Self(value)
            }

            #[inline]
            pub const fn inner(self) -> u32 {
                self.0
            }
        }

        impl From<u32> for $name {
            #[inline]
            fn from(value: u32) -> Self {
                Self::new(value)
            }
        }
        
        impl From<$name> for u32 {
            fn from(value: $name) -> Self {
                value.inner()
            }
        }
        
        impl Add for $name {
            type Output = Self;
        
            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0.saturating_add(rhs.0))
            }
        }
        
        impl Sub for $name {
            type Output = Self;
        
            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0.saturating_sub(rhs.0))
            }
        }
        
        impl Mul<f32> for $name {
            type Output = Self;
        
            #[inline]
            fn mul(self, rhs: f32) -> Self::Output {
                let v = self.0 as f32 * rhs;
                return Self(v as u32);
            }
        }
        
        impl Div<f32> for $name {
            type Output = Self;
        
            #[inline]
            fn div(self, rhs: f32) -> Self::Output {
                let v = self.0 as f32 / rhs;
                return Self(v as u32);
            }
        }

        $(
            conv!(from ($($in_nm: $in_ty),*) for $name { $expr });
        ),*
    };
}

macro_rules! conv {
    (from ($($in_nm:ident: $in_ty:ty),*) for $out:ty { $op:expr }) => {
        impl From<($($in_ty),*)> for $out {
            fn from(value: ($($in_ty),*)) -> $out {
                let ($($in_nm),*) = value;
                let ($($in_nm),*) = ($($in_nm.inner()),*);
                return <$out>::new($op);
            }
        }
    };
}

unit! {
    name: Energy,
    doc: "A unit of energy.",
    aliases: [ "J", "Joule" ],
    conversions: [],
}

unit! {
    name: Current,
    doc: "A unit of current.",
    aliases: [ "mA", "Milliampere" ],
    conversions: [],
}

unit! {
    name: Force,
    doc: "A unit of force.",
    aliases: [ "mN", "Millinewton" ],
    conversions: [
        (pressure: Pressure, area: Area) { pressure * area }
    ],
}

unit! {
    name: Length,
    doc: "A unit of length.",
    aliases: [ "mm", "Millimeter" ],
    conversions: [],
}

unit! {
    name: Area,
    doc: "A unit of area",
    aliases: [ "mL^2", "Square millimeter" ],
    conversions: [],
}

unit! {
    name: Volume,
    doc: "A unit of weight.",
    aliases: [ "mL", "Milliliter" ],
    conversions: [],
}

unit! {
    name: Weight,
    doc: "A unit of weight.",
    aliases: [ "mG", "Milligram" ],
    conversions: [
        (density: Density, volume: Volume) { density * volume }
    ],
}

unit! {
    name: Density,
    doc: "A unit of density, derived from mass and volume.",
    aliases: [ "mG/mL" ],
    conversions: [
        (weight: Weight, volume: Volume) { weight.checked_div(volume).unwrap_or(0) }
    ],
}

unit! {
    name: Pressure,
    doc: "A measurement of pressure, derived from force and area.",
    aliases: [ "mPa", "Millipascal" ],
    conversions: [
        (force: Force, area: Area) { force.checked_div(area).unwrap_or(0) }
    ],
}

impl Density {
    /// Calculates density, without checking that `volume` is not zero.
    pub const fn new_unchecked(
        weight: Weight,
        volume: Volume,
    ) -> Self {
        let value = weight.inner() / volume.inner();
        return Density(value);
    }
}