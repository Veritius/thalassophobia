use std::fmt::Display;

use shared::bevy::prelude::*;
use shared::{bevy_ecs, bevy_state};

pub(crate) struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ClientState>();

        app.add_systems(OnEnter(ClientState::Initial), initial_transition_check);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, States)]
pub enum ClientState {
    /// Loading state before the main menu.
    #[default]
    Initial,

    /// In the main menu.
    MainMenu,

    /// In a singleplayer campaign.
    Singleplayer,

    /// In a multiplayer campaign.
    #[cfg(feature="multiplayer")]
    Multiplayer,
}

/// Added only when the [`Initial`](ClientState::Initial) state is passed.
#[derive(Resource)]
pub(crate) struct InitialPassed;

// Sanity check that makes the game explode if an invalid state transition occurs.
fn initial_transition_check(
    passed: Option<Res<InitialPassed>>,
) {
    if passed.is_none() { return }
    panic!("Transitioned to the Initial state from another state, which isn't allowed.");
}