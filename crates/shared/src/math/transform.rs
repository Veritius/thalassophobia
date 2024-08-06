use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A set of values corresponding to each direction in all three dimensions.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(where T: Reflect)]
pub struct TranslateSet<T> {
    pub up: T,
    pub down: T,
    pub left: T,
    pub right: T,
    pub fwd: T,
    pub back: T,
}

impl<T> TranslateSet<T> {
    pub const fn splat(value: T) -> Self
    where
        T: Copy,
    {
        Self {
            up: value,
            down: value,
            left: value,
            right: value,
            fwd: value,
            back: value,
        }
    }

    pub fn merge<F>(self, other: Self, mut func: F) -> Self
    where
        F: FnMut(T, T) -> T,
    {
        Self {
            up: func(self.up, other.up),
            down: func(self.down, other.down),
            left: func(self.left, other.left),
            right: func(self.right, other.right),
            fwd: func(self.fwd, other.fwd),
            back: func(self.back, other.back),
        }
    }

    pub fn merge_checked<F, E>(self, other: Self, mut func: F) -> Result<Self, E>
    where
        F: FnMut(T, T) -> Result<T, E>,
    {
        Ok(Self {
            up: func(self.up, other.up)?,
            down: func(self.down, other.down)?,
            left: func(self.left, other.left)?,
            right: func(self.right, other.right)?,
            fwd: func(self.fwd, other.fwd)?,
            back: func(self.back, other.back)?,
        })
    }

    pub fn merge_in_place<F>(&mut self, other: Self, mut func: F)
    where
        F: FnMut(&mut T, T),
    {
        func(&mut self.up, other.up);
        func(&mut self.down, other.down);
        func(&mut self.left, other.left);
        func(&mut self.right, other.right);
        func(&mut self.fwd, other.fwd);
        func(&mut self.back, other.back);
    }
}

impl<T: Add<Output = T>> Add for TranslateSet<T> {
    type Output = TranslateSet<T>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        TranslateSet::merge(self, rhs, T::add)
    }
}

impl<T: AddAssign> AddAssign for TranslateSet<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.merge_in_place(rhs, T::add_assign)
    }
}

impl<T: Sub<Output = T>> Sub for TranslateSet<T> {
    type Output = TranslateSet<T>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        TranslateSet::merge(self, rhs, T::sub)
    }
}

impl<T: SubAssign> SubAssign for TranslateSet<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.merge_in_place(rhs, T::sub_assign)
    }
}

impl<T: Mul<Output = T>> Mul for TranslateSet<T> {
    type Output = TranslateSet<T>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        TranslateSet::merge(self, rhs, T::mul)
    }
}

impl<T: MulAssign> MulAssign for TranslateSet<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.merge_in_place(rhs, T::mul_assign)
    }
}

impl<T: Div<Output = T>> Div for TranslateSet<T> {
    type Output = TranslateSet<T>;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        TranslateSet::merge(self, rhs, T::div)
    }
}

impl<T: DivAssign> DivAssign for TranslateSet<T> {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        self.merge_in_place(rhs, T::div_assign)
    }
}

impl Mul<Vec3> for TranslateSet<f32> {
    type Output = TranslateSet<f32>;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            up: self.up * rhs.y,
            down: self.down * rhs.y,
            left: self.left * rhs.z,
            right: self.left * rhs.z,
            fwd: self.fwd * rhs.x,
            back: self.fwd * rhs.x,
        }
    }
}

impl Div<Vec3> for TranslateSet<f32> {
    type Output = TranslateSet<f32>;

    fn div(self, rhs: Vec3) -> Self::Output {
        Self {
            up: self.up / rhs.y,
            down: self.down / rhs.y,
            left: self.left / rhs.z,
            right: self.left / rhs.z,
            fwd: self.fwd / rhs.x,
            back: self.fwd / rhs.x,
        }
    }
}

impl From<Vec3> for TranslateSet<f32> {
    fn from(value: Vec3) -> Self {
        Self {
            up: value.y,
            down: value.y,
            left: value.z,
            right: value.z,
            fwd: value.x,
            back: value.x,
        }
    }
}