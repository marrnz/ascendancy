use crate::server::PlayerInputConfirmed;
use crate::shared::{Player, PlayerInputAttempt, PlayerInputType, Position};
use bevy::prelude::{Commands, EventReader, EventWriter, Query, Vec2, With};

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((Player, Position(Vec2::new(0., 0.))));
}

pub fn validate_player_inputs(
    mut player_input_attempts: EventReader<PlayerInputAttempt>,
    mut confirmed_player_input_queue: EventWriter<PlayerInputConfirmed>,
) {
    // TODO: Check wall bounds
    for player_input_attempt in player_input_attempts.read() {
        confirmed_player_input_queue.send(PlayerInputConfirmed::from(player_input_attempt));
    }
}

pub fn handle_player_input_events(
    mut confirmed_player_inputs: EventReader<PlayerInputConfirmed>,
    mut transforms: Query<&mut Position, With<Player>>,
) {
    for confirmed_player_input in confirmed_player_inputs.read() {
        if let PlayerInputType::MoveAttempt(dir) = confirmed_player_input.input_type {
            if let Ok(mut position) = transforms.get_mut(confirmed_player_input.entity) {
                position.0 = position.0 + dir.as_vec2();
            }
        }
    }
}
