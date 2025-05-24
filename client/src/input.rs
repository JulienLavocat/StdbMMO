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
}

pub fn create_input_map() -> InputMap<Actions> {
    let mut input_map = InputMap::<Actions>::default();

    input_map.insert_dual_axis(Actions::Move, VirtualDPad::wasd());

    input_map.insert(Actions::Jump, KeyCode::Space);
    input_map.insert(Actions::Run, KeyCode::ShiftLeft);
    input_map.insert(Actions::Look, MouseButton::Right);

    input_map
}
