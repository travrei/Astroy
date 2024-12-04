use bevy::{prelude::*, window::WindowResolution};
/*
    Change the Resolution and Title of the Window!
*/
pub fn change_window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            position: WindowPosition::Centered(MonitorSelection::Primary),
            resolution: WindowResolution::new(1280., 720.),
            title: "Astroy: Clean the Space!".to_string(),
            resizable: false,
            ..default()
        }),
        ..default()
    }
}
