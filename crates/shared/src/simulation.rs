use crate::prelude::*;

pub trait SimulationAppExt {
    /// Adds an event type that is only needed during a simulation.
    fn add_sim_event<E: Event>(&mut self);
}

impl SimulationAppExt for App {
    fn add_sim_event<E: Event>(&mut self) {
        self.init_resource::<Events<E>>();

        self.add_systems(SimulationInit, |mut commands: Commands| commands.init_resource::<Events<E>>());
        self.add_systems(SimulationUpdate, |mut events: ResMut<Events<E>>| events.update());
        self.add_systems(SimulationClear, |mut commands: Commands| commands.remove_resource::<Events<E>>());
    }
}