use ascendancy_shared::Player;
use bevy::prelude::{Camera, Camera2d, Commands, Entity, ParamSet, Query, Transform, TransformHelper, Vec3, With, Without};

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

/*
pub(crate) fn camera_follow(
    player: Query<Entity, With<Player>>,
    mut transform_params: ParamSet<(TransformHelper, Query<&mut Transform, With<Camera>>)>,
) {
    if let Some(player) = player.iter().next() {
        // compute its actual current GlobalTransform
        // (could be Err if entity doesn't have transforms)
        let Ok(global) = transform_params.p0().compute_global_transform(player) else {
            return;
        };
        // get camera transform and make it look at the global translation
        transform_params
            .p1()
            .single_mut()
            .look_at(global.translation(), Vec3::Y);
    }
}
*/