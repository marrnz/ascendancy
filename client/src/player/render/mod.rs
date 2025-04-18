use crate::player::render::assets::PlayerAssets;
use crate::player::render::systems::{
    spawn_opponent_graphics, spawn_player_graphics, update_opponent_position,
    update_player_position,
};
use ascendancy_shared::{ClientGameState, Player};
use bevy::app::{App, Plugin};
use bevy::prelude::{
    Condition, IntoSystemConfigs, Update, any_with_component, in_state, not, run_once,
};
use bevy_asset_loader::loading_state::LoadingStateAppExt;
use bevy_asset_loader::prelude::{ConfigureLoadingState, LoadingStateConfig};

mod assets;
mod systems;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(ClientGameState::AssetLoading)
                .load_collection::<PlayerAssets>(),
        )
        .add_systems(
            Update,
            spawn_player_graphics.run_if(in_state(ClientGameState::Spawning).and(run_once)),
        )
        .add_systems(
            Update,
            (spawn_player_graphics, spawn_opponent_graphics)
                .run_if(in_state(ClientGameState::PlayerVsPlayer).and(run_once)),
        )
        .add_systems(
            Update,
            (
                update_opponent_position.run_if(
                    in_state(ClientGameState::PlayerVsEnvironment)
                        .and(in_state(ClientGameState::PlayerVsPlayer)),
                ),
                update_player_position.run_if(
                    in_state(ClientGameState::PlayerVsEnvironment)
                        .and(in_state(ClientGameState::PlayerVsPlayer)),
                ),
            ),
        );
    }
}
