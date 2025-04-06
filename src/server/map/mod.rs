use crate::server::map::systems::spawn_map;
use crate::shared::Map;
use bevy::app::{App, Plugin, Startup};

pub mod resources;
mod systems;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Map::new(500, 500))
            .add_systems(Startup, spawn_map);
    }
}
