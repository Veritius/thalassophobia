pub mod controller;

pub(super) fn setup_movement(app: &mut bevy::prelude::App) {
    app.register_type::<controller::MovementMode>();
    app.register_type::<controller::MovementController>();
}