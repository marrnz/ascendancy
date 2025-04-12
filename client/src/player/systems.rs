use crate::PlayerSpawned;
use ascendancy_shared::PlayerInputType::MoveAttempt;
use ascendancy_shared::{Player, Position, PreviousPosition};
use bevy::input::ButtonInput;
use bevy::math::Dir2;
use bevy::prelude::{Commands, Entity, EventReader, EventWriter, KeyCode, Query, Res, Vec2, With};
use std::f32::consts::FRAC_1_SQRT_2;

pub fn insert_player_entity(
    mut commands: Commands,
    mut player_spawned: EventReader<PlayerSpawned>,
) {
    if let Some(player_spawned) = player_spawned.read().next() {
        commands.spawn((
            Player,
            player_spawned.position.clone(),
            player_spawned.position.clone(),
        ));
    }
}

/*
pub fn player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<Player>>,
    mut player_input_events: EventWriter<PlayerInputAttempt>,
) {
    let up = keyboard_input.pressed(KeyCode::KeyW);
    let down = keyboard_input.pressed(KeyCode::KeyS);
    let left = keyboard_input.pressed(KeyCode::KeyA);
    let right = keyboard_input.pressed(KeyCode::KeyD);

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

    if dir == Vec2::ZERO {
        return;
    }

    let dir = Dir2::new(dir).expect("Should never be an invalid direction");

    let player_entity = query
        .get_single()
        .expect("There should always only be one player!");

    player_input_events.send(PlayerInputAttempt {
        entity: player_entity,
        input_type: MoveAttempt(dir),
    });
}
*/
