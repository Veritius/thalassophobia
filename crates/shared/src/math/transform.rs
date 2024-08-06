use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A set of values corresponding to each direction in all three dimensions.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(where T: Reflect)]
pub struct TranslateSet<T> {
    #[doc(alias("left", "yaw left"))]
    pub xn: T,

    #[doc(alias("right", "yaw right"))]
    pub xp: T,

    #[doc(alias("descent", "down", "pitch down"))]
    pub yn: T,

    #[doc(alias("ascent", "up", "pitch up"))]
    pub yp: T,

    #[doc(alias("far", "forward", "roll left"))]
    pub zn: T,

    #[doc(alias("near", "backward", "roll right"))]
    pub zp: T,
}

impl<T> TranslateSet<T> {
    pub const fn splat(value: T) -> Self
    where
        T: Copy,
    {
        Self {
            xn: value,
            xp: value,
            yn: value,
            yp: value,
            zn: value,
            zp: value,
        }
    }

    pub fn alter<F>(self, mut func: F) -> Self
    where
        F: FnMut(T) -> T,
    {
        Self {
            xn: func(self.xn),
            xp: func(self.xp),
            yn: func(self.yn),
            yp: func(self.yp),
            zn: func(self.zn),
            zp: func(self.zp),
        }
    }

    pub fn alter_in_place<F>(&mut self, mut func: F)
    where
        F: FnMut(&mut T),
    {
        func(&mut self.xn);
        func(&mut self.xp);
        func(&mut self.yn);
        func(&mut self.yp);
        func(&mut self.zn);
        func(&mut self.zp);
    }

    pub fn merge<F>(self, other: Self, mut func: F) -> Self
    where
        F: FnMut(T, T) -> T,
    {
        Self {
            xn: func(self.xn, other.xn),
            xp: func(self.xp, other.xp),
            yn: func(self.yn, other.yn),
            yp: func(self.yp, other.yp),
            zn: func(self.zn, other.zn),
            zp: func(self.zp, other.zp),
        }
    }

    pub fn merge_checked<F, E>(self, other: Self, mut func: F) -> Result<Self, E>
    where
        F: FnMut(T, T) -> Result<T, E>,
    {
        Ok(Self {
            yp: func(self.yp, other.yp)?,
            yn: func(self.yn, other.yn)?,
            xn: func(self.xn, other.xn)?,
            xp: func(self.xp, other.xp)?,
            zn: func(self.zn, other.zn)?,
            zp: func(self.zp, other.zp)?,
        })
    }

    pub fn merge_in_place<F>(&mut self, other: Self, mut func: F)
    where
        F: FnMut(&mut T, T),
    {
        func(&mut self.yp, other.yp);
        func(&mut self.yn, other.yn);
        func(&mut self.xn, other.xn);
        func(&mut self.xp, other.xp);
        func(&mut self.zn, other.zn);
        func(&mut self.zp, other.zp);
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

impl<T: Mul<Output = T> + Copy> Mul<T> for TranslateSet<T> {
    type Output = TranslateSet<T>;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        self.alter(|v| T::mul(v, rhs))
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for TranslateSet<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.alter_in_place(|v| *v *= rhs);
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for TranslateSet<T> {
    type Output = TranslateSet<T>;

    #[inline]
    fn div(self, rhs: T) -> Self::Output {
        self.alter(|v| T::div(v, rhs))
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for TranslateSet<T> {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.alter_in_place(|v| *v /= rhs);
    }
}

impl TranslateSet<f32> {
    pub fn squash(self) -> Vec3 {
        Vec3 {
            x: self.xp - self.xn,
            y: self.yp - self.yn,
            z: self.zp - self.zn,
        }
    }
}

impl Mul<Vec3> for TranslateSet<f32> {
    type Output = TranslateSet<f32>;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            xn: self.xn * rhs.z,
            xp: self.xp * rhs.z,
            yn: self.yn * rhs.y,
            yp: self.yp * rhs.y,
            zn: self.zn * rhs.x,
            zp: self.zp * rhs.x,
        }
    }
}

impl Div<Vec3> for TranslateSet<f32> {
    type Output = TranslateSet<f32>;

    fn div(self, rhs: Vec3) -> Self::Output {
        Self {
            xn: self.xn / rhs.x,
            xp: self.xp / rhs.x,
            yn: self.yn / rhs.y,
            yp: self.yp / rhs.y,
            zn: self.zn / rhs.z,
            zp: self.zp / rhs.z,
        }
    }
}

impl From<Vec3> for TranslateSet<f32> {
    fn from(value: Vec3) -> Self {
        Self {
            xn: value.x,
            xp: value.x,
            yn: value.y,
            yp: value.y,
            zn: value.z,
            zp: value.z,
        }
    }
}

impl Mul<TranslateSet<f32>> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: TranslateSet<f32>) -> Self::Output {
        Self {
            x: self.x * if self.x > 0.0 { rhs.xp } else { rhs.xn },
            y: self.y * if self.y > 0.0 { rhs.yp } else { rhs.yn },
            z: self.z * if self.z > 0.0 { rhs.zp } else { rhs.zn },
        }
    }
}