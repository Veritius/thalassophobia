use shared::bevy::prelude::*;
use shared::bevy_ecs;
use shared::bevy_reflect;

#[derive(Resource, Reflect)]
pub struct AudioSettings {

}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            
        }
    }
}