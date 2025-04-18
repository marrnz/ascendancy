use crate::player::events::{OpponentStateConfirmed, PlayerStateConfirmed};
use crate::{MapSpawned, PlayerSpawned, StartPlayerVsEnvEvent, StartPlayerVsPlayerEvent};
use ascendancy_shared::{ServerNetworkMessage, bincode_config};
use bevy::prelude::{EventWriter, ResMut, info};
use bevy_renet::renet::{DefaultChannel, RenetClient};

pub fn receive_reliable_unordered_server_messages(
    mut client: ResMut<RenetClient>,
    mut player_spawned: EventWriter<PlayerSpawned>,
    mut map_spawned: EventWriter<MapSpawned>,
    mut start_pve: EventWriter<StartPlayerVsEnvEvent>,
    mut start_pvp: EventWriter<StartPlayerVsPlayerEvent>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableUnordered) {
        let (decoded, _): (ServerNetworkMessage, usize) =
            bincode::decode_from_slice(&message[..], bincode_config())
                .expect("Error decoding reliable ordered server messages");
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
            ServerNetworkMessage::StartPlayerVsEnvironment => {
                info!("Received message from server {:?}", &decoded);
                start_pve.send(StartPlayerVsEnvEvent);
            }
            ServerNetworkMessage::StartPlayerVsPlayer {
                position: player_position,
                opponent_position,
            } => {
                start_pvp.send(StartPlayerVsPlayerEvent {
                    position: player_position,
                    opponent_position,
                });
            }
            _ => panic!(
                "Received unexpected message {:?} for channel type {}",
                &message, "reliable unordered'"
            ),
        }
    }
}

pub fn receive_unreliable_server_messages(
    mut client: ResMut<RenetClient>,
    mut player_state_confirmed: EventWriter<PlayerStateConfirmed>,
    mut opponent_state_confirmed: EventWriter<OpponentStateConfirmed>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
        let (decoded, _): (ServerNetworkMessage, usize) =
            bincode::decode_from_slice(&message[..], bincode_config())
                .expect("Error decoding unreliable server messages");
        match decoded {
            ServerNetworkMessage::PlayerMovement {
                player_position,
                opponent_position,
            } => {
                player_state_confirmed.send(PlayerStateConfirmed {
                    player_pos: player_position,
                });
                opponent_state_confirmed.send(OpponentStateConfirmed {
                    opponent_pos: opponent_position,
                });
            }
            _ => panic!(
                "Received unexpected message {:?} for channel type {}",
                &message, "unreliable'"
            ),
        }
    }
}
