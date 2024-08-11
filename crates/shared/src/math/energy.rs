use std::ops::{Add, Div, Mul, Sub};
use crate::prelude::*;

/// A measure of force.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct Newton(u32);

impl From<u32> for Newton {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Newton> for u32 {
    fn from(value: Newton) -> Self {
        value.0
    }
}

impl Add for Newton {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_add(rhs.0))
    }
}

impl Sub for Newton {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_sub(rhs.0))
    }
}

impl Mul<f32> for Newton {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        let v = self.0 as f32 * rhs;
        return Self(v as u32);
    }
}

impl Div<f32> for Newton {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        let v = self.0 as f32 / rhs;
        return Self(v as u32);
    }
}

/// A measure of energy.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct Joule(u32);

impl From<u32> for Joule {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<Joule> for u32 {
    fn from(value: Joule) -> Self {
        value.0
    }
}

impl Add for Joule {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_add(rhs.0))
    }
}

impl Sub for Joule {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.saturating_sub(rhs.0))
    }
}

impl Mul<f32> for Joule {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        let v = self.0 as f32 * rhs;
        return Self(v as u32);
    }
}

impl Div<f32> for Joule {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        let v = self.0 as f32 / rhs;
        return Self(v as u32);
    }
}