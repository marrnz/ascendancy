use crate::netcode::network_message_handler;
use ascendancy_shared::{ClientGameState, ClientNetworkMessage};
use bevy::prelude::{NextState, Res, ResMut};
use bevy_renet::renet::RenetClient;

pub fn check_server_connection(
    client: Res<RenetClient>,
    mut next_state: ResMut<NextState<ClientGameState>>,
) {
    if client.is_connected() {
        next_state.set(ClientGameState::WaitingForFullLobby);
    }
}

pub fn send_waiting_for_lobby_state_message(mut client: ResMut<RenetClient>) {
    let waiting_for_lobby_message = ClientNetworkMessage::StateTransition {
        target_state: ClientGameState::WaitingForFullLobby,
    };
    network_message_handler::send(waiting_for_lobby_message, &mut client);
}
