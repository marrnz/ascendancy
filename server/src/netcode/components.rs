use bevy::prelude::Component;
use bevy_renet::renet::ClientId;

#[derive(Component)]
pub struct Client(pub ClientId);