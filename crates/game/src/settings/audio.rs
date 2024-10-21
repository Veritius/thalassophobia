use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub struct MasterVolume(pub f32);

#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub struct AmbientVolume(pub f32);

/// Visually displays sounds for the hearing impaired.
/// 
/// https://gameaccessibilityguidelines.com/provide-a-pingable-sonar-style-audio-map
#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub enum AudioSonar {
    #[default]
    Disabled,
    Enabled,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Reflect)]
pub enum Subtitles {
    #[default]
    Disabled,
    Minimal,
    Verbose,
}