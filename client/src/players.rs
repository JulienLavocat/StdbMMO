use bevy::{color::palettes::css::SILVER, ecs::error::debug, prelude::*};
use bevy_spacetimedb::{ReadDeleteEvent, ReadInsertEvent, StdbConnection};
use leafwing_input_manager::prelude::ActionState;
use spacetimedb_sdk::Identity;

use crate::{
    bindings::{DbConnection, Player as PlayerTable},
    input::{Actions, create_input_map},
};

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
            .add_systems(PreUpdate, (on_player_inserted, on_player_deleted).chain())
            .add_systems(Update, player_movement);
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

        let row = event.row.clone();
        if row.id == conn.identity() {
            let model = asset_server.load("character.glb#Scene0");

            commands.spawn((
                Player { id: event.row.id },
                Name::new(format!("Player#{}", event.row.id.to_abbreviated_hex())),
                create_input_map(),
                SceneRoot(model),
                Transform::from_xyz(row.x, row.y, row.z),
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

fn player_movement(
    mut camera_transform: Single<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    q_player: Single<(&mut Transform, &ActionState<Actions>), With<Player>>,
    time: Res<Time>,
) {
    let (mut transform, action_state) = q_player.into_inner();

    let direction = action_state.clamped_axis_pair(&Actions::Move);
    let translation_y = transform.translation.y;
    let direction = Vec3::new(direction.x, translation_y, direction.y) * time.delta_secs() * 10.0;

    transform.translation += direction;
    camera_transform.translation += direction;
}
