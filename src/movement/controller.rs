use bevy::prelude::*;

/// Controller for how an object wants to move.
/// This data is independent of the object's current transform data, but are still to be applied relative to it.
/// For example, pressing W as a player character should walk forward in the direction they are looking.
/// 
/// This component is generic for anything that moves around consciously, including player characters and NPCs.
/// How the component affects objects' transformations is up to other game systems.
#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component, Default)]
pub struct MovementController {
    /// Intent to move up and down, on the Y axis.
    pub intent_vertical: f32,
    /// Intent to move left and right, on the X axis.
    pub intent_horizontal: f32,
    /// Intent to move forward and back, on the Z axis.
    pub intent_forward: f32,

    /// Intent to rotate around the Y axis.
    pub intent_yaw: f32,
    /// Intent to rotate around the X axis.
    pub intent_pitch: f32,
    /// Intent to rotate around the Z axis.
    pub intent_roll: f32,

    /// Imposes special restrictions in cases like VR headsets.
    /// See the [MovementMode] documentation for more information.
    pub move_mode: MovementMode,
}

impl MovementController {
    /// Returns the translation intent (vertical, horizontal, forward) as a 3 dimensional vector.
    pub const fn translate_intent(&self) -> Vec3 {
        Vec3 {
            x: self.intent_horizontal,
            y: self.intent_vertical,
            z: self.intent_forward,
        }
    }

    /// Returns the rotation intent (yaw, pitch, roll) as a 3 dimensional vector.
    pub const fn rotate_intent_vec(&self) -> Vec3 {
        Vec3 {
            x: self.intent_yaw,
            y: self.intent_pitch,
            z: self.intent_roll,
        }
    }

    /// Returns the rotation intent (yaw, pitch, roll) as a quaternion.
    pub fn rotate_intent_quat(&self) -> Quat {
        Quat::from_euler(
            EulerRot::YXZ,
            self.intent_yaw,
            self.intent_pitch,
            self.intent_roll
        )
    }
}

/// Special restrictions for how an object can move.
#[derive(Debug, Default, Reflect)]
pub enum MovementMode {
    /// No restrictions - full freedom of movement!
    #[default]
    Desktop,
    /// No pitching or rolling allowed, to prevent incidents of extreme nausea and vertigo.
    Vr,
}