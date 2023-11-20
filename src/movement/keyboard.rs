use bevy::{prelude::*, input::mouse::MouseMotion};
use crate::settings::ControlsSettings;
use super::intent::*;

/// Marks an entity to apply keyboard and mouse input to their [MovementIntent](super::intent::MovementIntent) component.
#[derive(Debug, Default, Component, Reflect)]
#[reflect(Debug, Component)]
pub struct KeyboardMouseControl;

pub(super) fn keyboard_and_mouse_input_system(
    keyboard: Res<Input<KeyCode>>,
    mut mouse: EventReader<MouseMotion>,
    mut query: Query<&mut MovementIntent, With<KeyboardMouseControl>>,
    controls: Res<ControlsSettings>,
) {
    fn buttons_intent(a: bool, b: bool) -> f32 {
        let mut intent = 0.0;
        if a { intent += 1.0 }
        if b { intent -= 1.0 }
        intent
    }

    let intent_vertical = buttons_intent(
        keyboard.pressed(controls.ascend),
        keyboard.pressed(controls.descend));
    let intent_horizontal = buttons_intent(
        keyboard.pressed(controls.walk_right),
        keyboard.pressed(controls.walk_left));
    let intent_forward = buttons_intent(
        keyboard.pressed(controls.walk_forward),
        keyboard.pressed(controls.walk_backward));
    let intent_roll = buttons_intent(
        keyboard.pressed(controls.roll_left),
        keyboard.pressed(controls.roll_right));
    let mouse_delta = mouse
        .read()
        .last() // We only care about the most recent event
        .map(|f| f.delta)
        .unwrap_or(Vec2::splat(0.0));

    fn add_clamped(val: &mut f32, pt: f32) {
        *val = (*val + pt).clamp(-1.0, 1.0);
    }

    for mut controller in query.iter_mut() {
        add_clamped(&mut controller.vertical, intent_vertical);
        add_clamped(&mut controller.horizontal, intent_horizontal);
        add_clamped(&mut controller.forward, intent_forward);
        add_clamped(&mut controller.roll, intent_roll);

        // We don't clamp these values for responsiveness
        controller.yaw += mouse_delta.x;
        controller.pitch += mouse_delta.y;
    }
}