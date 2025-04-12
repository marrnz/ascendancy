use ascendancy_shared::{ClientNetworkMessage, bincode_config};
use bevy::prelude::{EventReader, NextState, Res, ResMut, State, info, EventWriter};
use bevy_renet::renet::{DefaultChannel, RenetServer, ServerEvent};
use crate::ClientStateTransitionEvent;
use crate::state::state::GameState;

pub fn receive_reliable_ordered_client_messages(mut server: ResMut<RenetServer>) {
    // TODO: Can this method of sequentially iterating clients cause input lag for the player?
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            let (decoded, _): (ClientNetworkMessage, usize) =
                bincode::decode_from_slice(&message[..], bincode_config())
                    .expect("Error decoding reliable ordered client messages");
            match decoded {
                ClientNetworkMessage::PlayerInput { .. } => todo!(),
                _ => panic!("Received unexpected message {:?} for channel type {}", &message, "reliable ordered'")
            }
        }
    }
}

pub fn receive_reliable_unordered_client_messages(mut server: ResMut<RenetServer>, mut state_transitions: EventWriter<ClientStateTransitionEvent>) {
    // TODO: Can this method of sequentially iterating clients cause input lag for the player?
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableUnordered)
        {
            let (decoded, _): (ClientNetworkMessage, usize) =
                bincode::decode_from_slice(&message[..], bincode_config())
                    .expect("Error decoding reliable ordered client messages");
            info!("Receiving client message {:?}", &decoded);
            match decoded {
                ClientNetworkMessage::StateTransition {target_state} => {
                    state_transitions.send(ClientStateTransitionEvent{client_id, target_state});
                },
                _ => panic!("Received unexpected message {:?} for channel type {}", &message, "reliable unordered'")
            }
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
                GameState::WaitingForFullLobby => next_state.set(GameState::GenerateWorld),
                _ => panic!(
                    "Server crash because second player connected when not in waiting for full lobby"
                ),
            }
        }
    }
}
