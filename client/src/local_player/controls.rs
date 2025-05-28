use bevy::prelude::*;
use bevy_tnua::prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController};
use leafwing_input_manager::prelude::ActionState;

use crate::{constants::PLAYER_WALK_SPEED, input::Actions};

use super::{LocalPlayer, LocalPlayerCamera};

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

    controller.basis(TnuaBuiltinWalk {
        desired_velocity: direction * PLAYER_WALK_SPEED,
        float_height: 0.51,
        ..Default::default()
    });

    if actions.just_pressed(&Actions::Jump) {
        println!("Jump action triggered");
    }

    if actions.pressed(&Actions::Jump) {
        controller.action(TnuaBuiltinJump {
            height: 4.0,
            fall_extra_gravity: 10.0,
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
