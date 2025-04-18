use crate::ClientStateTransitionEvent;
use crate::netcode::components::Client;
use crate::netcode::network_handler;
use crate::states::resources::Lobby;
use crate::states::state::GameState;
use ascendancy_shared::{ClientGameState, Map, Player, Position, ServerNetworkMessage};
use bevy::math::Vec2;
use bevy::prelude::{EventReader, NextState, Query, Res, ResMut, With, info};
use bevy_renet::renet::RenetServer;

pub fn transition_to_waiting_for_players_ready(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::WaitingForPlayersReady);
}

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

pub fn check_for_player_vs_player_transition(mut next_state: ResMut<NextState<GameState>>) {
    // TODO: Conditions
    next_state.set(GameState::PlayerVsPlayer);
}

pub fn check_for_player_vs_env_transition(
    lobby: Res<Lobby>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if lobby.full
        && lobby
            .players
            .values()
            .map(|lobby_state| &lobby_state.client_game_state)
            .all(|client_game_state| *client_game_state == ClientGameState::PlayerVsEnvironment)
    {
        next_state.set(GameState::PlayerVsEnvironment);
    }
}

pub fn check_for_generate_world_transition(
    lobby: Res<Lobby>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if lobby.full
        && lobby
            .players
            .values()
            .map(|lobby_state| &lobby_state.client_game_state)
            .all(|client_game_state| *client_game_state == ClientGameState::WaitingForFullLobby)
    {
        next_state.set(GameState::GenerateWorld);
    }
}

pub fn send_player_vs_player_message(mut server: ResMut<RenetServer>) {
    server
        .clients_id()
        .iter()
        .enumerate()
        .for_each(|(index, client_id)| {
            let position_one = Vec2::new(10., 10.);
            let position_two = Vec2::new(50., 50.);
            let message = if index == 0 {
                ServerNetworkMessage::StartPlayerVsPlayer {
                    position: Position(position_one),
                    opponent_position: Position(position_two),
                }
            } else {
                ServerNetworkMessage::StartPlayerVsPlayer {
                    position: Position(position_two),
                    opponent_position: Position(position_one),
                }
            };
            network_handler::send(server.as_mut(), *client_id, message);
        });
}

pub fn send_player_vs_env_message(mut server: ResMut<RenetServer>) {
    network_handler::broadcast(
        server.as_mut(),
        ServerNetworkMessage::StartPlayerVsEnvironment,
    );
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
