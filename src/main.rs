//Import the Crates
use bevy::prelude::*;
use plugins::window::change_window;

//Importing the Modules
mod plugins;

fn main() {
    //Declaring the app var.
    let mut app = App::new();

    //Plugins
    app.add_plugins(
        DefaultPlugins
            .set(change_window())
            .set(ImagePlugin::default_nearest() /*for pixelart*/),
    );

    //Run the game
    app.run();
}
