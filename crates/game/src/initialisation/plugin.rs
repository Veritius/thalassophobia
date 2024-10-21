use bevy::prelude::*;
use bevy_mod_progress::*;
use super::Initialisation;

type ObserverType = Observer<Done<Initialisation>, ()>;

pub(crate) struct InitialLoadingPlugin;

impl Plugin for InitialLoadingPlugin {
    fn build(&self, app: &mut App) {
        // Initialisation state
        app.init_state::<Initialisation>();

        // Add the progress tracking stuff
        app.add_plugins(ResourceProgressTrackingPlugin::<Initialisation>::default());
        app.init_resource::<Progress<Initialisation>>();

        // type hack to ensure that we can remove this observer later
        let observer: ObserverType = Observer::new(load_finished_observer);
        app.world_mut().spawn(observer);
    }
}

fn load_finished_observer(
    trigger: Trigger<Done<Initialisation>>,
    query: Query<Entity, With<ObserverType>>,
    mut state: ResMut<NextState<Initialisation>>,
    mut commands: Commands,
) {
    // Log the completion of initialisation
    let work = trigger.event().work();
    info!(work, "Initialisation complete");

    // Set the next state
    state.set(Initialisation::Finished);

    // Remove all observers watching the Done<Initial> event
    // since it'll never be triggered again
    for observer in &query {
        commands.entity(observer).despawn();
    }
}