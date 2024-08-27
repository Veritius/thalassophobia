use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};
use crate::prelude::*;

macro_rules! unit {
    {
        name: $name:ident,
        doc: $doc:expr,
        unit: $unit:literal,
        aliases: [$($alias:literal),*],
    } => {
        #[doc=$doc]
        #[doc(alias=$unit)]
        #[doc(alias($($alias),*))]
        #[derive(Default, Clone, Copy, PartialEq, PartialOrd, Reflect, Serialize, Deserialize)]
        #[reflect(Default, PartialEq, Serialize, Deserialize)]
        pub struct $name(f32);

        impl $name {
            #[inline]
            pub const fn new(value: f32) -> Self {
                Self(value)
            }

            #[inline]
            pub const fn inner(self) -> f32 {
                self.0
            }
        }

        impl From<f32> for $name {
            #[inline]
            fn from(value: f32) -> Self {
                Self::new(value)
            }
        }
        
        impl From<$name> for f32 {
            fn from(value: $name) -> Self {
                value.inner()
            }
        }
        
        impl Add for $name {
            type Output = Self;
        
            #[inline]
            fn add(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl AddAssign for $name {
            #[inline]
            fn add_assign(&mut self, rhs: Self) {
                self.0 += rhs.0
            }
        }
        
        impl Sub for $name {
            type Output = Self;
        
            #[inline]
            fn sub(self, rhs: Self) -> Self::Output {
                Self(self.0 + rhs.0)
            }
        }

        impl SubAssign for $name {
            #[inline]
            fn sub_assign(&mut self, rhs: Self) {
                self.0 -= rhs.0
            }
        }
        
        impl Mul<f32> for $name {
            type Output = Self;
        
            #[inline]
            fn mul(self, rhs: f32) -> Self::Output {
                return Self(self.0 * rhs);
            }
        }

        impl MulAssign<f32> for $name {
            fn mul_assign(&mut self, rhs: f32) {
                *self = *self * rhs;
            }
        }
        
        impl Div<f32> for $name {
            type Output = Self;
        
            #[inline]
            fn div(self, rhs: f32) -> Self::Output {
                return Self(self.0 * rhs);
            }
        }

        impl DivAssign<f32> for $name {
            fn div_assign(&mut self, rhs: f32) {
                *self = *self / rhs;
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)?;
                f.write_str($unit)?;
                return Ok(());
            }
        }
    };
}

unit! {
    name: Energy,
    doc: "A unit of energy, corresponding to 1 joule.",
    unit: "J",
    aliases: [ "Joule" ],
}

unit! {
    name: Current,
    doc: "A unit of current, corresponding to 1 ampere.",
    unit: "A",
    aliases: [ "Ampere" ],
}

unit! {
    name: Force,
    doc: "A unit of force, corresponding to 1 newton.",
    unit: "N",
    aliases: [ "Newton" ],
}

unit! {
    name: Torque,
    doc: "A unit of torque, corresponding to 1 newton meter.",
    unit: "Nm",
    aliases: [ "Newton-meter" ],
}

unit! {
    name: Length,
    doc: "A unit of length, corresponding to 1 meter.",
    unit: "M",
    aliases: [ "Meter" ],
}

unit! {
    name: Area,
    doc: "A unit of area, corresponding to 1 square meter.",
    unit: "M²",
    aliases: [ "Square meter" ],
}

unit! {
    name: Volume,
    doc: "A unit of volume, corresponding to 1 liter.",
    unit: "L",
    aliases: [ "Liter" ],
}

unit! {
    name: Weight,
    doc: "A unit of weight, corresponding to 1 gram.",
    unit: "g",
    aliases: [ "Gram" ],
}

unit! {
    name: Density,
    doc: "A unit of density, derived from mass and volume.",
    unit: "g/mL",
    aliases: [],
}

unit! {
    name: Pressure,
    doc: "A measurement of pressure, derived from force and area.",
    unit: "Pa",
    aliases: [ "Pascal" ],
}

impl Mul<Volume> for Density {
    type Output = Weight;

    #[inline]
    fn mul(self, rhs: Volume) -> Self::Output {
        return Weight::new(self.0 * rhs.0);
    }
}

impl Mul<Density> for Volume {
    type Output = Weight;

    #[inline]
    fn mul(self, rhs: Density) -> Self::Output {
        return Weight::new(self.0 * rhs.0);
    }
}

impl Div<Area> for Force {
    type Output = Pressure;

    #[inline]
    fn div(self, rhs: Area) -> Self::Output {
        return Pressure::new(self.0 / rhs.0);
    }
}

impl Div<Volume> for Weight {
    type Output = Density;

    #[inline]
    fn div(self, rhs: Volume) -> Self::Output {
        return Density::new(self.0 / rhs.0);
    }
}