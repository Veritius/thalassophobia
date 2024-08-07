use bevy::ecs::query::{QueryData as BevyQueryData, QueryFilter as BevyQueryFilter};
use bevy_rapier3d::{plugin::RapierContext, prelude::*};
use leafwing_input_manager::prelude::ActionState;
use crate::{bevy::prelude::*, disabling::Disabled, math::transform::TranslateSet, physics::*};

use super::{GroundedMovements, RotationMovements, SwimmingMovements};

/// A player character controller.
#[derive(Debug, Component, Reflect)]
pub struct PlayerController {
    /// The head entity.
    /// 
    /// This may be the same entity that the `PlayerController` component is attached to.
    pub head: Entity,

    /// Current rotation (yaw).
    pub rotation_yaw: f32,

    /// Base ground movement speed.
    pub base_walk_speed: TranslateSet<f32>,
    /// Coefficient applied to walk speed while sprinting.
    pub walk_sprint_coefficient: TranslateSet<f32>,

    /// Base swim speed.
    pub base_swim_speed: TranslateSet<f32>,
    /// Coefficient applied to swim speed while sprinting.
    pub swim_sprint_coefficient: TranslateSet<f32>,

    /// The length of the raycast used to check if the controller is touching the ground.
    pub ground_raycast_len: f32,
    /// Set to `true` when touching an object that it can collide with.
    pub is_touching_ground: bool,
}

/// The head entity of a [`PlayerController`].
#[derive(Debug, Component, Reflect)]
pub struct PlayerControllerHead {
    /// Current rotation (pitch).
    pub rotation_pitch: f32,
}

pub(super) fn controller_grounding_system(
    rapier_context: Res<RapierContext>,
    // We can't filter out entities that haven't changed positions, since we can't account for other entities changing position.
    mut controllers: Query<(Entity, &mut PlayerController, &GlobalTransform, Option<&CollisionGroups>), Without<Disabled>>,
) {
    for (entity, mut controller, transform, groups) in controllers.iter_mut() {
        // Get membership data from a component if present, otherwise use a default
        let group = if let Some(groups) = groups {
            groups.clone()
        } else {
            // Default collision group
            CollisionGroups {
                memberships: PHYS_GROUP_CHARACTER,
                filters: PHYS_GROUP_TERRAIN | PHYS_GROUP_STRUCTURE
            }
        };

        // Cast a ray to see if there's anything below us to jump off of
        let raycast = rapier_context.cast_ray(
            transform.translation(),
            Vec3::NEG_Y,
            controller.ground_raycast_len,
            false,
            QueryFilter::default().exclude_collider(entity).groups(group),
        );

        // Apply the raycast result to the controller
        match raycast {
            Some(_) => { controller.is_touching_ground = true; },
            None => { controller.is_touching_ground = false; },
        }
    }
}

#[derive(BevyQueryData)]
#[query_data(mutable)]
struct MovementSystemQueryData<'w> {
    body_controller: Option<&'w mut PlayerController>,
    head_controller: Option<&'w mut PlayerControllerHead>,

    transform: &'w mut Transform,
    impulse: Option<&'w mut ExternalImpulse>,

    rotation_action_state: Option<&'w ActionState<RotationMovements>>,
    grounded_action_state: Option<&'w ActionState<GroundedMovements>>,
    swimming_action_state: Option<&'w ActionState<SwimmingMovements>>,
}

#[derive(BevyQueryFilter)]
struct MovementSystemQueryFilter(Or<(
    With<PlayerController>,
    With<PlayerControllerHead>,
)>);

pub(super) fn controller_movement_system(

) {

}