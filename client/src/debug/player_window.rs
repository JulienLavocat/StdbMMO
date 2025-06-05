use bevy::{
    color::palettes::css::{RED, YELLOW},
    prelude::*,
};
use bevy_spacetimedb::StdbConnection;
use bindings::{DbConnection, PlayersWindowsTableAccess, SubscriptionHandle};
use leafwing_input_manager::prelude::ActionState;
use spacetimedb_sdk::{SubscriptionHandle as _, Table};

use crate::{input::Actions, local_player::LocalPlayer};

#[derive(Resource, Default)]
pub struct EnablePlayerWindowGizmos {
    pub enabled: bool,
    pub subscription: Option<SubscriptionHandle>,
}

pub struct PlayerWindowDebugPlugin;

impl Plugin for PlayerWindowDebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnablePlayerWindowGizmos>()
            .add_systems(Update, toggle_player_window_gizmos)
            .add_systems(Update, show_player_window_gizmos);
    }
}

fn toggle_player_window_gizmos(
    mut debug_gizmos: ResMut<EnablePlayerWindowGizmos>,
    conn: Res<StdbConnection<DbConnection>>,
    actions: Single<&ActionState<Actions>>,
) {
    if !actions.just_pressed(&Actions::DebugTogglePlayerWindowGizmos) {
        return;
    }
    debug_gizmos.enabled = !debug_gizmos.enabled;
    info!("Toggled player window gizmos: {}", debug_gizmos.enabled);

    if debug_gizmos.enabled {
        let subscription = format!(
            "SELECT * FROM players_windows WHERE id = 0x{}",
            conn.identity()
        );
        let err_subscription = subscription.clone();

        let subscription = conn
            .subscribe()
            .on_error(move |_, err| {
                error!(
                    "Error subscribing to players_windows: {} -> {}",
                    err_subscription, err
                );
            })
            .subscribe(subscription);
        debug_gizmos.subscription = Some(subscription);
    } else if let Some(subscription) = debug_gizmos.subscription.take() {
        if let Err(err) = subscription.unsubscribe() {
            error!("Error unsubscribing from player_window: {}", err);
        }
    }
}

fn show_player_window_gizmos(
    mut gizmos: Gizmos,
    debug_gizmos: ResMut<EnablePlayerWindowGizmos>,
    conn: Res<StdbConnection<DbConnection>>,
    player: Single<&Transform, With<LocalPlayer>>,
) {
    if !debug_gizmos.enabled {
        return;
    }

    for window in conn.db().players_windows().iter() {
        let center_x = window.hr_bl_x + (window.hr_tr_x - window.hr_bl_x) / 2.0;
        let center_z = window.hr_bl_z + (window.hr_tr_z - window.hr_bl_z) / 2.0;

        let isometry = Isometry3d::new(
            Vec3::new(center_x, player.translation.y, center_z),
            Quat::from_rotation_x(90.0f32.to_radians()),
        );

        // gizmos.sphere(isometry, 5.0, RED);
        gizmos.rect(isometry, Vec2::splat(window.hr_size), RED);
        gizmos.rect(isometry, Vec2::splat(window.lr_size), YELLOW);
    }
}
