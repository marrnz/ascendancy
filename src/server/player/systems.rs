use bevy::prelude::{Commands, Vec2};
use crate::shared::{Player, Position};

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((Player, Position(Vec2::new(0., 0.))));
}
