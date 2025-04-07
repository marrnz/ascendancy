use crate::shared::Player;
use bevy::prelude::{Camera, Camera2d, Commands, Query, Transform, With, Without};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    for player_transform in player_query.iter() {
        let mut camera_transform = camera_query.single_mut();

        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}
