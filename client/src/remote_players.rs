use avian3d::prelude::{Collider, RigidBody};
use bevy::{color::palettes::css::BLUE, platform::collections::HashMap, prelude::*};
use bevy_health_bar3d::{
    plugin::HealthBarPlugin,
    prelude::{BarHeight, BarSettings, ColorScheme, ForegroundColor, Percentage},
};
use bevy_spacetimedb::{ReadDeleteEvent, ReadInsertEvent, ReadUpdateEvent, StdbConnection};
use spacetimedb_sdk::Identity;

use crate::{
    bindings::{DbConnection, Player},
    constants::PLAYER_WALK_SPEED,
};

#[derive(Resource, Default)]
pub struct RemotePlayersRegistry {
    entities: HashMap<Identity, Entity>,
}

#[derive(Component)]
pub struct RemotePlayer {
    pub target_position: Vec3,
}

#[derive(Component, Reflect)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

impl Percentage for Health {
    fn value(&self) -> f32 {
        if self.max == 0.0 {
            0.0
        } else {
            self.current / self.max
        }
    }
}

#[derive(Component, Reflect)]
pub struct Mana {
    pub current: f32,
    pub max: f32,
}

impl Percentage for Mana {
    fn value(&self) -> f32 {
        if self.max == 0.0 {
            0.0
        } else {
            self.current / self.max
        }
    }
}

impl RemotePlayersRegistry {
    pub fn register(&mut self, id: Identity, entity: Entity) {
        self.entities.insert(id, entity);
    }

    pub fn get_entity(&self, id: &Identity) -> Option<Entity> {
        self.entities.get(id).copied()
    }

    pub fn remove(&mut self, id: &Identity) {
        self.entities.remove(id);
    }
}

pub struct RemotePlayersPlugin;

impl Plugin for RemotePlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            HealthBarPlugin::<Health>::default(),
            HealthBarPlugin::<Mana>::default(),
        ))
        .init_resource::<RemotePlayersRegistry>()
        .insert_resource(
            ColorScheme::<Mana>::new().foreground_color(ForegroundColor::Static(BLUE.into())),
        )
        .add_systems(
            PreUpdate,
            (on_remote_player_inserted, on_remote_player_deleted).chain(),
        )
        .add_systems(PostUpdate, lerp_remote_players)
        .add_systems(Update, on_remote_player_updated);
    }
}

fn on_remote_player_inserted(
    mut commands: Commands,
    mut registry: ResMut<RemotePlayersRegistry>,
    mut events: ReadInsertEvent<Player>,
    asset_server: Res<AssetServer>,
    conn: Res<StdbConnection<DbConnection>>,
) {
    for event in events.read() {
        if event.row.id == conn.identity() {
            continue;
        }

        info!("Remote player inserted: {:?}", event.row);
        let model = asset_server.load("character.glb#Scene0");

        let entity = commands
            .spawn((
                Name::new(format!(
                    "RemotePlayer#{}",
                    event.row.id.to_abbreviated_hex()
                )),
                Visibility::Visible,
                Transform::from_xyz(event.row.x, event.row.y, event.row.z),
                RigidBody::Kinematic,
                Collider::capsule_endpoints(
                    0.3,
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 1.0, 0.0),
                ),
                RemotePlayer {
                    target_position: Vec3::new(event.row.x, event.row.y, event.row.z),
                },
                (
                    Health {
                        current: event.row.health,
                        max: event.row.max_health,
                    },
                    BarSettings::<Health> {
                        offset: 1.6,
                        height: BarHeight::Static(0.05),
                        width: 1.0,
                        ..default()
                    },
                    Mana {
                        current: event.row.mana,
                        max: event.row.max_mana,
                    },
                    BarSettings::<Mana> {
                        offset: 1.5,
                        height: BarHeight::Static(0.05),
                        width: 1.0,
                        ..default()
                    },
                ),
                children![(SceneRoot(model), Transform::from_xyz(0.0, -0.5, 0.0))],
            ))
            .id();
        registry.register(event.row.id, entity);
    }
}

fn on_remote_player_updated(
    mut commands: Commands,
    registry: ResMut<RemotePlayersRegistry>,
    mut events: ReadUpdateEvent<Player>,
) {
    for event in events.read() {
        let row = &event.new;
        if let Some(entity) = registry.get_entity(&row.id) {
            commands.entity(entity).insert(RemotePlayer {
                target_position: Vec3::new(row.x, row.y, row.z),
            });
        }
    }
}

fn on_remote_player_deleted(
    mut commands: Commands,
    mut registry: ResMut<RemotePlayersRegistry>,
    mut events: ReadDeleteEvent<Player>,
    conn: Res<StdbConnection<DbConnection>>,
) {
    for event in events.read() {
        if event.row.id == conn.identity() {
            continue;
        }

        info!("Remote player deleted: {:?}", event.row);

        if let Some(remote_player_entity) = registry.get_entity(&event.row.id) {
            commands.entity(remote_player_entity).despawn();
            registry.remove(&event.row.id);
        } else {
            warn!("Remote player entity not found for ID: {}", event.row.id);
        }
    }
}

fn lerp_remote_players(time: Res<Time>, mut query: Query<(&mut Transform, &RemotePlayer)>) {
    for (mut transform, remote_player) in query.iter_mut() {
        let delta = remote_player.target_position - transform.translation;
        if delta.length() <= 0.01 {
            continue;
        }

        let current_translation = transform.translation;
        let target_translation = remote_player.target_position;
        let t = 1.0 - (-PLAYER_WALK_SPEED * time.delta_secs()).exp();

        transform.translation = current_translation.lerp(target_translation, t);
    }
}
