use crate::client::ClientPlugin;
use crate::server::{PlayerInputConfirmed, ServerPlugin};
use crate::shared::PlayerInputAttempt;
use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};

mod client;
mod server;
pub mod shared;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Main),
        )
        .add_event::<PlayerInputAttempt>()
        .add_event::<PlayerInputConfirmed>()
        .add_plugins(ServerPlugin)
        .add_plugins(ClientPlugin)
        .run();
}

#[derive(Default, States, Eq, Clone, Debug, Hash, PartialEq)]
pub enum GameState {
    #[default]
    AssetLoading,
    Main,
}
