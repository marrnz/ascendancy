use crate::netcode::components::Client;
use crate::netcode::network_handler;
use crate::player::events::PlayerInput;
use crate::states::resources::Lobby;
use ascendancy_shared::{EncodedKeyCode, Player, Position, PreviousPosition, ServerNetworkMessage};
use bevy::prelude::{info, Commands, Dir2, EventReader, Query, Res, ResMut, Vec2, With};
use bevy::time::Time;
use bevy_renet::renet::{ClientId, RenetClient, RenetServer};
use std::f32::consts::FRAC_1_SQRT_2;

const VELOCITY: f32 = 200.0;

pub fn spawn_players(mut commands: Commands, lobby: Res<Lobby>) {
    for client in lobby.players.keys() {
        commands.spawn((
            Player,
            Client(*client),
            Position(Vec2::ZERO),
            PreviousPosition(Vec2::ZERO),
        ));
    }
}

pub fn send_update_player_state(
    mut server: ResMut<RenetServer>,
    players: Query<(&Position, &Client), With<Player>>,
) {
    let (player_one_pos, player_one_client) = players
        .iter()
        .next()
        .expect("There should be at least one player");
    let (player_two_pos, player_two_client) = players
        .iter()
        .next()
        .expect("There should be a second player");
    network_handler::send(
        server.as_mut(),
        player_one_client.0,
        ServerNetworkMessage::PlayerMovement {
            player_position: player_one_pos.clone(),
            opponent_position: player_two_pos.clone(),
        },
    );
    network_handler::send(
        server.as_mut(),
        player_two_client.0,
        ServerNetworkMessage::PlayerMovement {
            player_position: player_two_pos.clone(),
            opponent_position: player_one_pos.clone(),
        },
    );
}

pub fn handle_player_inputs(
    time: Res<Time>,
    lobby: Res<Lobby>,
    mut player_input: EventReader<PlayerInput>,
    mut players: Query<(&mut Position, &Client), With<Player>>,
) {
    lobby
        .players
        .keys()
        .map(|lobby_client| {
            let collected_key_codes = player_input
                .read()
                .into_iter()
                .filter(|input| input.client.0 == *lobby_client)
                .map(|input| input.key_code)
                .collect::<Vec<EncodedKeyCode>>();
            (lobby_client, collected_key_codes)
        })
        .map(|(lobby_client, collected_key_codes)| {
            let dir = parse_player_movement_input(collected_key_codes);
            (lobby_client, dir)
        })
        .for_each(|(lobby_client, dir)| {
            apply_movement_to_player(time.as_ref(), lobby_client, dir, &mut players)
        });
}

fn apply_movement_to_player(
    time: &Time,
    lobby_client: &ClientId,
    direction: Vec2,
    players: &mut Query<(&mut Position, &Client), With<Player>>,
) {
    if direction == Vec2::ZERO {
        return;
    }
    let dir = Dir2::new(direction).expect("Should never be an invalid direction");
    for (mut position, client) in players.iter_mut() {
        if &client.0 == lobby_client {
            position.0 += dir.as_vec2() * VELOCITY * time.delta().as_secs_f32();
        }
    }
}

fn parse_player_movement_input(inputs: Vec<EncodedKeyCode>) -> Vec2 {
    let up = inputs.contains(&EncodedKeyCode::KeyW);
    let down = inputs.contains(&EncodedKeyCode::KeyS);
    let left = inputs.contains(&EncodedKeyCode::KeyA);
    let right = inputs.contains(&EncodedKeyCode::KeyD);

    let dir = match (up, down, left, right) {
        (true, false, false, false) => Vec2::Y,
        (false, true, false, false) => Vec2::NEG_Y,
        (false, false, true, false) => Vec2::NEG_X,
        (false, false, false, true) => Vec2::X,
        (true, false, true, false) => Vec2::new(-FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        (true, false, false, true) => Vec2::new(FRAC_1_SQRT_2, FRAC_1_SQRT_2),
        (false, true, true, false) => Vec2::new(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
        (false, true, false, true) => Vec2::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2),
        _ => Vec2::ZERO,
    };
    dir
}
