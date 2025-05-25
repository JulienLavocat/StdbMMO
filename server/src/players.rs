use spacetimedb::{reducer, ReducerContext};

use crate::tables::players::player_positions;

#[reducer]
fn move_player(ctx: &ReducerContext, x: f32, y: f32, z: f32) {
    let mut player = ctx.db.player_positions().id().find(ctx.sender).unwrap();
    player.x = x;
    player.y = y;
    player.z = z;
    ctx.db.player_positions().id().update(player);
}
