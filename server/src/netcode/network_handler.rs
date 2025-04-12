use ascendancy_shared::{ServerNetworkMessage, bincode_config};
use bevy::prelude::{error, info};
use bevy_renet::renet::{ClientId, DefaultChannel, RenetServer};
use bincode::encode_to_vec;

pub fn send(server: &mut RenetServer, client: ClientId, message: ServerNetworkMessage) {
    let encoded_message = encode_to_vec(&message, bincode_config())
        .expect(format!("Error encoding message {:?}", message).as_str());
    // Decide whether broadcast or not, if client add it to enum variant
    match message {
        ServerNetworkMessage::WaitingForPlayers {  .. } => {
            if !server.can_send_message(
                client,
                DefaultChannel::ReliableUnordered,
                encoded_message.len(),
            ) {
                error!("Error sending message {:?}", &message);
            }
            server.send_message(client, DefaultChannel::ReliableUnordered, encoded_message);
        }
    }
}
