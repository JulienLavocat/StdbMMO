use bevy::log::{DEFAULT_FILTER, LogPlugin};
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use input::Actions;
use leafwing_input_manager::plugin::InputManagerPlugin;
use players::PlayersPlugin;
use server::ServerPlugin;

mod bindings;
mod input;
mod players;
mod server;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Aria Online".to_string(),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    filter: DEFAULT_FILTER.to_owned()
                        + ",client=debug,bevy_egui=error,bevy_render::view::window=error",
                    ..default()
                }),
        )
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins((
            WorldInspectorPlugin::new(),
            InputManagerPlugin::<Actions>::default(),
        ))
        .add_plugins(ServerPlugin)
        .add_plugins(PlayersPlugin)
        .run();
}
