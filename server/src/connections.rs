use spacetimedb::{reducer, ReducerContext, Table};

use crate::tables::players::{players, Player};

#[reducer(client_connected)]
fn on_connected(ctx: &ReducerContext) {
    ctx.db.players().insert(Player {
        id: ctx.sender,
        x: 0.0,
        y: 0.0,
        z: 0.0,
        max_health: 100.0,
        health: 75.0,
        max_mana: 100.0,
        mana: 50.0,
        online: true,
    });
}

#[reducer(client_disconnected)]
fn on_disconnected(ctx: &ReducerContext) {
    let mut player = ctx.db.players().id().find(ctx.sender).unwrap();
    player.online = false;
    ctx.db.players().id().update(player);
}
