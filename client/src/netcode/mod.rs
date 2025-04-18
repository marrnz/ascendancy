use crate::netcode::systems::{receive_reliable_unordered_server_messages, receive_unreliable_server_messages};
use ascendancy_shared::PROTOCOL_ID;
use bevy::app::{App, Plugin, PreUpdate};
use bevy_renet::RenetClientPlugin;
use bevy_renet::netcode::{ClientAuthentication, NetcodeClientPlugin, NetcodeClientTransport};
use bevy_renet::renet::{ConnectionConfig, RenetClient};
use std::net::UdpSocket;
use std::time::SystemTime;

pub mod network_handler;
mod systems;

pub struct NetcodePlugin;

impl Plugin for NetcodePlugin {
    fn build(&self, app: &mut App) {
        let client = RenetClient::new(ConnectionConfig::default());
        app.add_plugins(RenetClientPlugin)
            .add_plugins(NetcodeClientPlugin)
            .insert_resource(client)
            .insert_resource(create_netcode_client_transport())
            .add_systems(PreUpdate, receive_reliable_unordered_server_messages)
            .add_systems(PreUpdate, receive_unreliable_server_messages);
    }
}

fn create_netcode_client_transport() -> NetcodeClientTransport {
    let server_address = "127.0.0.1:5000".parse().unwrap();
    let authentication = ClientAuthentication::Unsecure {
        server_addr: server_address,
        client_id: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
        user_data: None,
        protocol_id: PROTOCOL_ID,
    };
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    NetcodeClientTransport::new(current_time, authentication, socket).unwrap()
}
