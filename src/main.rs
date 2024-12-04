use bevy::prelude::*;

fn main() {
    //Declaring the app var.
    let mut app = App::new();

    //Plugins
    app.add_plugins(DefaultPlugins);

    //Run the game
    app.run();
}
