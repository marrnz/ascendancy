use crate::netcode::network_handler;
use crate::player::events::{OpponentStateConfirmed, PlayerInputJustPressed, PlayerStateConfirmed};
use crate::{PlayerSpawned, StartPlayerVsPlayerEvent};
use ascendancy_shared::{
    ClientNetworkMessage, EncodedKeyCode, Opponent, Player, Position, PreviousPosition,
};
use bevy::prelude::{info, ButtonInput, Commands, Entity, EventReader, EventWriter, KeyCode, Query, Res, ResMut, With};
use bevy::reflect::map_apply;
use bevy_renet::renet::RenetClient;

pub fn insert_player_entity(
    mut commands: Commands,
    mut player_spawned: EventReader<PlayerSpawned>,
) {
    if let Some(player_spawned) = player_spawned.read().next() {
        commands.spawn((
            Player,
            player_spawned.position.clone(),
            PreviousPosition(player_spawned.position.0),
        ));
    }
}

pub fn insert_pvp_players(
    mut commands: Commands,
    pve_player: Query<Entity, With<Player>>,
    mut pvp_started: EventReader<StartPlayerVsPlayerEvent>,
) {
    for player in pve_player.iter() {
        commands.entity(player).despawn();
    }

    if let Some(pvp_started) = pvp_started.read().next() {
        commands.spawn((
            Player,
            pvp_started.position.clone(),
            PreviousPosition(pvp_started.position.0),
        ));
        commands.spawn((
            Opponent,
            pvp_started.opponent_position.clone(),
            PreviousPosition(pvp_started.opponent_position.0),
        ));
    }
}

pub fn update_player_pos_from_server(
    mut player_state_confirmed: EventReader<PlayerStateConfirmed>,
    mut query: Query<&mut Position, With<Player>>,
) {
    if let Some(player_state) = player_state_confirmed.read().next() {
        info!("Player State Event - Update Player Pos");
        let mut player_position = query
            .get_single_mut()
            .expect("There should be exactly one player!");

        player_position.0 = player_state.player_pos.0;
    }
}

pub fn update_opponent_pos_from_server(
    mut opponent_state_confirmed: EventReader<OpponentStateConfirmed>,
    mut query: Query<&mut Position, With<Opponent>>,
) {
    if let Some(opponent_state) = opponent_state_confirmed.read().next() {
        info!("Opponent State Event - Update Opponent Pos");
        for mut opponent_pos in query.iter_mut() {
            opponent_pos.0 = opponent_state.opponent_pos.0;
        }
    }
}

pub fn record_player_input(
    inputs: Res<ButtonInput<KeyCode>>,
    mut inputs_just_pressed: EventWriter<PlayerInputJustPressed>,
) {
    inputs
        .get_just_pressed()
        .into_iter()
        .map(|key_code| PlayerInputJustPressed {
            key_code: *key_code,
        })
        .for_each(|input| {
            inputs_just_pressed.send(input);
        });
}

pub fn send_player_input(
    mut client: ResMut<RenetClient>,
    mut inputs_just_pressed: EventReader<PlayerInputJustPressed>,
) {
    inputs_just_pressed
        .read()
        .map(|input| EncodedKeyCode::from(input.key_code))
        .map(|key_code| ClientNetworkMessage::PlayerInput { key_code })
        .for_each(|message| network_handler::send(&mut client, message));
}
