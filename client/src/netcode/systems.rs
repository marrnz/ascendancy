use crate::{MapSpawned, PlayerSpawned};
use ascendancy_shared::{ServerNetworkMessage, bincode_config};
use bevy::prelude::{info, EventWriter, ResMut};
use bevy_renet::renet::{DefaultChannel, RenetClient};

pub fn receive_reliable_unordered_server_messages(
    mut client: ResMut<RenetClient>,
    mut player_spawned: EventWriter<PlayerSpawned>,
    mut map_spawned: EventWriter<MapSpawned>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableUnordered) {
        let (decoded, _): (ServerNetworkMessage, usize) =
            bincode::decode_from_slice(&message[..], bincode_config())
                .expect("Error decoding reliable ordered server messages");
        info!("Receiving server message {:?}", &decoded);
        match decoded {
            ServerNetworkMessage::WaitingForPlayers {
                player_position,
                map,
            } => {
                player_spawned.send(PlayerSpawned {
                    position: player_position,
                });
                map_spawned.send(MapSpawned { map });
            }
            _ => panic!(
                "Received unexpected message {:?} for channel type {}",
                &message, "reliable unordered'"
            ),
        }
    }
}
