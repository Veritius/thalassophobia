use bevy::prelude::*;
use thalassophobia::plugins::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_plugins((
        PhysicsPlugin,
        EssentialPlugins,
        InitialLoadingPlugin,
        DevtoolsPlugin,
        MechanicPlugins,
        ControllerPlugins,
    ));

    app.run();
}