use spacetimedb::{reducer, ReducerContext, Table};

use crate::tables::{
    players::{players_positions, players_positions_lr},
    players_windows::{players_windows, PlayerWindowUpdate},
};

const LR_UPDATE_THRESHOLD: u8 = 10; // Update LR positions every n updates

#[reducer]
fn move_player(ctx: &ReducerContext, x: f32, y: f32, z: f32) {
    let mut player = ctx.db.players_positions().id().find(ctx.sender).unwrap();
    player.x = x;
    player.y = y;
    player.z = z;

    if player.update_count >= LR_UPDATE_THRESHOLD {
        player.update_count = 0;
        ctx.db.players_positions_lr().id().update(player);
        ctx.db.players_positions().id().update(player);
    } else {
        player.update_count += 1;
        ctx.db.players_positions().id().update(player);
    }
}

#[reducer]
pub fn update_players_windows(ctx: &ReducerContext, _row: PlayerWindowUpdate) {
    for mut window in ctx.db.players_windows().iter() {
        let player = ctx.db.players_positions().id().find(window.id).unwrap();
        if window.needs_recompute(player.x, player.z) {
            window.recompute(player.x, player.z);
            ctx.db.players_windows().id().update(window);
        }
    }
}
