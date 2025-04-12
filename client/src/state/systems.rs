use bevy::log::{ info};
use bevy::prelude::{NextState, Res, ResMut};
use bevy_renet::renet::RenetClient;
use ascendancy_shared::{ClientGameState, ClientNetworkMessage};
use crate::netcode::network_handler;

pub fn check_server_connection(
    client: Res<RenetClient>,
    mut next_state: ResMut<NextState<ClientGameState>>,
) {
    if client.is_connected() {
        info!("Server connection established");
        next_state.set(ClientGameState::WaitingForFullLobby);
    }
}

pub fn set_spawning_state(mut next_state: ResMut<NextState<ClientGameState>>) {
    next_state.set(ClientGameState::Spawning);
}

pub fn send_waiting_for_lobby_state_message(mut client: ResMut<RenetClient>) {
    info!("Sending waiting for lobby state");
    let waiting_for_lobby_message = ClientNetworkMessage::StateTransition {
        target_state: ClientGameState::WaitingForFullLobby,
    };
    network_handler::send(&mut client, waiting_for_lobby_message);
}

pub fn send_pve_ready_message(mut client: ResMut<RenetClient>) {
    info!("Sending pve ready");
    let pve_ready_message = ClientNetworkMessage::StateTransition {
        target_state: ClientGameState::PlayerVsEnvironment,
    };
    network_handler::send(&mut client, pve_ready_message);
}

pub fn transition_to_pve_state(mut next_state: ResMut<NextState<ClientGameState>>) {
    info!("Transitioning to pve state");
    next_state.set(ClientGameState::PlayerVsEnvironment);
}