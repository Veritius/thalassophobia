use std::ops::{Add, Div, Mul, Sub};
use crate::prelude::*;

macro_rules! unit {
    ($name:ident, $doc:literal) => {
        #[doc=$doc]
        #[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Hash, Reflect, Serialize, Deserialize)]
        #[reflect(Serialize, Deserialize)]
        pub struct $name(u32);

        impl From<u32> for $name {
            fn from(value: u32) -> Self {
                Self(value)
            }
        }
        
        impl From<$name> for u32 {
            fn from(value: $name) -> Self {
                value.0
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

unit!(Gram, "A unit of mass.");
unit!(Joule, "A unit of energy.");
unit!(Litre, "A unit of volume.");
unit!(Newton, "A unit of force.");