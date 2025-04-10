
use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, OnEnter, Update, in_state};
use bevy_asset_loader::loading_state::LoadingStateAppExt;
use bevy_asset_loader::prelude::{ConfigureLoadingState, LoadingStateConfig};
use ascendancy_shared::ClientGameState;
use crate::player::render::assets::PlayerAssets;
use crate::player::render::systems::{spawn_player_graphics, update_player_position};

mod assets;
mod systems;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(ClientGameState::AssetLoading).load_collection::<PlayerAssets>(),
        )
        .add_systems(OnEnter(ClientGameState::ConnectingToServer), spawn_player_graphics)
        .add_systems(
            Update,
            update_player_position.run_if(in_state(ClientGameState::ConnectingToServer)),
        );
    }
}
