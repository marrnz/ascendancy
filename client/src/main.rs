use crate::camera::CameraPlugin;
use crate::map::MapPlugin;
use crate::netcode::NetcodePlugin;
use crate::player::PlayerPlugin;
use crate::state::StatePlugin;
use ascendancy_shared::{ClientGameState, Map, Position};
use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};

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
        .add_plugins(PlayerPlugin)
        .add_plugins(MapPlugin)
        .add_plugins(CameraPlugin)
        .init_state::<ClientGameState>()
        .add_event::<MapSpawned>()
        .add_event::<PlayerSpawned>()
        .add_event::<StartPlayerVsEnvEvent>()
        .add_event::<StartPlayerVsPlayerEvent>()
        .run();
}

#[derive(Event)]
pub struct MapSpawned {
    map: Map,
}

#[derive(Event)]
pub struct PlayerSpawned {
    position: Position,
}

#[derive(Event)]
pub struct StartPlayerVsEnvEvent;

#[derive(Event)]
pub struct StartPlayerVsPlayerEvent {
    position: Position,
    opponent_position: Position,
}
