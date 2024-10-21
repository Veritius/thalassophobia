use bevy::prelude::*;
use thalassophobia::plugins::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.add_plugins((
        MechanicPlugins,
    ));

    app.run();
}