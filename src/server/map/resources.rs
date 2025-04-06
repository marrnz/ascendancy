use crate::shared::{Map, TileType};

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut map = Self {
            width,
            tiles: Vec::with_capacity(width * height),
        };
        for x in 0..width {
            for y in 0..height {
                let mut tile_type = TileType::Floor;
                if x == 0 || x == width || y == 0 || y == height {
                    tile_type = TileType::Wall;
                }
                map.tiles.push(tile_type);
            }
        }
        map
    }
}
