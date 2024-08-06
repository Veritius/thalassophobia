use bevy::{ecs::{query::QueryEntityError, system::SystemParam}, prelude::*};
use super::{PlayerController, PlayerControllerHead};

/// Helpful shorthand for various controller-related operations.
#[derive(SystemParam)]
pub struct PlayerControllers<'w, 's> {
    pub bodies: Query<'w, 's, (&'static PlayerController, &'static GlobalTransform)>,
    pub heads: Query<'w, 's, (&'static PlayerControllerHead, &'static GlobalTransform)>,
}

impl<'w, 's> PlayerControllers<'w, 's> {
    /// Returns a ray showing the direction of where they are looking.
    pub fn look_ray(&self, entity: Entity) -> Result<Ray3d, QueryEntityError> {
        let (body_data, body_transform) = self.bodies.get(entity)?;
        let head_query = match body_data.head_entity {
            Some(head_id) => Some(self.heads.get(head_id)?),
            None => None,
        };

        let origin;
        let direction;

        match head_query {
            Some((_, head_transform)) => {
                origin = head_transform.translation();
                direction = Dir3::new_unchecked(body_data.look_quat() * -Vec3::Z);
            },
            None => {
                origin = body_transform.translation();
                direction = Dir3::new_unchecked(body_data.yaw_quat() * -Vec3::Z);
            },
        }

        return Ok(Ray3d { origin, direction })
    }
}