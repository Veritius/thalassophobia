mod progress;
mod ui;

use shared::bevy::prelude::*;
use shared::progress::*;
use crate::state::ClientState;
use progress::dummy_progress_tracker;

pub(crate) struct InitialLoadingPlugin;

impl Plugin for InitialLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ProgressTrackerPlugin::<InitialLoading>::default());

        app.add_systems(Update, (
            dummy_progress_tracker.track_progress::<InitialLoading>(),
        ).run_if(in_state(ClientState::Initial)));

        app.add_systems(OnEnter(ClientState::Initial), ui::spawn_loading_screen);
        app.add_systems(Done::<InitialLoading>::new(), (
            |mut next: ResMut<NextState<ClientState>>| { next.set(ClientState::MainMenu); },
            ui::despawn_loading_screen,
        ));
    }
}

struct InitialLoading;