mod detail;
mod initial;
mod settings;
mod state;

mod debugsys;

use shared::{bevy::prelude::*, SetupMode};

fn main() {
    // Create app and add shared setup
    let mut app = App::new();
    app.insert_resource(SetupMode::Full);
    shared::pre_setup(&mut app);

    // Client subsystems
    app.add_plugins(detail::LevelOfDetailPlugin);
    app.add_plugins(initial::InitialLoadingPlugin);
    app.add_plugins(settings::UserSettingsPlugin);
    app.add_plugins(state::GameStatePlugin);

    app.add_plugins(debugsys::DebugSystemsPlugin);

    // Final shared setup and run app
    shared::post_setup(&mut app);
    app.run();
}