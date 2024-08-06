use std::ops::{Add, Mul, Sub};
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
    pub pitch_up: T,
    pub pitch_down: T,
    pub yaw_left: T,
    pub yaw_right: T,
    pub roll_left: T,
    pub roll_right: T,
}

impl<T: Add> Add for RotationSet<T> {
    type Output = RotationSet<<T as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        RotationSet {
            pitch_up: self.pitch_up + rhs.pitch_up,
            pitch_down: self.pitch_down + rhs.pitch_down,
            yaw_left: self.yaw_left + rhs.yaw_left,
            yaw_right: self.yaw_right + rhs.yaw_right,
            roll_left: self.roll_left + rhs.roll_left,
            roll_right: self.roll_right + rhs.roll_right,
        }
    }
}

impl<T: Sub> Sub for RotationSet<T> {
    type Output = RotationSet<<T as Sub>::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        RotationSet {
            pitch_up: self.pitch_up - rhs.pitch_up,
            pitch_down: self.pitch_down - rhs.pitch_down,
            yaw_left: self.yaw_left - rhs.yaw_left,
            yaw_right: self.yaw_right - rhs.yaw_right,
            roll_left: self.roll_left - rhs.roll_left,
            roll_right: self.roll_right - rhs.roll_right,
        }
    }
}

impl<T: Mul> Mul for RotationSet<T> {
    type Output = RotationSet<<T as Mul>::Output>;

    fn mul(self, rhs: Self) -> Self::Output {
        RotationSet {
            pitch_up: self.pitch_up * rhs.pitch_up,
            pitch_down: self.pitch_down * rhs.pitch_down,
            yaw_left: self.yaw_left * rhs.yaw_left,
            yaw_right: self.yaw_right * rhs.yaw_right,
            roll_left: self.roll_left * rhs.roll_left,
            roll_right: self.roll_right * rhs.roll_right,
        }
    }
}