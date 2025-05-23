use spacetimedb::{reducer, ReducerContext};

use crate::tables::players::{players, Player};

#[reducer]
fn move_player(ctx: &ReducerContext, x: f32, y: f32, z: f32) {
    ctx.db.players().id().update(Player {
        id: ctx.sender,
        x,
        y,
        z,
    });
}
