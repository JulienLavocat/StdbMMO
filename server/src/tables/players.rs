use spacetimedb::{table, Identity};

#[table(name = players, public)]
pub struct Player {
    #[primary_key]
    pub id: Identity,
    pub health: f32,
    pub max_health: f32,
    pub mana: f32,
    pub max_mana: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    #[index(btree)]
    pub online: bool,
}

impl Player {
    pub fn new(id: Identity) -> Self {
        Self {
            id,
            health: 100.0,
            max_health: 100.0,
            mana: 100.0,
            max_mana: 100.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            online: true,
        }
    }
}
