use bevy::prelude::*;
use bevy_asset_loader::{
    asset_collection::AssetCollection,
    loading_state::{LoadingState, LoadingStateAppExt, config::ConfigureLoadingState},
};

use crate::state::GameState;

#[derive(AssetCollection, Resource)]
pub struct CharacterAssets {
    #[asset(path = "character.glb#Scene0")]
    pub character_scene: Handle<Scene>,
    #[asset(path = "character.glb")]
    pub character: Handle<Gltf>,
}

#[derive(AssetCollection, Resource)]
pub struct NameplateAssets {
    #[asset(path = "fonts/GeistMono-Regular.ttf")]
    pub font: Handle<Font>,
}

pub struct LoadWorldPlugin;

impl Plugin for LoadWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::LoadingWorld)
                .continue_to_state(GameState::InGame)
                .load_collection::<NameplateAssets>()
                .load_collection::<CharacterAssets>(),
        );
    }
}
