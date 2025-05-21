use bevy::{color::palettes::css::SILVER, prelude::*};
use bevy_spacetimedb::{ReadDeleteEvent, ReadInsertEvent, StdbConnection};
use spacetimedb_sdk::Identity;

use crate::bindings::{DbConnection, Player as PlayerTable};

#[derive(Component)]
pub struct Player {
    pub id: Identity,
}

#[derive(Component)]
pub struct PlayerCamera;

pub struct PlayersPlugin;

impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(PreUpdate, (on_player_inserted, on_player_deleted).chain());
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0).subdivisions(10))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));
}

fn on_player_inserted(
    mut commands: Commands,
    mut events: ReadInsertEvent<PlayerTable>,
    asset_server: Res<AssetServer>,
    conn: Res<StdbConnection<DbConnection>>,
) {
    for event in events.read() {
        info!("Player inserted: {:?}", event.row);

        if event.row.id == conn.identity() {
            let model = asset_server.load("character.glb#Scene0");

            commands.spawn((
                Player { id: event.row.id },
                Name::new(format!("Player#{}", event.row.id.to_abbreviated_hex())),
                SceneRoot(model),
            ));

            commands.spawn((
                PlayerCamera,
                Camera3d::default(),
                Transform::from_xyz(0.0, 2.5, -5.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
            ));
        }
    }
}

fn on_player_deleted(
    mut commands: Commands,
    mut events: ReadDeleteEvent<PlayerTable>,
    q_players: Query<(Entity, &Player)>,
) {
    for event in events.read() {
        info!("Player deleted: {:?}", event.row);

        for (player_entity, player) in q_players.iter() {
            if player.id == event.row.id {
                info!("Removing player entity: {:?}", player_entity);
                commands.entity(player_entity).despawn();
            }
        }
    }
}
