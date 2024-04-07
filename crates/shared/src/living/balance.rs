use std::{fmt::Debug, ops::{AddAssign, SubAssign}};
use bevy::prelude::*;

#[derive(Component)]
pub struct Balance {
    val: u32,
    max: u32,
}

impl Balance {
    pub fn value_fract(&self) -> f32 {
        self.val as f32 / self.max as f32
    }

    pub fn get_val_pts(&self) -> u32 {
        self.val
    }

    pub fn get_max_pts(&self) -> u32 {
        self.max
    }

    pub fn set_val(&mut self, val: u32) {
        self.val = val.min(self.max);
    }

    pub fn set_max(&mut self, val: u32) {
        self.max = val;
        self.set_val(self.val);
    }
}

impl AddAssign<u32> for Balance {
    fn add_assign(&mut self, rhs: u32) {
        self.val = self.val.saturating_add(rhs).min(self.max);
    }
}

impl SubAssign<u32> for Balance {
    fn sub_assign(&mut self, rhs: u32) {
        self.val = self.val.saturating_sub(rhs);
    }
}

impl Debug for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Balance")
        .field("val", &self.val)
        .field("max", &self.max)
        .field("frac", &self.value_fract())
        .finish()
    }
}