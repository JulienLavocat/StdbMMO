use bevy::prelude::*;
use physics_gizmos::PhysicsGizmosPlugin;
use player_window::PlayerWindowDebugPlugin;

mod physics_gizmos;
mod player_window;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerWindowDebugPlugin, PhysicsGizmosPlugin));
    }
}
