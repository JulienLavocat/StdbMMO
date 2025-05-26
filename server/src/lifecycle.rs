use std::time::Duration;

use spacetimedb::{reducer, ReducerContext, ScheduleAt, Table, TimeDuration};

use crate::tables::{
    players::{players, players_positions, players_positions_lr, Player, PlayerPosition},
    players_windows::{players_window_updates, players_windows, PlayerWindow, PlayerWindowUpdate},
};

#[reducer(init)]
fn on_init(ctx: &ReducerContext) {
    ctx.db.players_window_updates().insert(PlayerWindowUpdate {
        id: 1,
        scheduled_at: ScheduleAt::Interval(TimeDuration::from_duration(Duration::from_millis(500))),
    });
}

#[reducer(client_connected)]
fn on_connected(ctx: &ReducerContext) {
    ctx.db.players().insert(Player::new(ctx.sender));

    let position = PlayerPosition::new(ctx.sender, 0.0, 0.0, 0.0);
    ctx.db.players_positions().insert(position);
    ctx.db.players_positions_lr().insert(position);

    ctx.db.players_windows().insert(PlayerWindow::new(
        ctx.sender, position.x, position.z, 256.0, 128.0,
    ));
}

#[reducer(client_disconnected)]
fn on_disconnected(ctx: &ReducerContext) {
    let mut player = ctx.db.players().id().find(ctx.sender).unwrap();
    let position = ctx.db.players_positions().id().find(ctx.sender).unwrap();

    player.online = false;
    // Save the player's data to cold storage
    player.x = position.x;
    player.y = position.y;
    player.z = position.z;

    ctx.db.players().id().update(player);

    // Clear the player's hot data
    ctx.db.players_windows().id().delete(ctx.sender);
    ctx.db.players_positions().id().delete(ctx.sender);
    ctx.db.players_positions_lr().id().delete(ctx.sender);
}
