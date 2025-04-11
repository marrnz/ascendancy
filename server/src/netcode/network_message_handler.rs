use bevy_renet::renet::{RenetServer};
use bincode::encode_to_vec;
use ascendancy_shared::{bincode_config, ServerNetworkMessage};

pub fn send(message: ServerNetworkMessage, server: &mut RenetServer) {
    let encoded_message = encode_to_vec(&message, bincode_config())
        .expect(format!("Error encoding message {:?}", message).as_str());
    // Decide whether broadcast or not, if client add it to enum variant
    todo!();
}
