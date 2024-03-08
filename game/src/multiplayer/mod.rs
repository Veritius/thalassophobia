use bevy::prelude::*;
use bevy_stardust::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect, States)]
pub enum MultiplayerState {
    #[default]
    Disabled,
    Standby,
    Running,
}

pub(super) fn setup_multiplayer(app: &mut App) {
    app.add_plugins((
        StardustPlugin,
    ));

    app.register_type::<MultiplayerState>();
    app.init_state::<MultiplayerState>();
}