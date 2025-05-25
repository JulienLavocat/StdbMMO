use spacetimedb::{reducer, ReducerContext};

use crate::tables::players::players;

#[reducer]
fn move_player(ctx: &ReducerContext, x: f32, y: f32, z: f32) {
    let mut player = ctx.db.players().id().find(ctx.sender).unwrap();
    player.x = x;
    player.y = y;
    player.z = z;
    ctx.db.players().id().update(player);
}
