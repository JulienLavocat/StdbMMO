use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_spacetimedb::{ReadDeleteEvent, ReadInsertEvent, StdbConnection};
use bevy_third_person_camera::{CameraSyncSet, ThirdPersonCamera, ThirdPersonCameraTarget, Zoom};
use bevy_tnua::{
    TnuaUserControlsSystemSet,
    prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController},
};
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    bindings::{DbConnection, Player as PlayerTable},
    input::{Actions, create_input_map},
};

const WALK_SPEED: f32 = 10.0;

#[derive(Component)]
pub struct LocalPlayer;

#[derive(Component)]
pub struct LocalPlayerCamera;

pub struct LocalPlayerPlugin;

impl Plugin for LocalPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(PostUpdate, CameraSyncSet.after(PhysicsSet::Sync))
            .add_systems(PreUpdate, (on_player_inserted, on_player_deleted).chain())
            .add_systems(
                FixedUpdate,
                apply_controls.in_set(TnuaUserControlsSystemSet),
            )
            .add_systems(PostUpdate, rotate_character);
    }
}

fn on_player_inserted(
    mut commands: Commands,
    mut events: ReadInsertEvent<PlayerTable>,
    asset_server: Res<AssetServer>,
    conn: Res<StdbConnection<DbConnection>>,
) {
    for event in events.read() {
        if event.row.id != conn.identity() {
            return;
        }

        info!("Local player inserted: {:?}", event.row);
        let model = asset_server.load("character.glb#Scene0");

        commands.spawn((
            LocalPlayer,
            Name::new(format!("Player#{}", event.row.id.to_abbreviated_hex())),
            create_input_map(),
            Transform::from_xyz(event.row.x, event.row.y, event.row.z),
            RigidBody::Dynamic,
            Collider::capsule_endpoints(0.3, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
            TnuaController::default(),
            TnuaAvian3dSensorShape(Collider::cylinder(0.29, 0.0)),
            LockedAxes::ROTATION_LOCKED,
            Visibility::Visible,
            ThirdPersonCameraTarget,
            children![(SceneRoot(model), Transform::from_xyz(0.0, -0.5, 0.0))],
        ));

        commands.spawn((
            LocalPlayerCamera,
            Camera3d::default(),
            ThirdPersonCamera {
                cursor_lock_key: KeyCode::Escape,
                sensitivity: Vec2::new(2.0, 2.0),
                zoom: Zoom::new(2.0, 10.0),
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
            return;
        }

        info!(
            "Local player deleted: {:?}, removing player {} and camera {}",
            event.row, player_entity, camera_entity
        );
        commands.entity(player_entity).despawn();
    }
}

fn apply_controls(
    mut controller: Single<&mut TnuaController>,
    actions: Single<&ActionState<Actions>, With<LocalPlayer>>,
    camera_transform: Single<&Transform, With<LocalPlayerCamera>>,
) {
    let direction = actions.clamped_axis_pair(&Actions::Move);

    let mut forward: Vec3 = camera_transform.forward().into();
    forward.y = 0.0;
    forward = forward.normalize_or_zero();

    let mut right: Vec3 = camera_transform.right().into();
    right.y = 0.0;
    right = right.normalize();

    let direction = (forward * direction.y + right * direction.x).normalize_or_zero();

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: direction * WALK_SPEED,
        float_height: 0.51,
        ..Default::default()
    });

    if actions.pressed(&Actions::Jump) {
        controller.action(TnuaBuiltinJump {
            height: 4.0,
            ..Default::default()
        });
    }
}

fn rotate_character(
    mut player_transform: Single<&mut Transform, (With<LocalPlayer>, Without<LocalPlayerCamera>)>,
    camera_transform: Single<&Transform, With<LocalPlayerCamera>>,
) {
    let forward = camera_transform.forward();
    let flat_forward = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();

    if flat_forward.length_squared() > 0.0 {
        let target_rotation = Quat::from_rotation_arc(Vec3::Z, -flat_forward);
        player_transform.rotation = target_rotation;
    }
}
