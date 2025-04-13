use ascendancy_shared::{ServerNetworkMessage, bincode_config};
use bevy::prelude::{error, info};
use bevy_renet::renet::{ClientId, DefaultChannel, RenetServer};
use bincode::encode_to_vec;

pub fn send(server: &mut RenetServer, client: ClientId, message: ServerNetworkMessage) {
    let encoded_message = encode_to_vec(&message, bincode_config())
        .unwrap_or_else(|_| panic!("Error encoding message {:?}", message));
    // Decide whether broadcast or not, if client add it to enum variant
    match message {
        ServerNetworkMessage::WaitingForPlayers { .. }
        | ServerNetworkMessage::StartPlayerVsPlayer { .. } => {
            if !server.can_send_message(
                client,
                DefaultChannel::ReliableUnordered,
                encoded_message.len(),
            ) {
                error!("Error sending message {:?}", &message);
            }
            server.send_message(client, DefaultChannel::ReliableUnordered, encoded_message);
        }
        _ => panic!("Trying to send a broadcast message {:?}", &encoded_message),
    }
}

pub fn broadcast(server: &mut RenetServer, message: ServerNetworkMessage) {
    let encoded_message = encode_to_vec(&message, bincode_config())
        .unwrap_or_else(|_| panic!("Error encoding message {:?}", message));
    match message {
        ServerNetworkMessage::StartPlayerVsEnvironment => {
            server.broadcast_message(DefaultChannel::ReliableUnordered, encoded_message);
        }
        _ => panic!("Trying to broadcast a send message {:?}", &encoded_message),
    }
}
