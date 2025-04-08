use bevy::prelude::{EventReader, info};
use bevy_renet::renet::ServerEvent;

pub fn handle_server_events(mut events: EventReader<ServerEvent>) {
    for event in events.read() {
        match event {
            ServerEvent::ClientConnected {client_id} => info!("Connected {}!", client_id),
            ServerEvent::ClientDisconnected{client_id, reason} => {
                info!("Disconnected {} because {}!", client_id, reason)
            }
        }
    }
}
