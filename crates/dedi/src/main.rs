use shared::{bevy::prelude::*, SetupMode};

fn main() {
    // Create app and add shared setup
    let mut app = App::new();
    app.insert_resource(SetupMode::Headless);
    shared::pre_setup(&mut app);

    // Client subsystems
    /*
        Nothing yet.
    */

    // Final shared setup and run app
    shared::post_setup(&mut app);
    app.run();
}