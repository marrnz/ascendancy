
use bevy::prelude::{Added, Commands, Entity, Query, Res, Sprite, TextureAtlas, Transform};
use ascendancy_shared::{Map, Position, Tile, TileType, TILESIZE};
use crate::map::render::assets::MapAssets;

pub fn render_map(
    map: Res<Map>,
    map_textures: Res<MapAssets>,
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Tile>>,
) {
    let mut texture_atlas = TextureAtlas::from(map_textures.atlas.clone());
    for (tile_entity, position) in query.iter() {
        let index = map.index_from_float(position.0.x, position.0.y);
        let atlas_index = match map.tiles.get(index).unwrap() {
            TileType::Floor => 0,
            TileType::Wall => 3,
        };
        texture_atlas.index = atlas_index;
        commands.entity(tile_entity).insert((
            Sprite::from_atlas_image(map_textures.spritesheet.clone(), texture_atlas.clone()),
            Transform::from_xyz(position.0.x * TILESIZE, position.0.y * TILESIZE, 0.),
        ));
    }
}
