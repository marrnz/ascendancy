use ascendancy_shared::{ClientNetworkMessage, bincode_config};
use bevy::log::{error, warn};
use bevy_renet::renet::{DefaultChannel, RenetClient};
use bincode::encode_to_vec;

pub fn send(message: ClientNetworkMessage, client: &mut RenetClient) {
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
            if !client.can_send_message(DefaultChannel::ReliableOrdered, encoded_message.len()) {
                error!("Error sending message {:?}", &message);
            }
            client.send_message(DefaultChannel::ReliableOrdered, encoded_message);
        }
        _ => panic!("Trying to send an unknown message type {:?}!", &message),
    };
}
