use shared::bevy::prelude::*;

fn main() {
    let mut app = App::new();
    shared::pre_setup(&mut app);

    app.run();
}