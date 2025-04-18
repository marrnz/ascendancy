use crate::ClientStateTransitionEvent;
use crate::player::events::PlayerInput;
use ascendancy_shared::{ClientNetworkMessage, bincode_config};
use bevy::prelude::{EventReader, EventWriter, Res, ResMut, State, info};
use bevy_renet::renet::{DefaultChannel, RenetServer, ServerEvent};
use crate::netcode::components::Client;

pub fn receive_reliable_ordered_client_messages(
    mut server: ResMut<RenetServer>,
    mut player_inputs: EventWriter<PlayerInput>,
) {
    // TODO: Can this method of sequentially iterating clients cause input lag for the player?
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            let (decoded, _): (ClientNetworkMessage, usize) =
                bincode::decode_from_slice(&message[..], bincode_config())
                    .expect("Error decoding reliable ordered client messages");
            match decoded {
                ClientNetworkMessage::PlayerInput { key_code } => {
                    // TODO: Is it important to separate events of each player (and process them in parallel)?!
                    player_inputs.send(PlayerInput { key_code, client: Client(client_id) });
                }
                _ => panic!(
                    "Received unexpected message {:?} for channel type {}",
                    &message, "reliable ordered'"
                ),
            }
        }
    }
}

pub fn receive_reliable_unordered_client_messages(
    mut server: ResMut<RenetServer>,
    mut state_transitions: EventWriter<ClientStateTransitionEvent>,
) {
    // TODO: Can this method of sequentially iterating clients cause input lag for the player?
    for client_id in server.clients_id() {
        while let Some(message) =
            server.receive_message(client_id, DefaultChannel::ReliableUnordered)
        {
            let (decoded, _): (ClientNetworkMessage, usize) =
                bincode::decode_from_slice(&message[..], bincode_config())
                    .expect("Error decoding reliable ordered client messages");
            info!("Receiving client message {:?}", &decoded);
            match decoded {
                ClientNetworkMessage::StateTransition { target_state } => {
                    state_transitions.send(ClientStateTransitionEvent {
                        client_id,
                        target_state,
                    });
                }
                _ => panic!(
                    "Received unexpected message {:?} for channel type {}",
                    &message, "reliable unordered'"
                ),
            }
        }
    }
}

pub fn handle_server_events(mut events: EventReader<ServerEvent>) {
    for event in events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => info!("Client connected: {}", client_id),
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Disconnected {} because {}!", client_id, reason)
            }
        }
    }
}
