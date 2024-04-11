use shared::bevy::prelude::*;
use super::*;

pub(super) fn show_hide_toggles_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut inspector: ResMut<WorldInspectorVisibility>,
    mut infodump: ResMut<InfodumpWindowVisibility>,
) {
    if keyboard.just_pressed(KeyCode::F1) {
        *inspector = match *inspector {
            WorldInspectorVisibility::Show => WorldInspectorVisibility::Hide,
            WorldInspectorVisibility::Hide => WorldInspectorVisibility::Show,
        }
    }

    if keyboard.just_pressed(KeyCode::F2) {
        *infodump = match *infodump {
            InfodumpWindowVisibility::Show => InfodumpWindowVisibility::Hide,
            InfodumpWindowVisibility::Hide => InfodumpWindowVisibility::Show,
        }
    }
}