use bevy::prelude::*;
use bevy_spacetimedb::StdbConnection;
use bindings::{DbConnection, move_player};

#[derive(Component)]
pub struct MovementReplication {
    pub timer: Timer,
    pub last_position: Vec3,
    pub position_threshold_squarred: f32,
}

pub fn sync_movement_with_server(
    time: Res<Time>,
    player: Single<(&GlobalTransform, &mut MovementReplication)>,
    conn: Res<StdbConnection<DbConnection>>,
) {
    let (player_transform, mut replication) = player.into_inner();

    replication.timer.tick(time.delta());
    if replication.timer.just_finished() {
        let current_position = player_transform.translation();
        let delta = current_position - replication.last_position;
        if delta.length_squared() >= replication.position_threshold_squarred {
            let pos = player_transform.translation();
            conn.reducers().move_player(pos.x, pos.y, pos.z).unwrap();
            replication.last_position = current_position;
        }
    }
}
