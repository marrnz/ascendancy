use crate::client::player::render::assets::PlayerAssets;
use crate::shared::{Player, Position, TILESIZE};
use bevy::prelude::{Added, Commands, Entity, Query, Res, Transform, With, info};
use bevy::sprite::{Sprite, TextureAtlas};

pub fn spawn_player_graphics(
    player_textures: Res<PlayerAssets>,
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Player>>,
) {
    let mut texture_atlas = TextureAtlas::from(player_textures.atlas.clone());
    texture_atlas.index = 5;
    for (player_entity, player_position) in query.iter() {
        commands.entity(player_entity).insert((
            Sprite::from_atlas_image(player_textures.spritesheet.clone(), texture_atlas.clone()),
            Transform::from_xyz(
                player_position.0.x * TILESIZE,
                player_position.0.y * TILESIZE,
                1.,
            ),
        ));
    }
}

pub fn update_player_position(mut query: Query<(&mut Transform, &Position), With<Player>>) {
    let (mut player_transform, player_position) = query
        .get_single_mut()
        .expect("There should be exactly one player!");

    player_transform.translation.x = player_position.0.x;
    player_transform.translation.y = player_position.0.y;
}
