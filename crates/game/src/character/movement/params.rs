use bevy::{prelude::*, ecs::{system::SystemParam, query::QueryEntityError}};
use super::{PlayerController, PlayerControllerHead};

/// Helpful shorthand for various controller-related operations.
#[derive(SystemParam)]
pub struct PlayerControllers<'w, 's> {
    bodies: Query<'w, 's, (&'static PlayerController, &'static GlobalTransform)>,
    heads: Query<'w, 's, &'static PlayerControllerHead>,
}

impl<'w, 's> PlayerControllers<'w, 's> {
    /// Returns a ray showing the direction of where they are looking.
    pub fn look_ray(&self, entity: Entity) -> Result<Ray3d, QueryEntityError> {
        let (body_controller, body_transform) = self.bodies.get(entity)?;
        let head_controller = self.heads.get(body_controller.head)?;

        let origin = body_transform.translation();

        let yaw_quat = Quat::from_axis_angle(
            Vec3::Y,
            -body_controller.rotation_yaw,
        );

        let pitch_quat = Quat::from_axis_angle(
            Vec3::X,
            -head_controller.rotation_pitch,
        );

        let direction = Dir3::new_unchecked((yaw_quat + pitch_quat) * Vec3::NEG_Z);

        return Ok(Ray3d { origin, direction });
    }
}