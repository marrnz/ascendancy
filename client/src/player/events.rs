use bevy::prelude::{Event, KeyCode};
use ascendancy_shared::Position;

#[derive(Event)]
pub struct PlayerInputJustPressed {
    pub key_code: KeyCode,
}

#[derive(Event)]
pub struct PlayerStateConfirmed {
    pub player_pos: Position
}

#[derive(Event)]
pub struct OpponentStateConfirmed {
    pub opponent_pos: Position
}