use shared::bevy::prelude::*;
use super::*;

pub(super) fn show_hide_toggles_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut infodump: ResMut<InfodumpWindowVisibility>,
    mut inspector: ResMut<WorldInspectorVisibility>,
    mut overlays: ResMut<OverlayWindowVisibility>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        *infodump = match *infodump {
            InfodumpWindowVisibility::Show => InfodumpWindowVisibility::Hide,
            InfodumpWindowVisibility::Hide => InfodumpWindowVisibility::Show,
        }
    }

    if keyboard.just_pressed(KeyCode::F2) {
        *inspector = match *inspector {
            WorldInspectorVisibility::Show => WorldInspectorVisibility::Hide,
            WorldInspectorVisibility::Hide => WorldInspectorVisibility::Show,
        }
    }

    if keyboard.just_pressed(KeyCode::F3) {
        *overlays = match *overlays {
            OverlayWindowVisibility::Show => OverlayWindowVisibility::Hide,
            OverlayWindowVisibility::Hide => OverlayWindowVisibility::Show,
        }
    }
}