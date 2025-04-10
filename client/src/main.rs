use std::net::UdpSocket;
use std::time::SystemTime;
use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_renet::netcode::{ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport};
use bevy_renet::renet::{ConnectionConfig, RenetClient};
use bevy_renet::RenetClientPlugin;
use ascendancy_shared::{ClientGameState, PlayerInputAttempt, PlayerInputConfirmed, PROTOCOL_ID};

mod camera;
mod map;
mod player;

fn main() {
    let client = RenetClient::new(ConnectionConfig::default());
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RenetClientPlugin)
        .add_plugins(NetcodeClientPlugin)
        .insert_resource(client)
        .insert_resource(create_netcode_client_transport())
        .init_state::<ClientGameState>()
        .add_loading_state(
            LoadingState::new(ClientGameState::AssetLoading).continue_to_state(ClientGameState::ConnectingToServer),
        )
        .add_event::<PlayerInputAttempt>()
        .add_event::<PlayerInputConfirmed>()
        .run();
}

fn create_netcode_client_transport() -> NetcodeClientTransport{
    let server_address = "127.0.0.1:5000".parse().unwrap();
    let authentication = ClientAuthentication::Unsecure {
        server_addr: server_address,
        client_id: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64,
        user_data: None,
        protocol_id: PROTOCOL_ID,
    };
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    NetcodeClientTransport::new(current_time, authentication, socket).unwrap()
}