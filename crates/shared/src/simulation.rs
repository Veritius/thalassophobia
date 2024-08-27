use bevy::app::MainScheduleOrder;
use bevy_ecs::schedule::ScheduleLabel;
use crate::{initial::Initialisation, prelude::*};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, SubStates)]
#[source(Initialisation = Initialisation::Finished)]
pub enum SimulationLoaded {
    #[default]
    Unloaded,
    Loaded,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, SubStates)]
#[source(SimulationLoaded = SimulationLoaded::Loaded)]
pub enum SimulationRunning {
    #[default]
    Paused,
    Running,
}

pub(crate) struct SimulationStatePlugin;

impl Plugin for SimulationStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<SimulationLoaded>();
        app.add_sub_state::<SimulationRunning>();

        app.init_schedule(TryUpdateSimulation);
        app.init_schedule(SimulationUpdate);

        let mut schedules = app.world_mut().resource_mut::<MainScheduleOrder>();
        schedules.insert_after(Update, TryUpdateSimulation);

        app.add_systems(TryUpdateSimulation, |world: &mut World| {
            let run_state = match world.get_resource::<State<SimulationRunning>>() {
                Some(state) => state,
                None => { return },
            };

            if *run_state.get() == SimulationRunning::Running {
                world.run_schedule(SimulationUpdate);
            }
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, ScheduleLabel)]
struct TryUpdateSimulation;

/// A schedule run to tick the simulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct SimulationUpdate;

pub trait SimulationAppExt {
    /// Adds an event type that is only needed during a simulation.
    fn add_sim_event<E: Event>(&mut self);
}

impl SimulationAppExt for App {
    fn add_sim_event<E: Event>(&mut self) {
        self.add_systems(OnEnter(SimulationLoaded::Loaded), |mut commands: Commands| commands.init_resource::<Events<E>>());
        self.add_systems(OnExit(SimulationLoaded::Loaded), |mut commands: Commands| commands.remove_resource::<Events<E>>());
        self.add_systems(SimulationUpdate, |mut events: ResMut<Events<E>>| events.update());
    }
}