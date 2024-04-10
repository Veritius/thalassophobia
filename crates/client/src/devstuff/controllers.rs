use bevy_egui::*;
use shared::{bevy::prelude::*, bevy_ecs, bevy_reflect, controller::{PlayerController, PlayerControllers}};

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct PlayerControllerGizmos {
    pub config_window: bool,
    pub show_pos_ring: bool,
    pub show_look_ray: bool,
}

pub(super) fn controller_window(
    mut ctx: EguiContexts,
    mut config: ResMut<PlayerControllerGizmos>,
) {
    if !config.config_window { return; }
    egui::Window::new("Player Controllers").show(ctx.ctx_mut(), |ui| {
        ui.checkbox(&mut config.show_pos_ring, "Show position rings");
        ui.checkbox(&mut config.show_look_ray, "Show look ray");
    });
}

pub(super) fn draw_controllers(
    config: Res<PlayerControllerGizmos>,
    mut gizmos: Gizmos,
    controllers: Query<(Entity, &PlayerController, &GlobalTransform)>,
    extra: PlayerControllers,
) {
    const B_OFFSET: Vec3 = Vec3::new(0.0, -0.9, 0.0);

    for (entity, controller, transform) in controllers.iter() {
        if config.show_pos_ring {
            let color = match controller.is_touching_ground {
                true => Color::TURQUOISE,
                false => Color::ORANGE_RED,
            };

            gizmos.circle(transform.translation() + B_OFFSET, Direction3d::Y, 0.5, color);
        }

        if config.show_look_ray {
            if let Ok(ray) = extra.look_ray(entity) {
                gizmos.arrow(ray.get_point(1.0), ray.get_point(6.0), Color::TURQUOISE);
            }
        }
    }
}