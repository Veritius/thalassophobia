mod ui;

use shared::bevy::prelude::*;
use shared::progress::*;
use crate::state::{ClientState, InitialPassed};

pub(crate) struct InitialLoadingPlugin;

impl Plugin for InitialLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ScheduleProgressTrackerPlugin::<InitialLoading>::default());

        // Run at app start
        app.add_systems(OnEnter(ClientState::Initial), (
            start_tracking::<InitialLoading>,
            ui::spawn_loading_screen,
        ));

        // Tracking update
        app.add_systems(Update, ((
            ui::update_loading_screen.after(ProgressTrackingSet::<InitialLoading>::new()),
        )).run_if(in_state(ClientState::Initial)));

        // End of tracking
        app.add_systems(Done::<InitialLoading>::new(), (
            |mut commands: Commands| { commands.insert_resource(InitialPassed); },
            |mut next: ResMut<NextState<ClientState>>| { next.set(ClientState::MainMenu); },
            ui::despawn_loading_screen,
        ));
    }
}

pub struct InitialLoading;