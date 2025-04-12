use crate::{MapSpawned};
use bevy::prelude::{Commands, EventReader};

pub fn insert_map_tile_entities(mut commands: Commands, mut map_spawned: EventReader<MapSpawned>) {
    if let Some(map_spawned) = map_spawned.read().next() {
        let map = &map_spawned.map;
        for tile in map.tiles.iter() {
            commands.spawn(tile.clone());
        }
    }
}
