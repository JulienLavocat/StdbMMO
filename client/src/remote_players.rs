use avian3d::prelude::{Collider, RigidBody};
use bevy::{color::palettes::css::BLUE, platform::collections::HashMap, prelude::*};
use bevy_health_bar3d::{
    plugin::HealthBarPlugin,
    prelude::{BarHeight, BarSettings, ColorScheme, ForegroundColor, Percentage},
};
use bevy_mod_billboard::prelude::*;
use bevy_spacetimedb::{ReadDeleteEvent, ReadInsertEvent, ReadUpdateEvent, StdbConnection};
use bindings::{DbConnection, PlayerPosition, PlayersTableAccess};
use spacetimedb_sdk::Identity;

use crate::{
    load_world::{CharacterAssets, NameplateAssets},
    local_player::PLAYER_WALK_SPEED,
    state::InGameSet,
};

#[derive(Resource, Default)]
pub struct RemotePlayersRegistry {
    entities: HashMap<Identity, Entity>,
}

#[derive(Component)]
pub struct RemotePlayerPosition {
    pub target_position: Vec3,
}

#[derive(Component)]
pub struct RemotePlayer;

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
            (
                on_remote_player_position_inserted,
                on_remote_player_position_deleted,
            )
                .in_set(InGameSet)
                .chain(),
        )
        .add_systems(PostUpdate, lerp_remote_players.in_set(InGameSet))
        .add_systems(Update, on_remote_player_position_updated.in_set(InGameSet));
    }
}

fn on_remote_player_position_inserted(
    mut commands: Commands,
    mut registry: ResMut<RemotePlayersRegistry>,
    mut events: ReadInsertEvent<PlayerPosition>,
    models: Res<CharacterAssets>,
    nameplates: Res<NameplateAssets>,
    conn: Res<StdbConnection<DbConnection>>,
) {
    for event in events.read() {
        if event.row.id == conn.identity() {
            continue;
        }

        info!("Remote player position inserted: {:?}", event.row.id);
        let player = conn.db().players().id().find(&conn.identity()).unwrap();

        let entity = commands
            .spawn((
                Name::new(format!("RemotePlayer#{}", player.id.to_abbreviated_hex())),
                Visibility::Visible,
                RigidBody::Kinematic,
                Collider::capsule_endpoints(
                    0.3,
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 1.0, 0.0),
                ),
                Transform::from_xyz(event.row.x, event.row.y, event.row.z),
                RemotePlayer,
                (
                    Health {
                        current: player.health,
                        max: player.max_health,
                    },
                    BarSettings::<Health> {
                        offset: 1.6,
                        height: BarHeight::Static(0.05),
                        width: 1.0,
                        ..default()
                    },
                    Mana {
                        current: player.mana,
                        max: player.max_mana,
                    },
                    BarSettings::<Mana> {
                        offset: 1.5,
                        height: BarHeight::Static(0.05),
                        width: 1.0,
                        ..default()
                    },
                ),
                children![(
                    SceneRoot(models.character_scene.clone()),
                    Transform::from_xyz(0.0, -0.5, 0.0)
                ),],
            ))
            .id();

        registry.register(event.row.id, entity);
    }
}

fn on_remote_player_position_deleted(
    mut commands: Commands,
    mut registry: ResMut<RemotePlayersRegistry>,
    mut events: ReadDeleteEvent<PlayerPosition>,
    conn: Res<StdbConnection<DbConnection>>,
) {
    for event in events.read() {
        if event.row.id == conn.identity() {
            continue;
        }

        info!("Remote player position deleted: {:?}", event.row.id);

        if let Some(remote_player_entity) = registry.get_entity(&event.row.id) {
            commands.entity(remote_player_entity).despawn();
            registry.remove(&event.row.id);
        } else {
            warn!(
                "Remote player position deleted for unknown entity: {}",
                event.row.id.to_abbreviated_hex()
            );
        }
    }
}

fn on_remote_player_position_updated(
    mut commands: Commands,
    registry: ResMut<RemotePlayersRegistry>,
    mut events: ReadUpdateEvent<PlayerPosition>,
) {
    for event in events.read() {
        let row = &event.new;
        if let Some(entity) = registry.get_entity(&row.id) {
            commands.entity(entity).insert(RemotePlayerPosition {
                target_position: Vec3::new(row.x, row.y, row.z),
            });
        }
    }
}

fn lerp_remote_players(time: Res<Time>, mut query: Query<(&mut Transform, &RemotePlayerPosition)>) {
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
