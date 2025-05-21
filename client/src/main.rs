use bevy::prelude::*;
use bevy::window::WindowMode;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Aria Online".to_string(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary), // FIXME: Use current monitor
                ..Default::default()
            }),
            ..default()
        }))
        .run();
}
