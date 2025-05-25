use spacetimedb::{reducer, ReducerContext, Table};

use crate::tables::players::{players, Player};

#[reducer(client_connected)]
fn on_connected(ctx: &ReducerContext) {
    ctx.db.players().insert(Player {
        id: ctx.sender,
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });
}

#[reducer(client_disconnected)]
fn on_disconnected(ctx: &ReducerContext) {
    // ctx.db.players().id().delete(ctx.sender);
}
