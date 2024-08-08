use shared::{bevy::prelude::*, SetupMode};

fn main() {
    // Create app and do shared setup
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    shared::setup(&mut app, SetupMode::Headless);

    // Run the app
    app.run();
}