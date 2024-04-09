mod detail;
mod initial;
mod settings;
mod state;

mod debugsys;

use shared::bevy::prelude::*;

fn main() {
    let mut app = App::new();
    shared::pre_setup(&mut app);

    app.add_plugins(detail::LevelOfDetailPlugin);
    app.add_plugins(initial::InitialLoadingPlugin);
    app.add_plugins(settings::UserSettingsPlugin);
    app.add_plugins(state::GameStatePlugin);

    app.add_plugins(debugsys::DebugSystemsPlugin);

    shared::post_setup(&mut app);
    app.run();
}