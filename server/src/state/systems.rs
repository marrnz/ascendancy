use crate::ClientStateTransitionEvent;
use crate::netcode::network_handler;
use crate::state::resources::Lobby;
use crate::state::state::GameState;
use ascendancy_shared::{ClientGameState, Map, Player, Position, ServerNetworkMessage, Tile};
use bevy::prelude::{EventReader, NextState, Query, Res, ResMut, With};
use bevy_renet::renet::RenetServer;
use crate::netcode::components::Client;

pub fn update_lobby_state(
    mut client_state_transitions: EventReader<ClientStateTransitionEvent>,
    mut lobby: ResMut<Lobby>,
) {
    for client_state_transition in client_state_transitions.read() {
        lobby
            .get_or_insert(client_state_transition.client_id)
            .client_game_state = client_state_transition.target_state.clone();
    }
}

pub fn check_for_waiting_for_players_transition(
    lobby: Res<Lobby>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if lobby
        .players
        .values()
        .map(|lobby_state| &lobby_state.client_game_state)
        .all(|client_game_state| *client_game_state == ClientGameState::WaitingForFullLobby)
    {
        next_state.set(GameState::WaitingForPlayersReady);
    }
}

pub fn send_waiting_for_players_message(
    mut server: ResMut<RenetServer>,
    map: Res<Map>,
    players: Query<(&Client, &Position), With<Player>>,
) {
    for (client, position) in players.iter() {
        network_handler::send(
            server.as_mut(),
            client.0,
            ServerNetworkMessage::WaitingForPlayers {
                player_position: position.clone(),
                map: map.clone(),
            },
        );
    }
}
