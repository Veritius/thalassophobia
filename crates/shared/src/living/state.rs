use crate::prelude::*;

/// The state of a living creature.
#[derive(Debug, Clone, Copy, Component, PartialEq, Eq, Reflect, Serialize, Deserialize)]
#[reflect(PartialEq, Component, Serialize, Deserialize)]
pub enum LifeState {
    Alive,
    Critical,
    Dead,
}

impl LifeState {
    /// Returns true if the creature is alive or critical.
    pub fn is_alive(&self) -> bool {
        match self {
            LifeState::Alive => true,
            LifeState::Critical => true,
            LifeState::Dead => false,
        }
    }

    /// Returns true if the creature is dead.
    pub fn is_dead(&self) -> bool {
        *self == LifeState::Dead
    }
}