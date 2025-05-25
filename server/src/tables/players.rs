use spacetimedb::{table, Identity};

#[table(name = players, public)]
pub struct Player {
    #[primary_key]
    pub id: Identity,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub health: f32,
    pub max_health: f32,
    pub mana: f32,
    pub max_mana: f32,
    pub online: bool,
}
