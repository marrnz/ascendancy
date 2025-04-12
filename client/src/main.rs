use ascendancy_shared::{ClientGameState, Map, Player, Position};
use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use crate::netcode::NetcodePlugin;
use crate::state::StatePlugin;

mod camera;
mod map;
mod player;

mod netcode;
mod state;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_loading_state(
            LoadingState::new(ClientGameState::AssetLoading)
                .continue_to_state(ClientGameState::ConnectingToServer),
        )
        // custom stuff
        .add_plugins(NetcodePlugin)
        .add_plugins(StatePlugin)
        .init_state::<ClientGameState>()
        .add_event::<MapSpawned>()
        .add_event::<PlayerSpawned>()
        .run();
}

#[derive(Event)]
pub struct MapSpawned {
    map: Map
}

#[derive(Event)]
pub struct PlayerSpawned {
    position: Position
}