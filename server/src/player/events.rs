use bevy::prelude::{Event};
use ascendancy_shared::EncodedKeyCode;
use crate::netcode::components::Client;

#[derive(Event)]
pub struct PlayerInput {
    pub key_code: EncodedKeyCode,
    pub client: Client
}