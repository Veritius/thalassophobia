use bevy::prelude::*;

pub(super) fn setup_vitality(app: &mut App) {
    app.register_type::<Vitality>();
    app.register_type::<VitalityAdjustor>();
}

/// The cached vitality offset. "Healthy" is zero, "hurt" is negative, and positive is... well, more healthy than usual.
#[derive(Debug, Default, Component, Reflect)]
#[reflect(Debug, Component)]
pub struct Vitality(f32);

impl Vitality {
    pub fn get(&self) -> f32 {
        self.0
    }
}

impl std::ops::Deref for Vitality {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Adjusts a parent entity's [Vitality].
#[derive(Debug, Component, Reflect)]
#[reflect(Debug, Component)]
pub enum VitalityAdjustor {
    Add(f32),
    Sub(f32),
    Mul(f32),
}

impl Default for VitalityAdjustor {
    fn default() -> Self {
        Self::Add(0.0)
    }
}