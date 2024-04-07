mod initial;
mod settings;
mod state;

mod debugsys;

use shared::bevy::prelude::*;

fn main() {
    let mut app = App::new();
    shared::setup(&mut app);

    app.add_plugins(state::GameStatePlugin);
    app.add_plugins(initial::InitialLoadingPlugin);

    app.add_plugins(debugsys::DebugSystemsPlugin);

    app.run();
}