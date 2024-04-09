use shared::bevy::prelude::*;
use shared::bevy_ecs;
use crate::initial::InitialPassed;

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

impl ClientState {
    /// Returns `true` if a campaign is loaded.
    pub fn in_campaign(&self) -> bool {
        match self {
            Self::Singleplayer => true,
            #[cfg(feature="multiplayer")]
            Self::Multiplayer => true,

            _ => false,
        }
    }
}

// Sanity check that makes the game explode if an invalid state transition occurs.
fn initial_transition_check(
    passed: Option<Res<InitialPassed>>,
) {
    if passed.is_none() { return }
    panic!("Transitioned to the Initial state from another state, which isn't allowed.");
}