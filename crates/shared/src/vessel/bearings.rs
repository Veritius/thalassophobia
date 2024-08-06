use std::ops::{Add, Mul, Sub};
use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Reflect)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
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