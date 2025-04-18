use ascendancy_shared::{ClientNetworkMessage, bincode_config};
use bevy::log::error;
use bevy::prelude::info;
use bevy_renet::renet::{DefaultChannel, RenetClient};
use bincode::encode_to_vec;

pub fn send(client: &mut RenetClient, message: ClientNetworkMessage) {
    let encoded_message = encode_to_vec(&message, bincode_config())
        .expect(format!("Error encoding message {:?}", &message).as_str());
    match &message {
        ClientNetworkMessage::StateTransition { .. } => {
            if !client.can_send_message(DefaultChannel::ReliableUnordered, encoded_message.len()) {
                error!("Error sending message {:?}", &message);
            }
            client.send_message(DefaultChannel::ReliableUnordered, encoded_message);
        }
        ClientNetworkMessage::PlayerInput { .. } => {
            info!("Sending player input from server");
            if !client.can_send_message(DefaultChannel::ReliableOrdered, encoded_message.len()) {
                error!("Error sending message {:?}", &message);
            }
            client.send_message(DefaultChannel::ReliableOrdered, encoded_message);
        }
    };
}
