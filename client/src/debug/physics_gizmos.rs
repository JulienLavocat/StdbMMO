use avian3d::prelude::PhysicsGizmos;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::input::Actions;

pub struct PhysicsGizmosPlugin;

impl Plugin for PhysicsGizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, toggle_physics_gizmos);
    }
}

fn toggle_physics_gizmos(
    mut physics_gizmos: ResMut<GizmoConfigStore>,
    actions: Single<&ActionState<Actions>>,
) {
    if !actions.just_pressed(&Actions::DebugTogglePhysicsGizmos) {
        return;
    }

    let enabled = !physics_gizmos.config_mut::<PhysicsGizmos>().0.enabled;
    physics_gizmos.config_mut::<PhysicsGizmos>().0.enabled = enabled;
}
