use crate::server::player::systems::spawn_player;
use bevy::app::{App, Plugin};
use bevy::prelude::Startup;

mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}
