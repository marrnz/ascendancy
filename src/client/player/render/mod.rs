
use crate::GameState;
use crate::client::player::render::systems::render_player;
use bevy::app::{App, Plugin};
use bevy::prelude::{OnEnter, Update};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_asset_loader::prelude::ConfigureLoadingState;
use crate::client::player::render::assets::PlayerAssets;

mod assets;
mod systems;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Main)
                .load_collection::<PlayerAssets>(),
        )
            .add_systems(OnEnter(GameState::Main), render_player);
    }
}
