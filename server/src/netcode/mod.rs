use crate::netcode::systems::{handle_server_events, receive_reliable_ordered_client_messages, receive_reliable_unordered_client_messages};
use ascendancy_shared::PROTOCOL_ID;
use bevy::app::{App, Plugin};
use bevy::prelude::{FixedPreUpdate, IntoSystemConfigs, Update};
use bevy_renet::netcode::{NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::renet::{ConnectionConfig, RenetServer};
use bevy_renet::RenetServerPlugin;
use std::net::UdpSocket;
use std::time::SystemTime;

mod systems;
pub mod network_handler;
pub mod components;

pub struct NetcodePlugin;

impl Plugin for NetcodePlugin {
    fn build(&self, app: &mut App) {
        let server = RenetServer::new(ConnectionConfig::default());
        app.add_plugins(RenetServerPlugin)
            .add_plugins(NetcodeServerPlugin)
            .insert_resource(server)
            .insert_resource(create_renet_netcode_server_transport())
            .add_systems(Update, handle_server_events)
            .add_systems(FixedPreUpdate, receive_reliable_ordered_client_messages)
            .add_systems(FixedPreUpdate, receive_reliable_unordered_client_messages);
    }
}

fn create_renet_netcode_server_transport() -> NetcodeServerTransport {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let server_config = ServerConfig {
        current_time: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(),
        max_clients: 2,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![server_addr],
        authentication: ServerAuthentication::Unsecure
    };
    NetcodeServerTransport::new(server_config, socket).unwrap()
}

