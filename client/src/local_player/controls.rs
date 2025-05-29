use bevy::prelude::*;
use bevy_tnua::prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController};
use leafwing_input_manager::prelude::ActionState;

use crate::input::Actions;

use super::{
    LocalPlayer, LocalPlayerCamera, PLAYER_JUMP_HEIGHT, PLAYER_RUN_SPEED, PLAYER_WALK_SPEED,
};

pub fn apply_controls(
    mut controller: Single<&mut TnuaController>,
    actions: Single<&ActionState<Actions>, With<LocalPlayer>>,
    camera_transform: Single<&Transform, With<LocalPlayerCamera>>,
) {
    let direction = actions.clamped_axis_pair(&Actions::Move);

    let mut forward: Vec3 = camera_transform.forward().into();
    forward.y = 0.0;
    forward = forward.normalize_or_zero();

    let mut right: Vec3 = camera_transform.right().into();
    right.y = 0.0;
    right = right.normalize();

    let direction = (forward * direction.y + right * direction.x).normalize_or_zero();

    let speed = if actions.pressed(&Actions::Run) {
        PLAYER_RUN_SPEED
    } else {
        PLAYER_WALK_SPEED
    };

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: direction * speed,
        float_height: 0.51,
        ..Default::default()
    });

    if actions.pressed(&Actions::Jump) {
        controller.action(TnuaBuiltinJump {
            height: PLAYER_JUMP_HEIGHT,
            ..Default::default()
        });
    }
}

pub fn rotate_character(
    mut player_transform: Single<&mut Transform, (With<LocalPlayer>, Without<LocalPlayerCamera>)>,
    camera_transform: Single<&Transform, With<LocalPlayerCamera>>,
) {
    let forward = camera_transform.forward();
    let flat_forward = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();

    if flat_forward.length_squared() > 0.0 {
        let target_rotation = Quat::from_rotation_arc(Vec3::Z, -flat_forward);
        player_transform.rotation = target_rotation;
    }
}
