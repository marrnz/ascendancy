use crate::GameState;
use bevy::app::{App, Plugin};
use bevy::prelude::{IntoSystemConfigs, Startup, Update, in_state};
use crate::camera::systems::{camera_follow, spawn_camera};

mod systems;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow.run_if(in_state(GameState::Main)));
    }
}
