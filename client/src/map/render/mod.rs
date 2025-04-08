use crate::GameState;
use bevy::app::{App, Plugin};
use bevy::prelude::OnEnter;
use bevy_asset_loader::prelude::{ConfigureLoadingState, LoadingStateAppExt, LoadingStateConfig};
use crate::map::render::assets::MapAssets;
use crate::map::render::systems::render_map;

mod assets;
mod systems;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(GameState::AssetLoading).load_collection::<MapAssets>(),
        )
        .add_systems(OnEnter(GameState::Main), render_map);
    }
}
