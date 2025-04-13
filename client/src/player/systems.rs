use crate::{PlayerSpawned, StartPlayerVsPlayerEvent};
use ascendancy_shared::{Opponent, Player, PreviousPosition};
use bevy::prelude::{Commands, Entity, EventReader, Query, With};

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
