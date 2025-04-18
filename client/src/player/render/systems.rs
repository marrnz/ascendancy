use crate::player::render::assets::PlayerAssets;
use ascendancy_shared::{Opponent, Player, Position, PreviousPosition, TILESIZE};
use bevy::prelude::{Added, Commands, Entity, Fixed, Query, Res, Time, Transform, With, info};
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

pub fn spawn_opponent_graphics(
    player_textures: Res<PlayerAssets>,
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Opponent>>,
) {
    let mut texture_atlas = TextureAtlas::from(player_textures.atlas.clone());
    texture_atlas.index = 6;
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

/*
pub fn update_player_position(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Transform, &Position, &PreviousPosition), With<Player>>,
) {
    let (mut player_transform, player_position, previous_player_position) = query
        .get_single_mut()
        .expect("There should be exactly one player!");

    let overstep_fraction = fixed_time.overstep_fraction();
    let delta = player_position.0 - previous_player_position.0;
    let future_position = player_position.0 + delta;
    let lerp_result = player_position.0.lerp(future_position, overstep_fraction);
    player_transform.translation.x = lerp_result.x;
    player_transform.translation.y = lerp_result.y;
}
*/

pub fn update_player_position(mut query: Query<(&mut Transform, &Position), With<Player>>) {
    let (mut player_transform, player_position) = query
        .get_single_mut()
        .expect("There should be exactly one player!");

    player_transform.translation.x = player_position.0.x;
    player_transform.translation.y = player_position.0.y;
}

pub fn update_opponent_position(mut query: Query<(&mut Transform, &Position), With<Opponent>>) {
    let (mut player_transform, player_position) = query
        .get_single_mut()
        .expect("There should be exactly one player!");

    player_transform.translation.x = player_position.0.x;
    player_transform.translation.y = player_position.0.y;
}
