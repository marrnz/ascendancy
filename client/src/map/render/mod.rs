
use bevy::app::{App, Plugin};
use bevy::prelude::{any_with_component, in_state, not, run_once, Condition, IntoSystemConfigs, Update};
use bevy_asset_loader::prelude::{ConfigureLoadingState, LoadingStateAppExt, LoadingStateConfig};
use ascendancy_shared::{ClientGameState, Tile};
use crate::map::render::assets::MapAssets;
use crate::map::render::systems::render_map;

mod assets;
mod systems;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(ClientGameState::AssetLoading).load_collection::<MapAssets>(),
        )
        .add_systems(Update, render_map.run_if(any_with_component::<Tile>.and(run_once)));
    }
}
