mod mainmenu;
mod settings;
mod state;

use shared::bevy::prelude::*;

fn main() {
    let mut app = App::new();
    shared::setup(&mut app);

    app.add_plugins(mainmenu::MainMenuPlugin);
    app.add_plugins(state::GameStatePlugin);

    app.run();
}