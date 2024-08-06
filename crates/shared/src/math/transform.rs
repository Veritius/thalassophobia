use std::ops::{Add, Div, Mul, Sub};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A set of values corresponding to each direction in all three translational axes.
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
        TranslateSet::merge(self, rhs, |a,b| a+b)
    }
}

impl<T: Sub<Output = T>> Sub for TranslateSet<T> {
    type Output = TranslateSet<T>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        TranslateSet::merge(self, rhs, |a,b| a-b)
    }
}

impl<T: Mul<Output = T>> Mul for TranslateSet<T> {
    type Output = TranslateSet<T>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        TranslateSet::merge(self, rhs, |a,b| a*b)
    }
}

impl<T: Div<Output = T>> Div for TranslateSet<T> {
    type Output = TranslateSet<T>;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        TranslateSet::merge(self, rhs, |a,b| a/b)
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

/// A set of values corresponding to each direction in all three rotational axes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(where T: Reflect)]
pub struct RotationSet<T> {
    pub pitch_up: T,
    pub pitch_down: T,
    pub yaw_left: T,
    pub yaw_right: T,
    pub roll_left: T,
    pub roll_right: T,
}

impl<T> RotationSet<T> {
    pub fn merge<F>(self, other: Self, mut func: F) -> Self
    where
        F: FnMut(T, T) -> T,
    {
        Self {
            pitch_up: func(self.pitch_up, other.pitch_up),
            pitch_down: func(self.pitch_down, other.pitch_down),
            yaw_left: func(self.yaw_left, other.yaw_left),
            yaw_right: func(self.yaw_right, other.yaw_right),
            roll_left: func(self.roll_left, other.roll_left),
            roll_right: func(self.roll_right, other.roll_right),
        }
    }

    pub fn merge_checked<F, E>(self, other: Self, mut func: F) -> Result<Self, E>
    where
        F: FnMut(T, T) -> Result<T, E>,
    {
        Ok(Self {
            pitch_up: func(self.pitch_up, other.pitch_up)?,
            pitch_down: func(self.pitch_down, other.pitch_down)?,
            yaw_left: func(self.yaw_left, other.yaw_left)?,
            yaw_right: func(self.yaw_right, other.yaw_right)?,
            roll_left: func(self.roll_left, other.roll_left)?,
            roll_right: func(self.roll_right, other.roll_right)?,
        })
    }

    pub fn merge_in_place<F>(&mut self, other: Self, mut func: F)
    where
        F: FnMut(&mut T, T),
    {
        func(&mut self.pitch_up, other.pitch_up);
        func(&mut self.pitch_down, other.pitch_down);
        func(&mut self.yaw_left, other.yaw_left);
        func(&mut self.yaw_right, other.yaw_right);
        func(&mut self.roll_left, other.roll_left);
        func(&mut self.roll_right, other.roll_right);
    }
}

impl<T: Add<Output = T>> Add for RotationSet<T> {
    type Output = RotationSet<T>;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        RotationSet::merge(self, rhs, |a,b| a+b)
    }
}

impl<T: Sub<Output = T>> Sub for RotationSet<T> {
    type Output = RotationSet<T>;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        RotationSet::merge(self, rhs, |a,b| a-b)
    }
}

impl<T: Mul<Output = T>> Mul for RotationSet<T> {
    type Output = RotationSet<T>;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        RotationSet::merge(self, rhs, |a,b| a*b)
    }
}

impl<T: Div<Output = T>> Div for RotationSet<T> {
    type Output = RotationSet<T>;

    fn div(self, rhs: Self) -> Self::Output {
        RotationSet::merge(self, rhs, |a, b| a/b)
    }
}

impl Mul<Vec3> for RotationSet<f32> {
    type Output = RotationSet<f32>;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            pitch_up: self.pitch_up * rhs.y,
            pitch_down: self.pitch_down * rhs.y,
            yaw_left: self.yaw_left * rhs.z,
            yaw_right: self.yaw_right * rhs.z,
            roll_left: self.roll_left * rhs.x,
            roll_right: self.roll_right * rhs.x,
        }
    }
}

impl Div<Vec3> for RotationSet<f32> {
    type Output = RotationSet<f32>;

    fn div(self, rhs: Vec3) -> Self::Output {
        Self {
            pitch_up: self.pitch_up / rhs.y,
            pitch_down: self.pitch_down / rhs.y,
            yaw_left: self.yaw_left / rhs.z,
            yaw_right: self.yaw_right / rhs.z,
            roll_left: self.roll_left / rhs.x,
            roll_right: self.roll_right / rhs.x,
        }
    }
}

impl From<Vec3> for RotationSet<f32> {
    fn from(value: Vec3) -> Self {
        Self {
            pitch_up: value.y,
            pitch_down: value.y,
            yaw_left: value.z,
            yaw_right: value.z,
            roll_left: value.x,
            roll_right: value.x,
        }
    }
}