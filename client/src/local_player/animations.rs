use bevy::prelude::*;
use bevy_tnua::{
    TnuaAction, TnuaAnimatingState, TnuaAnimatingStateDirective,
    builtins::TnuaBuiltinJumpState,
    prelude::{TnuaBuiltinJump, TnuaBuiltinWalk, TnuaController},
};

use super::LocalPlayer;
use crate::{
    animation_link::AnimationEntityLink,
    local_player::{PLAYER_WALK_SPEED, PlayerGltfHandle},
    state::InGameSet,
};

#[derive(Resource)]
struct PlayerAnimationNodes {
    standing: AnimationNodeIndex,
    walking: AnimationNodeIndex,
    running: AnimationNodeIndex,
}

pub enum PlayerAnimationState {
    Standing,
    Walking(f32),
    Running(f32),
    Jumping,
    Falling,
}

pub struct PlayerAnimationsPlugin;

impl Plugin for PlayerAnimationsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (prepare_animations, handle_animating).in_set(InGameSet),
        );
    }
}

fn prepare_animations(
    mut commands: Commands,
    mut animation_graph: ResMut<Assets<AnimationGraph>>,
    handle: Option<Res<PlayerGltfHandle>>,
    player_animation_entity: Single<&AnimationEntityLink, With<LocalPlayer>>,
    gtfs_assets: Res<Assets<Gltf>>,
) {
    let Some(handle) = handle else {
        return;
    };
    let Some(gltf_model) = gtfs_assets.get(&handle.0) else {
        return;
    };

    let mut graph = AnimationGraph::new();
    let root_node = graph.root;

    commands.insert_resource(PlayerAnimationNodes {
        standing: graph.add_clip(gltf_model.named_animations["Idle"].clone(), 1.0, root_node),
        walking: graph.add_clip(
            gltf_model.named_animations["Walking"].clone(),
            1.0,
            root_node,
        ),
        running: graph.add_clip(
            gltf_model.named_animations["Running"].clone(),
            1.0,
            root_node,
        ),
    });

    commands
        .entity(player_animation_entity.0)
        .insert(AnimationGraphHandle(animation_graph.add(graph)));

    commands.remove_resource::<PlayerGltfHandle>();
}

fn handle_animating(
    player: Single<
        (
            &TnuaController,
            &AnimationEntityLink,
            &mut TnuaAnimatingState<PlayerAnimationState>,
        ),
        With<LocalPlayer>,
    >,
    mut q_animation_players: Query<&mut AnimationPlayer>,
    animation_nodes: Option<Res<PlayerAnimationNodes>>,
) {
    let animation_nodes = match animation_nodes {
        Some(nodes) => nodes,
        None => return,
    };

    let (controller, animation_link, mut animating_state) = player.into_inner();

    let mut animation_player = q_animation_players
        .get_mut(animation_link.0)
        .expect("Animation player not found");

    let current_status_for_animation = match controller.action_name() {
        Some(TnuaBuiltinJump::NAME) => {
            let (_, jump_state) = controller
                .concrete_action::<TnuaBuiltinJump>()
                .expect("Jump action not found");

            match jump_state {
                TnuaBuiltinJumpState::NoJump => return,
                TnuaBuiltinJumpState::FallSection => PlayerAnimationState::Falling,
                _ => PlayerAnimationState::Jumping,
            }
        }

        Some(other) => panic!("Unexpected action: {}", other),
        None => {
            let Some((_, basis_state)) = controller.concrete_basis::<TnuaBuiltinWalk>() else {
                return;
            };
            if basis_state.standing_on_entity().is_none() {
                PlayerAnimationState::Falling
            } else {
                let speed = basis_state.running_velocity.length();
                if speed > 0.01 {
                    // Epsilon check for floating point precision
                    if (speed - PLAYER_WALK_SPEED).abs() < 1e-4 {
                        PlayerAnimationState::Walking(0.3 * speed)
                    } else {
                        PlayerAnimationState::Running(0.20 * speed)
                    }
                } else {
                    PlayerAnimationState::Standing
                }
            }
        }
    };

    let animating_directive = animating_state.update_by_discriminant(current_status_for_animation);

    match animating_directive {
        TnuaAnimatingStateDirective::Maintain { state } => {
            if let PlayerAnimationState::Walking(speed) = state {
                if let Some(animation) = animation_player.animation_mut(animation_nodes.walking) {
                    animation.set_speed(*speed);
                }
            }
        }
        TnuaAnimatingStateDirective::Alter {
            old_state: _,
            state,
        } => {
            animation_player.stop_all();

            match state {
                PlayerAnimationState::Standing => {
                    animation_player
                        .start(animation_nodes.standing)
                        .set_speed(1.0)
                        .repeat();
                }
                PlayerAnimationState::Walking(speed) => {
                    animation_player
                        .start(animation_nodes.walking)
                        .set_speed(*speed)
                        .repeat();
                }
                PlayerAnimationState::Running(speed) => {
                    animation_player
                        .start(animation_nodes.running)
                        .set_speed(*speed)
                        .repeat();
                }
                PlayerAnimationState::Jumping => {}
                PlayerAnimationState::Falling => {}
            }
        }
    }
}
