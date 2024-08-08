use shared::{bevy::prelude::*, SetupMode};

fn main() {
    // Create app and add shared setup
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    shared::setup(&mut app, SetupMode::Headless);

    // Run the server
    app.run();
}