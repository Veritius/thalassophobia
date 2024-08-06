use std::ops::{Add, Mul, Sub};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A set of values corresponding to each direction in all three translational axes.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(where T: Reflect)]
pub struct TranslateSet<T> {
    /// Up (Positive Y)
    pub up: T,

    /// Down (Negative Y)
    pub down: T,

    /// Left (Negative X)
    pub left: T,

    /// Right (Positive X)
    pub right: T,

    /// Forward (Positive Z)
    pub fwd: T,

    /// Backward (Negative Z)
    pub back: T,
}

impl<T: Add> Add for TranslateSet<T> {
    type Output = TranslateSet<<T as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        TranslateSet {
            up: self.up + rhs.up,
            down: self.down + rhs.down,
            left: self.left + rhs.left,
            right: self.right + rhs.right,
            fwd: self.fwd + rhs.fwd,
            back: self.back + rhs.back,
        }
    }
}

impl<T: Sub> Sub for TranslateSet<T> {
    type Output = TranslateSet<<T as Sub>::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        TranslateSet {
            up: self.up - rhs.up,
            down: self.down - rhs.down,
            left: self.left - rhs.left,
            right: self.right - rhs.right,
            fwd: self.fwd - rhs.fwd,
            back: self.back - rhs.back,
        }
    }
}

impl<T: Mul> Mul for TranslateSet<T> {
    type Output = TranslateSet<<T as Mul>::Output>;

    fn mul(self, rhs: Self) -> Self::Output {
        TranslateSet {
            up: self.up * rhs.up,
            down: self.down * rhs.down,
            left: self.left * rhs.left,
            right: self.right * rhs.right,
            fwd: self.fwd * rhs.fwd,
            back: self.back * rhs.back,
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
    pub pitch: T,
    pub yaw: T,
    pub roll: T,
}

impl<T: Add> Add for RotationSet<T> {
    type Output = RotationSet<<T as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        RotationSet {
            pitch: self.pitch + rhs.pitch,
            yaw: self.yaw + rhs.yaw,
            roll: self.roll + rhs.roll,
        }
    }
}

impl<T: Sub> Sub for RotationSet<T> {
    type Output = RotationSet<<T as Sub>::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        RotationSet {
            pitch: self.pitch - rhs.pitch,
            yaw: self.yaw - rhs.yaw,
            roll: self.roll - rhs.roll,
        }
    }
}

impl<T: Mul> Mul for RotationSet<T> {
    type Output = RotationSet<<T as Mul>::Output>;

    fn mul(self, rhs: Self) -> Self::Output {
        RotationSet {
            pitch: self.pitch * rhs.pitch,
            yaw: self.yaw * rhs.yaw,
            roll: self.roll * rhs.roll,
        }
    }
}

impl From<RotationSet<f32>> for Quat {
    fn from(value: RotationSet<f32>) -> Self {
        Quat::from_euler(
            EulerRot::YXZ,
            value.roll, 
            value.yaw,
            value.pitch,
        )
    }
}