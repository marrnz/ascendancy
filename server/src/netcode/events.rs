use bevy::prelude::{Event};
use ascendancy_shared::{NetworkMessageType};

#[derive(Event)]
pub struct ClientMessage {
    pub message_type: NetworkMessageType
}