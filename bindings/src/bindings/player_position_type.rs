// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#![allow(unused, clippy::all)]
use spacetimedb_sdk::__codegen::{self as __sdk, __lib, __sats, __ws};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct PlayerPosition {
    pub id: __sdk::Identity,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub update_count: u8,
}

impl __sdk::InModule for PlayerPosition {
    type Module = super::RemoteModule;
}
