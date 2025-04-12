use crate::map::render::assets::MapAssets;
use ascendancy_shared::{Map, Position, TILESIZE, Tile, TileType};
use bevy::prelude::{Added, Commands, Entity, Query, Res, Sprite, TextureAtlas, Transform};

pub fn render_map(
    map_textures: Res<MapAssets>,
    mut commands: Commands,
    query: Query<(Entity, &Tile), Added<Tile>>,
) {
    let mut texture_atlas = TextureAtlas::from(map_textures.atlas.clone());
    for (entity, tile) in query.iter() {
        let atlas_index = match tile.tile_type {
            TileType::Floor => 0,
            TileType::Wall => 3,
        };
        texture_atlas.index = atlas_index;
        commands.entity(entity).insert((
            Sprite::from_atlas_image(map_textures.spritesheet.clone(), texture_atlas.clone()),
            Transform::from_xyz(
                tile.position.0.x * TILESIZE,
                tile.position.0.y * TILESIZE,
                0.,
            ),
        ));
    }
}
