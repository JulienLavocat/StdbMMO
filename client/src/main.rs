use animation_link::AnimationEntityLinkPlugin;
use avian3d::PhysicsPlugins;
use avian3d::prelude::PhysicsDebugPlugin;
use bevy::log::{DEFAULT_FILTER, LogPlugin};
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use bevy_tnua::prelude::TnuaControllerPlugin;
use bevy_tnua_avian3d::TnuaAvian3dPlugin;
use input::Actions;
use iyes_perf_ui::PerfUiPlugin;
use iyes_perf_ui::prelude::PerfUiDefaultEntries;
use leafwing_input_manager::plugin::InputManagerPlugin;
use local_player::LocalPlayerPlugin;
use remote_players::RemotePlayersPlugin;
use server::ServerPlugin;
use world::WorldPlugin;

mod animation_link;
mod constants;
mod input;
mod local_player;
mod remote_players;
mod server;
mod world;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Aria Online".to_string(),
                        present_mode: bevy::window::PresentMode::Immediate,
                        mode: WindowMode::Windowed,
                        // mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
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
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
        .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins((
            PerfUiPlugin,
            WorldInspectorPlugin::new(),
            InputManagerPlugin::<Actions>::default(),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
            TnuaControllerPlugin::new(FixedUpdate),
            TnuaAvian3dPlugin::new(FixedUpdate),
            ThirdPersonCameraPlugin,
            AnimationEntityLinkPlugin,
        ))
        .add_plugins(ServerPlugin)
        .add_plugins((WorldPlugin, LocalPlayerPlugin, RemotePlayersPlugin))
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(PerfUiDefaultEntries::default());
}
