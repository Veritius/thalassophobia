mod initial;
mod settings;
mod state;

mod debugsys;

use shared::bevy::prelude::*;

fn main() {
    let mut app = App::new();
    shared::setup(&mut app);

    app.add_plugins(initial::InitialLoadingPlugin);
    app.add_plugins(settings::UserSettingsPlugin);
    app.add_plugins(state::GameStatePlugin);

    #[cfg(feature="editor")]
    app.add_plugins(bevy_editor_pls::EditorPlugin::new());

    app.add_plugins(debugsys::DebugSystemsPlugin);

    app.run();
}