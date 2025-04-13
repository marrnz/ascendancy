use ascendancy_shared::Player;
use bevy::prelude::{Camera, Camera2d, Commands, Entity, ParamSet, Query, Transform, TransformHelper, Vec3, With};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub(crate) fn camera_follow(
    player: Query<Entity, With<Player>>,
    mut transform_params: ParamSet<(
        TransformHelper,
        Query<&mut Transform, With<Camera>>,
    )>,
) {
    // get the Entity ID we want to target
    let e_target = player.single();
    // compute its actual current GlobalTransform
    // (could be Err if entity doesn't have transforms)
    let Ok(global) = transform_params.p0().compute_global_transform(e_target) else {
        return;
    };
    // get camera transform and make it look at the global translation
    transform_params.p1().single_mut().look_at(global.translation(), Vec3::Y);
}