use bevy::{ecs::{query::QueryEntityError, system::SystemParam}, prelude::*};
use super::{PlayerController, PlayerControllerHead};

#[derive(SystemParam)]
pub struct PlayerControllers<'w, 's> {
    pub bodies: Query<'w, 's, (&'static PlayerController, &'static GlobalTransform)>,
    pub heads: Query<'w, 's, (&'static PlayerControllerHead, &'static GlobalTransform)>,
}

impl<'w, 's> PlayerControllers<'w, 's> {
    /// Returns a ray showing the direction of where they are looking.
    pub fn look_ray(&self, entity: Entity) -> Result<Ray3d, QueryEntityError> {
        let (body_data, body_transform) = self.bodies.get(entity)?;

        match body_data.head_entity {
            Some(head_id) => todo!(),
            None => todo!(),
        }

        // let (head_data, head_transform) = self.heads.get()?;

        todo!()
    }
}