use crate::client::map::render::assets::MapAssets;
use crate::client::map::render::systems::render;
use crate::GameState;
use bevy::app::{App, Plugin};
use bevy::prelude::{OnEnter, Update};
use bevy_asset_loader::prelude::{ConfigureLoadingState, LoadingState, LoadingStateAppExt};

mod assets;
mod systems;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Main)
                .load_collection::<MapAssets>(),
        )
        .add_systems(OnEnter(GameState::Main), render);
    }
}
