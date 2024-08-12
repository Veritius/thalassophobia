use std::ops::{Add, Div, Mul, Sub};
use crate::prelude::*;

macro_rules! unit {
    {
        name: $name:ident,
        doc: $doc:expr,
        aliases: [$($alias:literal),*],
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
    };
}

unit! {
    name: Energy,
    doc: "A unit of energy.",
    aliases: [ "J", "Joule" ],
}

unit! {
    name: Current,
    doc: "A unit of current.",
    aliases: [ "mA", "Milliampere" ],
}

unit! {
    name: Force,
    doc: "A unit of force.",
    aliases: [ "mN", "Millinewton" ],
}

unit! {
    name: Length,
    doc: "A unit of length.",
    aliases: [ "mm", "Millimeter" ],
}

unit! {
    name: Area,
    doc: "A unit of area",
    aliases: [ "mL^2", "Square millimeter" ],
}

unit! {
    name: Volume,
    doc: "A unit of weight.",
    aliases: [ "mL", "Milliliter" ],
}

unit! {
    name: Weight,
    doc: "A unit of weight.",
    aliases: [ "G", "Gram" ],
}

unit! {
    name: Density,
    doc: "A unit of density, derived from mass and volume.",
    aliases: [ "mG/mL" ],
}

unit! {
    name: Pressure,
    doc: "A measurement of pressure, derived from force and area.",
    aliases: [ "mPa", "Millipascal" ],
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

macro_rules! overload {
    (mul ($a:ty, $b:ty) -> $o:ty) => {
        impl Mul<$b> for $a {
            type Output = $o;

            fn mul(self, rhs: $b) -> Self::Output {
                <$o>::new(self.inner() * rhs.inner())
            }
        }
    };

    (div ($a:ty, $b:ty) -> $o:ty) => {
        impl Div<$b> for $a {
            type Output = $o;

            fn div(self, rhs: $b) -> Self::Output {
                <$o>::new(self.inner() / rhs.inner())
            }
        }
    };
}

overload!(div (Force, Area) -> Pressure);
overload!(div (Weight, Volume) -> Density);
overload!(mul (Density, Volume) -> Weight);
overload!(mul (Pressure, Area) -> Force);