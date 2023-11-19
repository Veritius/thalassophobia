pub mod intent;
pub mod keyboard;

use bevy::prelude::*;
use self::intent::MovementIntent;

pub(super) fn setup_movement(app: &mut bevy::prelude::App) {
    app.register_type::<MovementIntent>();
    app.register_type::<keyboard::KeyboardMouseControl>();

    app.add_systems(Update, reset_intents
        .after(MovementSystem::Input)
        .after(MovementSystem::Apply)
        .in_set(MovementSystem::Reset));

    app.add_systems(Update, keyboard::keyboard_and_mouse_input_system
        .before(MovementSystem::Apply)
        .in_set(MovementSystem::Input));
}

fn reset_intents(
    mut query: Query<&mut MovementIntent>,
) {
    for mut item in query.iter_mut() {
        *item = MovementIntent::default();
    }
}

/// A system related to movement and handling [MovementController] components.
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub enum MovementSystem {
    /// Take movement inputs and applies them to [MovementController] components.
    Input,
    /// Applies movements based on the [MovementController] component data.
    Apply,
    /// Reset [MovementController] components to zero.
    Reset,
}