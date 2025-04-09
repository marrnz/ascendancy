use std::net::{UdpSocket};
use ascendancy_shared::{PlayerInputAttempt, PlayerInputConfirmed, PROTOCOL_ID};
use bevy::{DefaultPlugins, MinimalPlugins};
use bevy::app::{App, ScheduleRunnerPlugin};
use bevy::prelude::{info, PluginGroup};
use bevy_renet::RenetServerPlugin;
use std::time::{Duration, SystemTime};
use bevy::log::LogPlugin;
use bevy_renet::netcode::{NetcodeServerPlugin, NetcodeServerTransport, ServerAuthentication, ServerConfig};
use bevy_renet::netcode::NetcodeTransportError::Netcode;
use bevy_renet::renet::{ConnectionConfig, RenetServer};
use crate::netcode::NetcodePlugin;

mod map;
mod player;
mod netcode;

pub fn main() {
    info!("Starting...");
    let server = RenetServer::new(ConnectionConfig::default());
    info!("Server created...");
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 128.0,
            ))),
        )
        .add_plugins(LogPlugin::default())
        .add_plugins(RenetServerPlugin)
        .add_plugins(NetcodeServerPlugin)
        .add_plugins(NetcodePlugin)
        .insert_resource(server)
        .insert_resource(create_renet_netcode_server_transport())
        .add_event::<PlayerInputAttempt>()
        .add_event::<PlayerInputConfirmed>()
        .run();
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