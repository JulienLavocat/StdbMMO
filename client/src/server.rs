use bevy::prelude::*;
use bevy_spacetimedb::{
    ReadStdbConnectedEvent, ReadStdbDisconnectedEvent, StdbConnectedEvent, StdbConnection,
    StdbConnectionErrorEvent, StdbDisconnectedEvent, StdbPlugin, tables,
};

use bindings::{DbConnection, PlayersTableAccess};

const MODULE_NAME: &str = "ariaonline";
const STDB_URI: &str = "https://stdb.jlavocat.eu";

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        let plugin = StdbPlugin::default()
            .with_connection(|send_connected, send_disconnected, send_connect_error, _| {
                let conn = DbConnection::builder()
                    .with_module_name(MODULE_NAME)
                    .with_uri(STDB_URI)
                    .on_connect_error(move |_ctx, err| {
                        send_connect_error
                            .send(StdbConnectionErrorEvent { err })
                            .unwrap();
                    })
                    .on_disconnect(move |_ctx, err| {
                        send_disconnected
                            .send(StdbDisconnectedEvent { err })
                            .unwrap();
                    })
                    .on_connect(move |_ctx, _id, _c| {
                        send_connected.send(StdbConnectedEvent {}).unwrap();
                    })
                    .build()
                    .expect("SpacetimeDB connection failed");

                conn.run_threaded();
                conn
            })
            .with_events(|plugin, app, db, _| {
                tables!(players);
            });
        app.add_plugins(plugin);

        app.add_systems(First, on_connected)
            .add_systems(Last, on_disconnected);
    }
}

fn on_connected(mut events: ReadStdbConnectedEvent, conn: Res<StdbConnection<DbConnection>>) {
    for _ in events.read() {
        info!("Connected to SpacetimeDB");

        conn.subscribe()
            .on_applied(|_| info!("Subscribed to players"))
            .on_error(|_, err| {
                error!("Error subscribing to players: {}", err);
            })
            .subscribe("SELECT * FROM players WHERE online = true");
    }
}

fn on_disconnected(mut events: ReadStdbDisconnectedEvent) {
    for _ in events.read() {
        info!("Disconnected from SpacetimeDB");
    }
}
