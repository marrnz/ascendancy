use crate::client::player::render::assets::PlayerAssets;
use crate::shared::{Player, Position, TILESIZE};
use bevy::prelude::{info, Added, Commands, Entity, Query, Res, Transform};
use bevy::sprite::{Sprite, TextureAtlas};

pub fn render_player(
    player_textures: Res<PlayerAssets>,
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Player>>,
) {
    let mut texture_atlas = TextureAtlas::from(player_textures.atlas.clone());
    texture_atlas.index = 5;
    for (player_entity, player_position) in query.iter() {
        info!("Rendering player!");
        commands.entity(player_entity).insert((
            Sprite::from_atlas_image(player_textures.spritesheet.clone(), texture_atlas.clone()),
            Transform::from_xyz((player_position.0.x * TILESIZE), player_position.0.y * TILESIZE, 0.),
        ));
    }
}
