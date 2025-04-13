use crate::netcode::components::Client;
use crate::states::resources::Lobby;
use ascendancy_shared::{Player, Position, PreviousPosition};
use bevy::prelude::{Commands, Res, Vec2};

pub fn spawn_players(mut commands: Commands, lobby: Res<Lobby>) {
    for client in lobby.players.keys() {
        commands.spawn((Player, Client(*client), Position(Vec2::ZERO), PreviousPosition(Vec2::ZERO)));
    }
}