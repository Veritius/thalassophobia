use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::math::curve::FloatCurve;

/// A wound entity, applied to a creature with vitality.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Wound {

}

/// Decreases vitality by a flat amount.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct WoundFlatDamage(pub f32);

/// The severity of a [`Wound`]. Useless on its own.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct WoundSeverity(pub f32);

/// Passive change for the severity of the wound.
/// Multiplied by delta time before application.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct WoundSeverityPassiveChange(pub FloatCurve);

/// Decreases vitality based on the [`Severity`] of the wound.
#[derive(Debug, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct WoundSeverityAffectsVitality(pub FloatCurve);