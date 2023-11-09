use std::fmt::Display;
use bevy::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Eq, Reflect)]
pub enum GraphicsLevel {
    Low,
    #[default]
    Medium,
    High,
}

impl Display for GraphicsLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use GraphicsLevel::*;
        f.write_str(match self {
            Low => "Low",
            Medium => "Medium",
            High => "High",
        })
    }
}