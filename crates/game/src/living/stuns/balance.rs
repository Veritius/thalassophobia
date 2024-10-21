use std::ops::{Add, Sub, AddAssign, SubAssign};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Balance {
    #[reflect(@0.0 ..= 1.0)]
    current: f32,

    pub resilience: f32,
    pub regeneration: f32,
}

impl Default for Balance {
    fn default() -> Self {
        Self {
            current: 1.0,

            resilience: 1.0,
            regeneration: 0.2,
        }
    }
}

impl Add<f32> for Balance {
    type Output = Self;

    fn add(mut self, rhs: f32) -> Self::Output {
        self.current += rhs * self.resilience;
        return self;
    }
}

impl AddAssign<f32> for Balance {
    fn add_assign(&mut self, rhs: f32) {
        self.current = self.current + rhs;
    }
}

impl Sub<f32> for Balance {
    type Output = Self;

    fn sub(mut self, rhs: f32) -> Self::Output {
        self.current -= rhs * self.resilience;
        return self;
    }
}

impl SubAssign<f32> for Balance {
    fn sub_assign(&mut self, rhs: f32) {
        self.current = self.current - rhs;
    }
}