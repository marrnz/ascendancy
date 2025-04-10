use crate::GameState;
use crate::netcode::events::ClientMessage;
use ascendancy_shared::bincode_config;
use bevy::prelude::{EventReader, EventWriter, NextState, Res, ResMut, State, info};
use bevy_renet::renet::{DefaultChannel, RenetServer, ServerEvent};
use bincode::error::DecodeError;

pub fn receive_reliable_ordered_client_messages(
    mut server: ResMut<RenetServer>,
    mut client_messages: EventWriter<ClientMessage>,
) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
        }
    }
}

pub fn handle_server_events(
    mut events: EventReader<ServerEvent>,
    server: Res<RenetServer>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => info!("Client connected: {}", client_id),
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Disconnected {} because {}!", client_id, reason)
            }
        }
        if server.connected_clients() == 2 {
            match state.get() {
                GameState::WaitingForFullLobby => next_state.set(GameState::WaitingForPlayersReady),
                _ => panic!(
                    "Server crash because second player connected when not in waiting for full lobby"
                ),
            }
        }
    }
}
