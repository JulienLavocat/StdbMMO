use animations::{PlayerAnimationState, PlayerAnimationsPlugin};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_spacetimedb::{ReadDeleteEvent, ReadInsertEvent, StdbConnection};
use bevy_third_person_camera::{
    CameraSyncSet, Offset, ThirdPersonCamera, ThirdPersonCameraTarget, Zoom,
};
use bevy_tnua::{TnuaAnimatingState, TnuaUserControlsSystemSet, prelude::TnuaController};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use bindings::{DbConnection, Player as PlayerTable};
use controls::{apply_controls, rotate_character};
use movement_replication::{MovementReplication, sync_movement_with_server};

mod animations;
mod controls;
mod movement_replication;

use crate::{input::create_input_map, load_world::CharacterAssets, state::InGameSet};

pub const PLAYER_WALK_SPEED: f32 = 4.0;
pub const PLAYER_RUN_SPEED: f32 = 10.0;
pub const PLAYER_JUMP_HEIGHT: f32 = 2.0;

#[derive(Component)]
pub struct LocalPlayer;

#[derive(Component)]
pub struct LocalPlayerCamera;

pub struct LocalPlayerPlugin;

impl Plugin for LocalPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(PostUpdate, CameraSyncSet.after(PhysicsSet::Sync))
            .add_plugins(PlayerAnimationsPlugin)
            .add_systems(
                PreUpdate,
                (on_player_inserted, on_player_deleted)
                    .in_set(InGameSet)
                    .chain(),
            )
            .add_systems(
                FixedUpdate,
                apply_controls
                    .in_set(TnuaUserControlsSystemSet)
                    .in_set(InGameSet),
            )
            .add_systems(
                PostUpdate,
                (rotate_character, sync_movement_with_server).in_set(InGameSet),
            );
    }
}

fn on_player_inserted(
    mut commands: Commands,
    mut events: ReadInsertEvent<PlayerTable>,
    character_assets: Res<CharacterAssets>,
    conn: Res<StdbConnection<DbConnection>>,
) {
    for event in events.read() {
        if event.row.id != conn.identity() {
            continue;
        }

        info!("Local player inserted: {:?}", event.row);

        commands.spawn((
            LocalPlayer,
            Visibility::Visible,
            Name::new(format!("Player#{}", event.row.id.to_abbreviated_hex())),
            create_input_map(),
            Transform::from_xyz(event.row.x, event.row.y, event.row.z),
            RigidBody::Dynamic,
            Collider::capsule_endpoints(0.3, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
            TnuaController::default(),
            TnuaAvian3dSensorShape(Collider::cylinder(0.29, 0.0)),
            TnuaAnimatingState::<PlayerAnimationState>::default(),
            LockedAxes::ROTATION_LOCKED,
            ThirdPersonCameraTarget,
            MovementReplication {
                last_position: Vec3::new(event.row.x, event.row.y, event.row.z),
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
                position_threshold_squarred: 0.01,
            },
            children![(
                SceneRoot(character_assets.character_scene.clone()),
                Transform::from_xyz(0.0, -0.5, 0.0)
            )],
        ));

        commands.spawn((
            LocalPlayerCamera,
            Camera3d::default(),
            ThirdPersonCamera {
                cursor_lock_key: KeyCode::Escape,
                sensitivity: Vec2::new(2.0, 2.0),
                zoom: Zoom::new(2.0, 20.0),
                offset: Offset::new(0.0, 2.0),
                ..Default::default()
            },
        ));
    }
}

fn on_player_deleted(
    mut commands: Commands,
    mut events: ReadDeleteEvent<PlayerTable>,
    player_entity: Single<Entity, With<LocalPlayer>>,
    camera_entity: Single<Entity, With<LocalPlayerCamera>>,
    conn: Res<StdbConnection<DbConnection>>,
) {
    let player_entity = player_entity.into_inner();
    let camera_entity = camera_entity.into_inner();

    for event in events.read() {
        if event.row.id != conn.identity() {
            continue;
        }

        info!(
            "Local player deleted: {:?}, removing player {} and camera {}",
            event.row, player_entity, camera_entity
        );
        commands.entity(player_entity).despawn();
    }
}
