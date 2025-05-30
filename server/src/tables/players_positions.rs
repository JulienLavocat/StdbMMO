use spacetimedb::{table, Identity};

#[table(name = players_positions, public)]
#[table(name = players_positions_lr, public)]
#[derive(Clone, Copy)]
pub struct PlayerPosition {
    #[primary_key]
    pub id: Identity,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub update_count: u8,
}

impl PlayerPosition {
    pub fn new(id: Identity, x: f32, y: f32, z: f32) -> Self {
        Self {
            id,
            x,
            y,
            z,
            update_count: 0,
        }
    }
}
