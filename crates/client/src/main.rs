mod escape;
mod initial;
mod settings;
mod state;

#[cfg(feature="devstuff")]
mod devstuff;
mod debugsys;

use shared::{bevy::prelude::*, SetupMode};

fn main() {
    // Create app and add shared setup
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.insert_resource(SetupMode::Full);
    shared::pre_setup(&mut app);

    // Client subsystems
    app.add_plugins(escape::EscapeMenuPlugin);
    app.add_plugins(initial::InitialLoadingPlugin);
    app.add_plugins(settings::UserSettingsPlugin);
    app.add_plugins(state::GameStatePlugin);

    #[cfg(feature="devstuff")]
    app.add_plugins(devstuff::DevStuffPlugin);

    app.add_plugins(debugsys::DebugSystemsPlugin);

    // Final shared setup and run app
    shared::post_setup(&mut app);
    app.run();
}