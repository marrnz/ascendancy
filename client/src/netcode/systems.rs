use ascendancy_shared::{ClientGameState, NetworkMessageType, bincode_config, ClientNetworkMessage};
use bevy::prelude::{NextState, Res, ResMut};
use bevy_renet::renet::{DefaultChannel, RenetClient};

pub fn check_server_connection(client: Res<RenetClient>, mut next_state: ResMut<NextState<ClientGameState>>) {
    if client.is_connected() {
        next_state.set(ClientGameState::WaitingForFullLobby);
    }
}

pub fn send_waiting_for_lobby_state_message(mut client: ResMut<RenetClient>) {
    let waiting_for_lobby_message = ClientNetworkMessage {
        message_type: NetworkMessageType::StateTransition {
            target_state: ClientGameState::WaitingForFullLobby,
        },
    };
    let encoded = bincode::encode_to_vec(&waiting_for_lobby_message, bincode_config())
        .expect("Error encoding waiting for lobby message");
    client.send_message(DefaultChannel::ReliableOrdered, encoded);
}
