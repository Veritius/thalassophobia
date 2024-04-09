use shared::bevy::prelude::*;
use shared::bevy_ecs;

pub(crate) struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ClientState>();

        app.add_systems(OnTransition { from: ClientState::MainMenu, to: ClientState::Initial }, panic_bad_transition);
        app.add_systems(OnTransition { from: ClientState::Singleplayer, to: ClientState::Initial }, panic_bad_transition);

        #[cfg(feature="multiplayer")]
        app.add_systems(OnTransition { from: ClientState::Multiplayer, to: ClientState::Initial }, panic_bad_transition);
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

fn panic_bad_transition() {
    panic!("Somehow transitioned to the Initial state, which isn't allowed.");
}