use ascendancy_shared::{ClientGameState, PlayerInputAttempt, PlayerInputConfirmed};
use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_renet::netcode::NetcodeClientPlugin;
use crate::netcode::NetcodePlugin;

mod camera;
mod map;
mod player;

mod netcode;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NetcodePlugin)
        .init_state::<ClientGameState>()
        .add_loading_state(
            LoadingState::new(ClientGameState::AssetLoading)
                .continue_to_state(ClientGameState::ConnectingToServer),
        )
        .add_event::<PlayerInputAttempt>()
        .add_event::<PlayerInputConfirmed>()
        .run();
}
