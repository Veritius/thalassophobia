use bevy::{app::MainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};

/// Configures whether or not simulation ticking is enabled.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Resource, Reflect)]
#[reflect(Resource)]
pub enum Simulating {
    #[default]
    Enabled,
    Disabled,
}

pub(crate) struct SimulationSchedulesPlugin;

impl Plugin for SimulationSchedulesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Simulating>();

        app.init_schedule(TryUpdateSimulation);
        app.init_schedule(SimulationUpdate);
        app.init_schedule(SimulationCleanup);

        let mut schedules = app.world_mut().resource_mut::<MainScheduleOrder>();
        schedules.insert_after(Update, TryUpdateSimulation);

        app.add_systems(TryUpdateSimulation, |world: &mut World| {
            let sim = world.resource::<Simulating>();
            if *sim == Simulating::Enabled {
                world.run_schedule(SimulationUpdate);
            }
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, ScheduleLabel)]
struct TryUpdateSimulation;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct SimulationUpdate;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct SimulationCleanup;