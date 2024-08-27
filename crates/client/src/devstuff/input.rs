use shared::bevy::prelude::*;
use super::*;

pub(super) fn show_hide_toggles_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut infodump: ResMut<InfodumpWindowVisibility>,
    mut inspector: ResMut<WorldInspectorVisibility>,
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
}