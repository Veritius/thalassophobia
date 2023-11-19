use bevy::prelude::*;

/// Information for how an object wants to move.
/// This data is independent of the object's current transform data, but are still to be applied relative to it.
/// For example, pressing W as a player character should walk forward in the direction they are looking.
/// 
/// This component is generic for anything that moves around consciously, including player characters and NPCs.
/// How the component affects objects' transformations is up to other game systems.
#[derive(Debug, Default, Clone, Component, Reflect)]
#[reflect(Default, Component)]
pub struct MovementIntent {
    /// Intent to move up and down, on the Y axis.
    pub vertical: f32,
    /// Intent to move left and right, on the X axis.
    pub horizontal: f32,
    /// Intent to move forward and back, on the Z axis.
    pub forward: f32,

    /// Intent to rotate around the Y axis.
    pub yaw: f32,
    /// Intent to rotate around the X axis.
    pub pitch: f32,
    /// Intent to rotate around the Z axis.
    pub roll: f32,
}

impl MovementIntent {
    /// Returns the translation intent (vertical, horizontal, forward) as a 3 dimensional vector.
    pub const fn translate_intent(&self) -> Vec3 {
        Vec3 {
            x: self.horizontal,
            y: self.vertical,
            z: self.forward,
        }
    }

    /// Returns the rotation intent (yaw, pitch, roll) as a 3 dimensional vector.
    pub const fn rotate_intent_vec(&self) -> Vec3 {
        Vec3 {
            x: self.yaw,
            y: self.pitch,
            z: self.roll,
        }
    }

    /// Returns the rotation intent (yaw, pitch, roll) as a quaternion.
    pub fn rotate_intent_quat(&self) -> Quat {
        Quat::from_euler(
            EulerRot::YXZ,
            self.yaw,
            self.pitch,
            self.roll
        )
    }
}