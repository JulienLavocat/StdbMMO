use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;
use bevy_spacetimedb::{ReadDeleteEvent, ReadInsertEvent, StdbConnection};
use spacetimedb_sdk::Identity;

use crate::bindings::{DbConnection, Player};

#[derive(Component)]
pub struct RemotePlayer {
    pub id: Identity,
}

pub struct RemotePlayersPlugin;

impl Plugin for RemotePlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (on_remote_player_inserted, on_remote_player_deleted).chain(),
        );
    }
}

fn on_remote_player_inserted(
    mut commands: Commands,
    mut events: ReadInsertEvent<Player>,
    asset_server: Res<AssetServer>,
    conn: Res<StdbConnection<DbConnection>>,
) {
    for event in events.read() {
        if event.row.id == conn.identity() {
            return;
        }

        info!("Remote player inserted: {:?}", event.row);
        let model = asset_server.load("character.glb#Scene0");

        commands.spawn((
            Name::new(format!(
                "RemotePlayer#{}",
                event.row.id.to_abbreviated_hex()
            )),
            Transform::from_xyz(event.row.x, event.row.y, event.row.z),
            RigidBody::Kinematic,
            Collider::capsule_endpoints(0.3, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)),
            Mesh3d(model),
        ));
    }
}

fn on_remote_player_deleted(
    mut commands: Commands,
    mut events: ReadDeleteEvent<Player>,
    conn: Res<StdbConnection<DbConnection>>,
    q_players: Query<(Entity, &RemotePlayer)>,
) {
    for event in events.read() {
        if event.row.id == conn.identity() {
            return;
        }

        info!("Remote player deleted: {:?}", event.row);
        if let Some(rmeote_player_entity) = q_players
            .iter()
            .find(|(_, player)| player.id == event.row.id)
        {
            commands.entity(rmeote_player_entity.0).despawn();
        } else {
            warn!("Remote player entity not found for ID: {}", event.row.id);
        }
    }
}
