use bevy::prelude::*;
use leafwing_input_manager::{
    Actionlike,
    prelude::{InputMap, VirtualDPad},
};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Actions {
    #[actionlike(DualAxis)]
    Move,
    Jump,
    Run,
    Look,

    // Debug actions
    DebugTogglePlayerWindowGizmos,
    DebugTogglePhysicsGizmos,
}

pub fn create_input_map() -> InputMap<Actions> {
    let mut input_map = InputMap::<Actions>::new([
        (Actions::Jump, KeyCode::Space),
        (Actions::Run, KeyCode::ShiftLeft),
        (Actions::DebugTogglePlayerWindowGizmos, KeyCode::F12),
        (Actions::DebugTogglePhysicsGizmos, KeyCode::F11),
    ]);

    input_map.insert_dual_axis(Actions::Move, VirtualDPad::wasd());

    input_map.insert(Actions::Look, MouseButton::Right);

    input_map
}
