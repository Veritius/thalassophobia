mod escape;
mod initial;
mod player;
mod settings;

#[cfg(feature="devstuff")]
mod devstuff;
mod debugsys;

use shared::{bevy::prelude::*, SetupMode};

fn main() {
    // Create app and add shared setup
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    shared::setup(&mut app, SetupMode::Full);

    // Server setup, if hosting is enabled
    #[cfg(feature="hosting")]
    app.add_plugins(server::LobbyHostingPlugin);

    // Client subsystems
    app.add_plugins(escape::EscapeMenuPlugin);
    app.add_plugins(initial::InitialisationUiPlugin);
    app.add_plugins(settings::UserSettingsPlugin);

    #[cfg(feature="devstuff")]
    app.add_plugins(devstuff::DevStuffPlugin);

    app.add_plugins(debugsys::DebugSystemsPlugin);

    // Run the client
    app.run();
}