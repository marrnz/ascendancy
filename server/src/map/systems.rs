use bevy::prelude::*;
use ascendancy_shared::{Map, Position, Tile};

pub fn spawn_map(map: Res<Map>, mut commands: Commands) {
    for (index, _) in map.tiles.iter().enumerate() {
        let (x, y) = map.coordinates(index);
        commands.spawn((Tile, Position(Vec2::new(x as f32, y as f32))));
    }
}
