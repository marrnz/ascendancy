
use bevy::prelude::{Commands, EventReader, EventWriter, Query, Res, Time, Vec2, With};
use bevy_renet::renet::RenetServer;
use ascendancy_shared::{Player, PlayerInputType, Position, PreviousPosition};
use crate::netcode::components::Client;

const VELOCITY: f32 = 200.0;

pub fn spawn_player(mut commands: Commands, server: Res<RenetServer>) {
    server.clients_id_iter().for_each(|client_id| {
        commands.spawn((Player, Client(client_id), Position(Vec2::ZERO), PreviousPosition(Vec2::ZERO)));
    });
}

/*
pub fn validate_player_inputs(
    mut player_input_attempts: EventReader<PlayerInputAttempt>,
    mut confirmed_player_input_queue: EventWriter<PlayerInputConfirmed>,
) {
    // TODO: Check wall bounds for example
    for player_input_attempt in player_input_attempts.read() {
        confirmed_player_input_queue.send(PlayerInputConfirmed::from(player_input_attempt));
    }
}

pub fn handle_player_input_events(
    time: Res<Time>,
    mut confirmed_player_inputs: EventReader<PlayerInputConfirmed>,
    mut player_position: Query<(&mut Position, &mut PreviousPosition), With<Player>>,
) {
    for confirmed_player_input in confirmed_player_inputs.read() {
        if let PlayerInputType::MoveAttempt(dir) = confirmed_player_input.input_type {
            if let Ok((mut position, mut previous_position)) = player_position.get_mut(confirmed_player_input.entity) {
                previous_position.0 = position.0;
                position.0 += dir.as_vec2() * VELOCITY * time.delta().as_secs_f32();
            }
        }
    }
}
*/