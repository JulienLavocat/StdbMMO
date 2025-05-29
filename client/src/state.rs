use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    LoadingWorld,
    InGame,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoadingWorldSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InGameSet;

macro_rules! configure_all_schedules {
    ($app:expr, $set:expr) => {
        $app.configure_sets(Startup, $set);
        $app.configure_sets(First, $set);
        $app.configure_sets(PreUpdate, $set);
        $app.configure_sets(Update, $set);
        $app.configure_sets(PostUpdate, $set);
        $app.configure_sets(Last, $set);
        $app.configure_sets(FixedFirst, $set);
        $app.configure_sets(FixedPreUpdate, $set);
        $app.configure_sets(FixedUpdate, $set);
        $app.configure_sets(FixedPostUpdate, $set);
        $app.configure_sets(FixedLast, $set);
    };
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();

        configure_all_schedules!(
            app,
            LoadingWorldSet.run_if(in_state(GameState::LoadingWorld))
        );
        configure_all_schedules!(app, InGameSet.run_if(in_state(GameState::InGame)));
    }
}
